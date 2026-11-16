# Implementation conventions: Autoware on a Rust and nano-ros island

|                 |                                                                                                                                                                                                                               |
| :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Status          | **Decided**: the implementation team owns the layout and keeps the plan's names and structure where nano-ros allows; the scaffold pull request implements it                                                                  |
| Design baseline | Release V1.0 (version 20261023), pinned as the submodule [`design/RFC_L4_SafetyIslandDesign`](../../design/RFC_L4_SafetyIslandDesign) at tag `v1.0`                                                                           |
| Inputs          | [ARCHITECTURE.md](../ARCHITECTURE.md), [PLAN-S32Z2.md](../PLAN-S32Z2.md), [WP0-WP3-SafetyFloor.md](../WP0-WP3-SafetyFloor.md), [MILESTONES.md](../MILESTONES.md), the Autoware contribution guidelines, nano-ros 0.6.0-nros11 |

This note records how the repository applies Autoware's conventions to a code
base whose island side is `no_std` Rust and ported C++ on nano-ros, and which
rules it adds. Each section ends in the decision the scaffold implements.

## 1. What the plan fixes

From [ARCHITECTURE.md](../ARCHITECTURE.md) and the plan:

- Layers: shared crates (`si-core`, `si-nodes`, `si-msgs`, `si-sensors`,
  `si-hal`, `si-runtime`), platforms (`s32z2`, `x5h`, `host`) and `config/`
  (branch profiles, deployments). Dependencies point down; `crates/` never
  depends on `platforms/` (rule 1).
- Node logic is a pure step function: no I/O, no RTOS call, no middleware type
  (rule 2). An absent sensor is an absent input, not a code path (rule 3).
- `no_std` and no heap after init in every shared crate (rule 4).
- FreeRTOS + nano-ros on the R52 clusters (rule 5); host builds for tests.
- At least 85 % of lines shared per platform (section 6).
- Decisions Q1 (nano-ros built-in E2E, configured, not written) and Q2 (Rust
  for the new island nodes). Q8 (ported `I*` nodes stay C++) is proposed.
- M0/WP0: message package `autoware_safety_island_msgs` with fixed capacities
  (#16), E2E configuration (#17), node contracts and their checker in CI
  (#18), the test harness (#19). WP1 needs a host build from the same
  partition configuration (#23).

## 2. Language rule

| Package kind                                                    | Language                  | API                                                                                |
| :-------------------------------------------------------------- | :------------------------ | :--------------------------------------------------------------------------------- |
| Ported from Autoware (`I1`-`I3`, `I5`-`I13`, `I19`, `I20`, ...) | C++, as Autoware wrote it | nano-ros's rclcpp-compatible C++ API (typed component, `configure(rclcpp::Node&)`) |
| New island node (`N*`)                                          | Rust, `no_std`            | nano-ros Rust API (`nros::node!`); logic in `crates/si-nodes`                      |
| New HPC node (`H*`), where new                                  | Rust                      | nano-ros on `std`, or rclrs if it must run inside the Autoware process             |
| Shared logic, HAL, sensors                                      | Rust, `no_std`            | plain Rust, no middleware                                                          |

A port keeps its language so that it stays reviewable against the Autoware
package it came from, and Autoware's own tests remain an equivalence check.
This states Q8 as a rule. `I16` (vehicle interface) is new code for the golf
cart and therefore Rust.

**Verified (nano-ros 0.6.0-nros11).** One image holds C++ and Rust components:
when an image has a C or C++ package, nano-ros builds it with CMake and the
Rust packages build inside it through Corrosion. The scaffold's images hold a
Rust placeholder (`autoware_si_placeholder_rust`) and a C++ placeholder
(`autoware_si_placeholder_cpp`). All four images build, and on the host
images the C++ node receives the Rust node's heartbeat at the period the
branch's parameter file sets. What the packages and the build rely on:

- a Rust package is a plain cargo package (`nros_cargo`) in a mixed image
  too; nano-ros registers it from its `[[component]]` row in `system.toml`;
- a C++ class whose header is not `<pkg>/<Class>.hpp` names it with
  `HEADER` in `nano_ros_add_node`, which keeps Autoware's
  `include/autoware/<name>/` layout;
- a C++ component that prints makes `stdout` unbuffered in `configure`, so
  its lines reach the console `nros run` records;
- a cross image (`host_freertos`, `s32z2`) is checked against a census of
  the native image: `nros configure --census <bringup>:<image>` takes it, and
  needs `ninja`;
- the post-link stack-floor check of the FreeRTOS images reads the LLVM tools
  of the pinned Rust toolchain (`llvm-tools` in `rust-toolchain.toml`);
- a Rust node reads a launch value, including one from a `<param from>` file,
  with `ctx.param()`.

## 3. Layout

ARCHITECTURE.md's layout holds Rust crates only. Ported C++ packages, message
packages and deployments need ROS-shaped packages. nano-ros 0.6.0-nros8 found
packages only one level below `src/`, hence the flat `src/`; 0.6.0-nros10 and later also
searches nested directories and `nros.toml` `package_paths`, so a nested
layout is possible if the team wants one. Proposed layout:

```text
.
|-- crates/                         shared no_std Rust libraries, no ROS, no middleware
|   |-- si-core/  si-hal/  si-nodes/  si-sensors/
|-- platforms/                      per-platform Rust crates implementing si-hal
|   |-- host/  s32z2/               (x5h/ when its work starts)
|-- src/                            ROS 2 / nano-ros packages, one package.xml each
|   |-- autoware_safety_island_msgs/     IDL: the source of every message type
|   |-- autoware_si_<node>/              a new N* node (Rust): binds si-nodes logic to endpoints
|   |-- autoware_<ported_package>/       a ported I* node (C++), Autoware's package name
|   `-- autoware_safety_island_bringup/  deployments: system.toml, launch, contract, params
|-- design/RFC_L4_SafetyIslandDesign/    the design baseline (submodule, tag v1.0)
|-- ci/                             self-hosted integration lane (runner, suite)
`-- docs/
```

How the plan's layers map:

| ARCHITECTURE.md                                      | Here                                       | Why                                                                                                                                                         |
| :--------------------------------------------------- | :----------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `crates/si-core`, `si-nodes`, `si-sensors`, `si-hal` | same                                       | Pure Rust; `cargo test` on the host with no ROS and no nano-ros                                                                                             |
| `crates/si-msgs`                                     | `src/autoware_safety_island_msgs` (`.msg`) | One source per type (section 5): the Rust and C++ types are generated, so no hand-written crate                                                             |
| `crates/si-runtime`                                  | the node packages in `src/` + the bringup  | nano-ros generates the executors and entry from `system.toml`; the per-node shells are node packages                                                        |
| `platforms/{s32z2,x5h,host}`                         | same                                       | FreeRTOS, lwIP and the transport come from the nano-ros board crate the image row names (`s32z270-freertos`); the platform crate keeps what is the island's |
| `config/`                                            | `src/autoware_safety_island_bringup`       | nano-ros requires `system.toml` to live in a bringup package under `src/`; branch profiles are its `config/<branch>/` parameter files                       |

Rules 1-4 hold unchanged: `crates/` depends on nothing in `src/` or
`platforms/`; a node package depends on `crates/` and the generated message
crate only.

## 4. Autoware conventions and how they map

| Autoware convention                                                                                    | Here                                                                                                                                                                                         |
| :----------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Package per directory, folder name = package name, `autoware_` prefix, no `_node` suffix               | Every package under `src/`. Ported packages keep Autoware's name (`autoware_control_command_gate`); new island nodes are `autoware_si_<rfc_name>` (for example `autoware_si_hpc_supervisor`) |
| `package.xml` per package, maintainers drive review                                                    | Every package under `src/`. `build_type` is `nros_cmake` (C++, and Rust in a mixed image) or `ament_cmake` (message packages, also built by colcon on the HPC)                               |
| Rust crates                                                                                            | Not ROS packages: `crates/` and `platforms/` follow Rust naming (`si-hal`, lib `si_hal`) and have no `package.xml`                                                                           |
| C++ directory structure (`include/autoware/<pkg>/`, `src/`, `test/`, `config/`, `schema/`, `launch/`)  | Ported C++ packages keep it                                                                                                                                                                  |
| Parameters: `config/<node>.param.yaml` + `schema/<node>.schema.json`, validated by `json-schema-check` | Same for every node package; the bringup's `config/<branch>/<node>.param.yaml` overrides are validated against the node schema too                                                           |
| Message guidelines (`.msg` in an interface package)                                                    | `src/autoware_safety_island_msgs`; every string and sequence bounded (`string<=N`, `T[<=N]`)                                                                                                 |
| Launch files (`launch/*.launch.xml`)                                                                   | The bringup's `launch/system.launch.xml`; the branch is a launch argument                                                                                                                    |
| `colcon build`                                                                                         | `nros build <image>` for the island images (it drives CMake, cargo and Corrosion); `cargo` for `crates/` and `platforms/`; `colcon` only for the message packages on the HPC                 |
| pre-commit (sync-file-templates hooks)                                                                 | The hooks that apply, plus `rustfmt`; the planning files are excluded                                                                                                                        |
| clang-format, cpplint (C++)                                                                            | Kept for the ported C++ packages, with Autoware's `.clang-format` and `CPPLINT.cfg`                                                                                                          |
| clang-tidy, cppcheck                                                                                   | Not yet: added with the first ported package. For Rust: `clippy -D warnings` (pedantic), `cargo doc -D warnings`, `cargo deny`                                                               |
| spell-check-differential (cspell)                                                                      | Same action and dictionaries                                                                                                                                                                 |
| semantic-pull-request                                                                                  | Autoware's reusable workflow, called from the gate                                                                                                                                           |
| DCO (GitHub App)                                                                                       | A workflow (`.github/scripts/check_dco.py`), because the gate must report it in one required check                                                                                           |
| build-and-test(-differential)                                                                          | `build-and-test`: Rust on the host and for `armv8r-none-eabihf`, and the nano-ros images                                                                                                     |
| PR template, Conventional Commits, small PRs, AI contribution policy                                   | Same; see [CONTRIBUTING.md](../../CONTRIBUTING.md)                                                                                                                                           |

## 5. Sources of truth

Each fact has one source. Everything else is generated from it or checked
against it, and a gate job fails when they disagree.

| Fact                                                      | Source                                                                                          | Generated or checked                                                                        | Gate job                                                   |
| :-------------------------------------------------------- | :---------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------ | :--------------------------------------------------------- |
| Message types and their capacities                        | `src/autoware_safety_island_msgs/msg/*.msg`                                                     | Rust and C++ types (nano-ros codegen); the ROS 2 package on the HPC (colcon)                | `build-and-test` (images), `contract-check` (colcon build) |
| Node parameters: names, types, ranges, defaults           | `src/<pkg>/schema/<node>.schema.json` + `config/<node>.param.yaml`                              | Bringup overrides checked against the schema                                                | `json-schema-check`, `contract-check`                      |
| Branch profile (Least-Cost / Maximum-Safety)              | `src/autoware_safety_island_bringup/config/<branch>/`, selected by the launch argument `branch` | The image's baked parameters                                                                | `contract-check`, `build-and-test`                         |
| What runs where: images, boards, RMW, node set            | `src/autoware_safety_island_bringup/system.toml` + `launch/system.launch.xml`                   | The image entry, executors and pools (`nros build`)                                         | `build-and-test`                                           |
| ROS 2 edition and RMW                                     | `[system] ros_edition` and `[system] rmw` in `src/autoware_safety_island_bringup/system.toml`   | The CI container, `/opt/ros/<edition>`, the zenoh router (`.github/scripts/read_system.py`) | `build-and-test`, `contract-check`                         |
| Rates, budgets, FTTIs, priorities, monitors, independence | `src/autoware_safety_island_bringup/launch/system.contract.yaml` (rlm grammar)                  | Checked by `play_launch check --locked --strict`; nano-ros sizes the image from it          | `contract-check`, `build-and-test`                         |
| Toolchains                                                | `rust-toolchain.toml`, `nros-toolchain.toml`                                                    | Installed by rustup and `nros`                                                              | every job                                                  |
| Design text                                               | `design/RFC_L4_SafetyIslandDesign` at `v1.0`                                                    | Cited by section number                                                                     | review                                                     |

The placeholder message, parameter schema and contract in the scaffold show
each pattern; WP0 (#16, #18) replaces them with the real sets. The earlier
contract drafts of the island and HPC sides (rlm grammar, play_launch 0.34.0)
are the starting point for #18.

## 6. Merge flow and gates

- Every change goes through a pull request; `main` is protected by
  [`.github/rulesets/main.json`](../../.github/rulesets/main.json): one
  approving review, linear history, no force push or deletion, merge queue
  (rebase, all-green grouping, up to 5 entries), and one required status
  check, `gate`.
- `gate` ([`.github/workflows/gate.yaml`](../../.github/workflows/gate.yaml))
  runs on `pull_request`, `merge_group` and pushes to `main`, with no path
  filter, and needs every check job. Pull-request-only checks (title, DCO,
  spelling) are skipped in the merge group and accepted as skipped there only.
- GitHub-hosted runners carry the fast lane. The `integration` workflow runs
  on a self-hosted `l4si-integration` runner, manual and nightly (and in the
  merge queue once enabled), and is skipped where no such runner is
  registered.
- The progress workflow commits the README counts straight to `main` with
  the workflow token. The ruleset blocks that push, and GitHub does not accept
  GitHub Actions as a bypass actor (tested on the fork). The workflow needs a
  token that is on the bypass list, or must open a pull request instead
  (open question C7).

Applying the ruleset needs an administrator; see
[ci/repository-setup.md](../ci/repository-setup.md).

## 7. Prior work considered

- The Arm `autoware-safety-island` repository, `nano-ros` branch of the
  NEWSLabNTU fork (open point P4): already runs `I9`-`I12` (C++) with FreeRTOS
  targets for the S32Z2 and the X5H, the latter needing NXP-licensed inputs.
  **Decided:** `I9`-`I12` are imported from it as C++ packages under `src/`
  (#2); the repository does not start from it. See [asi-reuse.md](asi-reuse.md).
- nano-ros board crates: `s32z270-freertos` (Cortex-R52, build-only, licensed
  GIC port and NETC driver consumer-side) and `mps3-an536-freertos` (QEMU
  Cortex-R52, boots and schedules). The latter is the candidate for a
  hardware-free R52 run lane.

## 8. Open questions and decisions

| #   | Question                                                                                                                                                                       | Status                                                                                                                                                                                                                                           |
| :-- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| C2  | Repository license                                                                                                                                                             | **Decided**: Apache License 2.0, as Autoware Foundation work. `LICENSE` and `NOTICE` at the root; sources carry the header `Copyright <year> The Autoware Foundation` with the Apache boilerplate                                                |
| C3  | RMW of the images (the HPC boundary of WP1 is Cyclone DDS)                                                                                                                     | **Decided**: zenoh is the default and the RMW is configuration. `[system] rmw` in the bringup's `system.toml` is the one switch (`zenoh`, `xrce` or `cyclonedds`); CI reads it and installs the zenoh router only for `zenoh`. #22 may switch it |
| C4  | ROS 2 edition for codegen and the HPC                                                                                                                                          | **Decided**: Humble is the default and the edition is configuration. `[system] ros_edition` in `system.toml` is the one switch (`humble` or `jazzy`); CI derives its container (`ros:<edition>-ros-base`) and `/opt/ros/<edition>` from it       |
| C5  | The X5H platform crate and its RTOS (P1)                                                                                                                                       | Open, plan owner                                                                                                                                                                                                                                 |
| C6  | AI trailers in commits                                                                                                                                                         | **Decided**: no AI `Co-authored-by` or session trailers, as Autoware's AI contribution policy asks; the human author signs off (`git commit -s`) and discloses AI use in the pull request                                                        |
| C7  | How the progress workflow updates `README.md` once `main` is protected: a GitHub App token on the ruleset's bypass list (smallest change), or a pull request through the queue | Open, plan owner                                                                                                                                                                                                                                 |
