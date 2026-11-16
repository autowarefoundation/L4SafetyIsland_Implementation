# ci

Reserved for the self-hosted integration lane: the runner container and the
utilities that run Autoware (HPC side), the safety island and the
[scenario world bridge](https://github.com/NEWSLabNTU/scenario-world-bridge)
together.

The workflow is [`.github/workflows/integration.yaml`](../.github/workflows/integration.yaml).
It runs `ci/integration/run.sh` when that script exists, and is skipped on a
repository without a registered `l4si-integration` runner. See
[docs/testing.md](../docs/testing.md) and
[docs/ci/repository-setup.md](../docs/ci/repository-setup.md).
