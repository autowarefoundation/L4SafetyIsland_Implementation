# Testing

The test levels follow [PLAN-S32Z2.md](PLAN-S32Z2.md) section 4. This page
says where each level lives and which CI lane runs it.

## Levels

| Level                    | What                                                                                        | Where                                                               | Lane                                           |
| :----------------------- | :------------------------------------------------------------------------------------------ | :------------------------------------------------------------------ | :--------------------------------------------- |
| Unit (T1)                | Node logic as pure step functions; HAL types                                                | `#[cfg(test)]` in `crates/*`, `platforms/host/tests/`               | `build-and-test` (hosted)                      |
| Static                   | Message bounds, parameter files against schemas, contract rates and budgets                 | `src/*/msg`, `src/*/schema`, `src/*_bringup/launch/*.contract.yaml` | `json-schema-check`, `contract-check` (hosted) |
| Image build              | Every image of the bringup builds and links, host and S32Z2                                 | `src/autoware_safety_island_bringup/system.toml`                    | `build-and-test` (hosted)                      |
| Host integration (T2/T3) | Images run as Linux processes against scripted publishers; golden traces (WP0 harness, #19) | to come with #19                                                    | `build-and-test`                               |
| System integration       | Autoware on the HPC side + the island + the scenario world bridge                           | `ci/`                                                               | `integration` (self-hosted)                    |
| HIL (T4)                 | S32Z280-400EVB + HPC PC + CAN FD vehicle simulator (#23)                                    | `ci/`                                                               | `integration` (self-hosted, board attached)    |

## Running the fast lane locally

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy --workspace --exclude si-platform-host --target armv8r-none-eabihf -- -D warnings
pre-commit run --all-files
scripts/check_contracts.sh            # after building the message overlay, see getting-started.md
python3 scripts/check_param_overrides.py
nros build --all
```

## Writing tests

- Test node logic through `si_nodes::StepNode::step` with explicit inputs and
  times. The node has no I/O, so a unit test needs no middleware, no clock and
  no RTOS.
- A test for a fix must fail without the fix.
- A ported C++ package keeps Autoware's tests (`test/`) and runs them on the
  host image.
- Timing claims come from contracts (stated) or from the target trace (#23,
  measured). A host run measures no timing.

## CI lanes

| Lane                           | Runner                                | Triggers                                  | Required     |
| :----------------------------- | :------------------------------------ | :---------------------------------------- | :----------- |
| `gate` and the checks it calls | GitHub-hosted                         | pull request, merge queue, push to `main` | yes (`gate`) |
| `integration`                  | self-hosted, label `l4si-integration` | manual, nightly, merge queue when enabled | not yet      |

`integration` is skipped on any repository where the variable
`L4SI_INTEGRATION_RUNNER` is not `true`, so forks never wait for a runner they
do not have. [ci/repository-setup.md](ci/repository-setup.md) says how to
register the runner and when to make the lane required.
