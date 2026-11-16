<!-- cspell:ignore killall pipefail shellcheck -->

# ci/: integration runner and scripts

Rules for working under `ci/` (the top-level CLAUDE.md covers the rest of the repository).
Overview: `docs/testing/integration.md`; runner admin: `ci/runner/README.md`.

## Layout

- `ci/runner/`: `Dockerfile` (targets `stack` and `runner`), `compose.yaml` (`carla`,
  `runner`, `local`), `register.sh`, `entrypoint.sh`, `host.yaml` (swb host settings),
  `env.sh` (step environment).
- `ci/scripts/`: `integration.sh` = `stack-up.sh` + `run-scenarios.sh` +
  `collect-artifacts.sh` + `stack-down.sh`; `island-up.sh`/`island-down.sh` the island hook.
- `ci/project/`: the scenario-world-bridge (swb) project: `swb.yaml`, `scenarios/`,
  `suites/`.

## Rules

- **One code path.** CI and local runs use the same `ci/scripts/*.sh`; the workflow only
  calls `ci/scripts/integration.sh`. Do not put test logic in the workflow YAML.
- **Exit codes are swb's**: 0 passed, 1 failed, 2 error (stack could not judge), 3
  refused. Keep every script on that contract.
- **Settings are environment variables** (`L4SI_*`, `CARLA_HOST`, `CARLA_PORT`), listed in
  `ci/scripts/lib.sh`. Scenario and project choices live in `ci/project/`.
- **Stop only what you started**: process groups by pid file (`stop_group` in `lib.sh`).
  Never `pkill`/`killall`, never stop CARLA from a job.
- **No secrets in the repo or the images.** The runner token stays on the runner host
  (`register.sh` reads it from a mode-600 file); containers only get single-use JIT
  configs.
- **No privileged containers.** No `--privileged`, host network or Docker socket; the stack
  containers get `SYS_NICE` only, CARLA gets the GPU through the NVIDIA device request.
- **Pin versions** in the Dockerfile `ARG`s (Autoware, swb commit, play_launch, CARLA,
  nros, Rust, runner + checksum) and keep `docs/testing/integration.md` in step.
- `RUST_TOOLCHAIN` in the Dockerfile follows the root `rust-toolchain.toml`.
- Shell: bash, `set -euo pipefail` (or `-uo` where the script must reach cleanup); pass
  shellcheck. Text is keyboard ASCII.
