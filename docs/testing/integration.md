<!-- cspell:ignore carlasim rosbag rosdep -->

# Integration tests

Closed-loop tests of Autoware with the safety island: Autoware 2.0.0 drives a simulated
car in CARLA 0.9.16 through OpenSCENARIO scenarios, and
[scenario_simulator_v2](https://github.com/tier4/scenario_simulator_v2) (SSv2) gives
each run its verdict. [scenario-world-bridge](https://github.com/NEWSLabNTU/scenario-world-bridge)
(swb) connects the three. The island joins as one more ECU on the ego's ROS domain:
first the host build of this repository, later QEMU, then the S32Z2 board.

## Architecture

```text
 runner host (GPU)
 +---------------------------------------------------------------------------+
 |  carla container (carlasim/carla:0.9.16, GPU, long-lived)                 |
 |     CARLA server :2000 (RPC), :2001.. (sensor streams)                    |
 |          ^                                                                |
 |          | CARLA protocol (private bridge network "l4si")                 |
 |          v                                                                |
 |  runner container (l4si-ci-runner, one job, then removed)                 |
 |   GitHub Actions runner -> ci/scripts/integration.sh                      |
 |     swb stack (play_launch swb_launch swb.launch.xml)                     |
 |       scenario domain: SSv2, swb bridge, agent relay                      |
 |       ego domain:      Autoware 2.0.0, CARLA vehicle bridge, swb agent    |
 |                        safety island (host build)    <- L4SI_WITH_ISLAND  |
 |     swb run suites/<suite>.yaml  ->  passed / failed / error              |
 |   DDS: CycloneDDS on the container's loopback (swb's localhost profile)   |
 +---------------------------------------------------------------------------+
```

- **CARLA** owns the world and the ego's physics. It runs in its own container for the
  life of the host: starting it takes minutes, and swb is built to reuse one.
- **The stack** (swb's `swb.launch.xml`) brings up CARLA's world server, the bridge SSv2
  talks to, the agent relay, and the ego: Autoware with swb's Autoware profile for the
  Tesla Model 3 in `sim_vehicle_spec`, on the simulator's lidar and cameras.
- **A run** (`ros2 run swb run`) starts SSv2 on a scenario against the stack, and exits
  with the verdict: 0 passed, 1 failed, 2 error (the stack could not judge the run),
  3 refused. `ci/scripts/integration.sh` exits with the same code, which is the job's.
- **The island**, with `L4SI_WITH_ISLAND=1`, is built from this checkout
  (`platforms/host`) and started on the ego's ROS domain with swb's DDS profile before
  the stack comes up, so it sees Autoware's topics as a separate ECU would.

The pieces and their versions are in `/etc/l4si/versions` of the image and in every
run's `versions.txt` artifact.

| Path                                 |                                                                                     |
| ------------------------------------ | ----------------------------------------------------------------------------------- |
| `ci/runner/`                         | the images, compose file, runner registration ([README](../../ci/runner/README.md)) |
| `ci/scripts/`                        | the steps; the same in CI and on a workstation                                      |
| `ci/project/`                        | the swb project: `swb.yaml`, `scenarios/`, `suites/`                                |
| `.github/workflows/integration.yaml` | the workflow                                                                        |

### The scripts

| Script                           |                                                                                                                                                            |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `integration.sh [suite]`         | all of it: up, run, collect, down; exits with the verdict                                                                                                  |
| `stack-up.sh`                    | waits for CARLA, stages `ci/project` into the work directory, fetches its assets, starts the island (if asked) and the stack in a process group of its own |
| `run-scenarios.sh [suite]`       | `swb run <suite> --wait`: waits for the stack to be ready, runs the suite                                                                                  |
| `collect-artifacts.sh`           | summary, merged JUnit, run directories, stack and island logs, versions into `$L4SI_WORK/artifacts`                                                        |
| `stack-down.sh`                  | stops the stack's process group and the island; CARLA stays                                                                                                |
| `island-up.sh`, `island-down.sh` | the island hook (`L4SI_ISLAND_TARGET=host`, `qemu`; `hil` reserved)                                                                                        |

Settings are environment variables (`ci/scripts/lib.sh` lists them): `L4SI_WORK`,
`CARLA_HOST`, `CARLA_PORT`, `L4SI_READY_TIMEOUT`, `L4SI_WITH_ISLAND`,
`L4SI_ISLAND_TARGET`, `L4SI_ISLAND_BUILD`, `L4SI_ISLAND_CMD`.

### What a run leaves

`$L4SI_WORK/artifacts/`, uploaded by the workflow as `integration-<run>-<attempt>`:

|                                           |                                                                                                                                                                 |
| ----------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `runs.txt`, `summary.txt`                 | `swb runs list` and `swb runs summary`                                                                                                                          |
| `suites/<suite>/summary.csv`, `junit.xml` | the suite's table and merged JUnit                                                                                                                              |
| `runs/<run id>/`                          | `result.json` (verdict, reason, events), `timeline.jsonl`, `junit.xml`, `logs/`; a failed or broken run also keeps `record/` (a rosbag of the ego's key topics) |
| `stack.log`, `island*.log`, `runs.log`    | the processes the scripts started                                                                                                                               |
| `versions.txt`, `swb.lock`                | what ran, and the map dataset's pinned commit                                                                                                                   |

## Run it locally

Same images, same scripts, no runner and no GitHub token. On a host with Docker, the
NVIDIA container toolkit and a GPU:

```bash
ci/runner/build.sh                                       # once
docker compose -f ci/runner/compose.yaml up -d carla     # CARLA; wait until healthy
docker compose -f ci/runner/compose.yaml run --rm local  # the smoke suite
L4SI_WITH_ISLAND=1 docker compose -f ci/runner/compose.yaml run --rm local
docker compose -f ci/runner/compose.yaml down            # CARLA too
```

`local` mounts this checkout at `/work` and leaves its artifacts in
`build/integration/artifacts/`. Another suite:
`docker compose -f ci/runner/compose.yaml run --rm local bash ci/scripts/integration.sh suites/<name>.yaml`.

Without containers, on a host with Autoware 2.0.0, swb built against it, play_launch
and CARLA serving: point `ci/runner/env.sh` at your installs and run the scripts
directly.

```bash
export L4SI_AUTOWARE_SETUP=/opt/autoware/setup.bash
export L4SI_SWB_SETUP=~/scenario-world-bridge/install/setup.bash
CARLA_HOST=localhost ci/scripts/integration.sh suites/smoke.yaml
echo $?
```

The stack's web UI (every process and its log) is on `L4SI_WEB_ADDR`, default
`127.0.0.1:8080` inside the container.

## Add a scenario

1. Write the `.xosc` into `ci/project/scenarios/`. Name the map as
   `$(env SWB_MAPS)/<map>` and the ego as the catalog car `tesla_model3` with the
   controller `Autoware` (copy `drive_to_goal.xosc`); swb's
   [Writing scenarios](https://github.com/NEWSLabNTU/scenario-world-bridge/blob/main/docs/design/scenario-authoring.md)
   lists what the bridge supports.
2. A new map: add it to `take:` in `ci/project/swb.yaml` (datasets: swb's
   [Maps and assets](https://github.com/NEWSLabNTU/scenario-world-bridge/blob/main/docs/reference/maps-and-assets.md)),
   run `ros2 run swb assets fetch` in the project once and commit the new `swb.lock`.
3. Put it in a suite under `ci/project/suites/`: `smoke.yaml` (every nightly, no island),
   `island.yaml` (with the island), or a new one. A suite takes files or directories,
   a parameter grid, repetitions and `expect:` for a scenario that is meant to fail
   ([swb's suites](https://github.com/NEWSLabNTU/scenario-world-bridge/blob/main/docs/tutorials/05-suites.md)).
4. Check it: `ros2 run swb project check` in the staged project parses every scenario and
   suite; then run the suite locally as above.
5. A new suite in CI: the workflow's `suite` input (dispatch), or change the default in
   `.github/workflows/integration.yaml`.

## Known gaps

- **The ego does not start on Autoware 2.0.0 yet.** swb's Autoware profile
  (`autoware_sim_profile` in swb's `autoware_sim_vehicle`) is written against Autoware
  1.5.0 and includes the `tier4_{localization,perception,planning,control}_launch`
  packages, which 2.0.0 renamed to `autoware_*_launch`. swb itself builds against 2.0.0
  and the stack's simulator side comes up, but resolving the ego's launch stops at
  `package 'tier4_perception_launch' not found`, so a run ends as `error` (exit 2) until
  the profile is ported to 2.0.0 in swb. The image skips the one such key rosdep sees
  (`tier4_localization_launch`, Dockerfile `ROSDEP_SKIP_KEYS`); drop it with the port.
- The island's host binary does not exist yet: with `L4SI_WITH_ISLAND=1` the run is
  refused (exit 3) until `platforms/host` builds one.
- The board (`l4si-hil`) is reserved, not wired.
