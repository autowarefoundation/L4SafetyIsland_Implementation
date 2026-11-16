# ci

The self-hosted integration lane: the runner container and the utilities that
run Autoware (HPC side), the safety island and the
[scenario world bridge](https://github.com/NEWSLabNTU/scenario-world-bridge)
together.

| Path                          |                                                                                              |
| ----------------------------- | -------------------------------------------------------------------------------------------- |
| [`runner/`](runner/README.md) | the CI images, the compose file (CARLA + runner), runner registration                        |
| `scripts/`                    | stack up, run a suite, collect artifacts, tear down (`scripts/integration.sh` does all four) |
| `project/`                    | the scenario world bridge project: scenarios and suites                                      |

The workflow is [`.github/workflows/integration.yaml`](../.github/workflows/integration.yaml).
It runs `ci/scripts/integration.sh` and is skipped on a repository without a
registered `l4si-integration` runner. See
[docs/testing/integration.md](../docs/testing/integration.md),
[docs/testing.md](../docs/testing.md) and
[docs/ci/repository-setup.md](../docs/ci/repository-setup.md).
