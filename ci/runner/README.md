<!-- cspell:ignore carlasim cuda ipfrag jitconfig rmem -->

# Integration runner

The self-hosted GitHub Actions runner for the integration tests: Autoware 2.0.0 + the
safety island + [scenario-world-bridge](https://github.com/NEWSLabNTU/scenario-world-bridge)
(swb) on CARLA 0.9.16. What the tests do and how to run them without a runner:
[docs/testing/integration.md](../../docs/testing/integration.md).

| File                  |                                                                                               |
| --------------------- | --------------------------------------------------------------------------------------------- |
| `Dockerfile`          | the images: `stack` (Autoware + swb + toolchains) and `runner` (`stack` + the Actions runner) |
| `build.sh`            | builds both images                                                                            |
| `compose.yaml`        | the host's services: `carla`, `runner`, `local`                                               |
| `register.sh`         | registration and the runner loop (admin, on the runner host)                                  |
| `entrypoint.sh`       | the runner container's command                                                                |
| `host.yaml`, `env.sh` | swb's host settings and the step environment, baked into the images                           |

## Images

One Dockerfile, two images built from it:

| Image            | Base                                                                  | Adds                                                                                                                                                                                  | Size    |
| ---------------- | --------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------- |
| `l4si-ci-stack`  | `ghcr.io/autowarefoundation/autoware:universe-humble-2.0.0` (runtime) | swb (built in `universe-devel-humble-2.0.0`, copied to `/opt/swb-ws/install`), its runtime deps, play_launch, the CARLA client, Rust (the repository's channel), just, the `nros` CLI | 9.5 GB  |
| `l4si-ci-runner` | `l4si-ci-stack`                                                       | the Actions runner (pinned version and checksum), its entrypoint; no sudo                                                                                                             | 10.3 GB |

Why a base and a layer rather than one image: the `stack` image is everything a run
needs and holds no runner and no credentials, so a developer runs the same scripts in it
on a workstation (`compose run local`), and a runner version bump rebuilds one thin layer.
Why not more layers: Autoware, swb and the toolchain change together (swb is built
against that Autoware), so splitting them buys nothing.

Not in the images: CARLA (its own container, `carlasim/carla:0.9.16`, long-lived, the
only one with a GPU), and no CUDA/TensorRT: the integration profile uses swb's default
lidar detector (`clustering`) and the simulator's traffic lights, which need neither.

```bash
ci/runner/build.sh                 # l4si-ci-stack:dev and l4si-ci-runner:dev
L4SI_BUILD_ARGS="--build-arg SWB_REF=<commit>" ci/runner/build.sh nightly
```

Build arguments: `AUTOWARE_VERSION` (2.0.0), `SWB_REF` (a scenario-world-bridge commit),
`PLAY_LAUNCH_VERSION`, `CARLA_VERSION`, `NROS_VERSION`, `RUST_TOOLCHAIN`,
`RUNNER_VERSION` + `RUNNER_SHA256`. `/etc/l4si/versions` in the image lists what was
built; every run uploads it.

Keep Docker's data root on a fast disk with room: the devel base image is large and the
swb build keeps its `build/` tree in a BuildKit cache (`id=l4si-swb-build`), so a rebuild
after an swb bump is incremental. A cold build takes about 30 minutes, most of it
scenario_simulator_v2.

## The host

- Linux x86_64, Docker with the compose plugin and the NVIDIA container toolkit
  (`nvidia` runtime). An NVIDIA GPU with about 4 GB free for CARLA on Town01.
- No `--privileged`, no host network, no Docker socket in any container. The `carla`,
  `runner` and `local` services share a private bridge network; swb's DDS runs on each
  container's own loopback (its localhost profile), so two hosts' stacks, or the host's
  own ROS processes, never see each other.
- DDS socket buffers: point clouds need the kernel's `net.core.rmem_max` raised, which a
  container cannot do. Once per host (needs root):

  ```bash
  sudo tee /etc/sysctl.d/99-l4si-dds.conf <<'EOF'
  net.core.rmem_max=2147483647
  net.ipv4.ipfrag_time=3
  net.ipv4.ipfrag_high_thresh=134217728
  EOF
  sudo sysctl --system
  ```

## Register the runner

The runner is **ephemeral**: one container takes one job and exits, and the next job
gets a fresh container from the image. Nothing a job writes (checkout, build, logs,
credentials) reaches the next one, except the map cache volume `l4si-cache`.

### 1. A token, kept on the host

An admin of the repository (or organization) makes a fine-grained personal access token:

| Scope               | Permission                                              |
| ------------------- | ------------------------------------------------------- |
| repository runner   | the repository, "Administration: Read and write"        |
| organization runner | the organization, "Self-hosted runners: Read and write" |

Store it on the runner host only, readable by the account that runs the loop:

```bash
install -d -m 700 ~/.config/l4si-runner
( umask 077; cat > ~/.config/l4si-runner/token )   # paste, then Ctrl-D
```

Never commit it, never pass it to a container, never put it in a workflow or a secret
the workflow can read. `register.sh` gives it to `curl` on stdin; only single-use
just-in-time configs leave the host's shell.

### 2. Build, start CARLA, run the loop

```bash
ci/runner/build.sh
export L4SI_RUNNER_REPO=autowarefoundation/L4SafetyIsland_Implementation
# or: export L4SI_RUNNER_ORG=<org>   (L4SI_RUNNER_GROUP_ID=<id> for a runner group)
ci/runner/register.sh loop
```

`loop` starts the `carla` service, then forever: asks GitHub for a just-in-time runner
config (`generate-jitconfig`: name `l4si-<host>-<time>`, labels
`self-hosted,linux,x64,l4si-integration`), runs `docker compose run --rm runner` with it,
and waits for that one job to finish. Run it under a service manager (a systemd user
unit, tmux) on the host. Stop it with Ctrl-C; a runner that was idle unregisters itself
when its config expires.

Without the loop, a single registration:

```bash
docker compose -f ci/runner/compose.yaml up -d --wait carla
ACTIONS_RUNNER_JITCONFIG="$(ci/runner/register.sh jit)" \
    docker compose -f ci/runner/compose.yaml run --rm runner
```

or the classic flow with a one-hour registration token (`--ephemeral` is still set by the
entrypoint):

```bash
RUNNER_URL=https://github.com/<owner>/<repo> RUNNER_TOKEN="$(ci/runner/register.sh token)" \
    docker compose -f ci/runner/compose.yaml run --rm -e RUNNER_URL -e RUNNER_TOKEN runner
```

### 3. Turn the workflow on

In the repository's settings, Actions variables:

| Variable                               |                                                                                                                     |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `L4SI_INTEGRATION_RUNNER=true`         | the `integration` job runs (without it, it is skipped: forks and repositories without the runner never queue a job) |
| `L4SI_INTEGRATION_IN_MERGE_QUEUE=true` | also on the merge queue (`merge_group`)                                                                             |
| `L4SI_HIL_RUNNER=true`                 | the reserved `hil` job, once a board runner labelled `l4si-hil` exists                                              |

Labels: `run-integration` on a pull request runs the job for that PR (branches of this
repository only).

### Upkeep

```bash
ci/runner/register.sh list     # the scope's self-hosted runners
ci/runner/register.sh prune    # remove offline ones named l4si-*
docker compose -f ci/runner/compose.yaml restart carla   # a CARLA that stopped answering
docker volume rm l4si-integration_l4si-cache             # drop the cached maps
```

Security notes: a job runs code from this repository with the runner's network access
and the GPU host's CARLA, so the workflow runs only for same-repository branches, the
schedule, dispatch and the merge queue, never for a fork's pull request. The runner user
has no sudo in the image; the container has `SYS_NICE` and no other added capability.

## Hardware in the loop (later)

The S32Z2 board will sit on its own runner labelled `l4si-hil` (the reserved `hil` job in
`.github/workflows/integration.yaml`, and `L4SI_ISLAND_TARGET=hil` in
`ci/scripts/island-up.sh`). Not wired yet.
