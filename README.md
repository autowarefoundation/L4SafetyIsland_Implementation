# L4 Safety Island for Autoware — Implementation and Demo

Implementation and demonstration of the L4 Safety Island software design for Autoware, built on
[nano-ros](https://github.com/NEWSLabNTU/nano-ros).

## Design baseline

| Item | Value |
| :-- | :-- |
| Design document | *L4 Safety Island for Autoware — Software Design*, Part 1 |
| Design version | **Release V1.0** (version 20261023) |
| Source | [`autowarefoundation/RFC_L4_SafetyIslandDesign`](https://github.com/autowarefoundation/RFC_L4_SafetyIslandDesign), tag [`v1.0`](https://github.com/autowarefoundation/RFC_L4_SafetyIslandDesign/releases/tag/v1.0) |

Node identifiers used in this repository (`I*` migrated Autoware nodes, `N*` new island nodes,
`H*` new HPC nodes, behaviours `B1`–`B6`) follow the design document.

## Status

**Importing the Autoware nodes onto the S32Z2 island** (S32Z280-400EVB, FreeRTOS + nano-ros). The
plan is in [docs/](docs/); the work is tracked in GitHub issues (below).

## Quick start

On Ubuntu 22.04 with ROS 2 Humble, Rust (rustup) and nano-ros installed as
[docs/getting-started.md](docs/getting-started.md) shows:

```bash
git clone --recurse-submodules https://github.com/autowarefoundation/L4SafetyIsland_Implementation.git
cd L4SafetyIsland_Implementation
cargo test --workspace                       # shared Rust crates
source /opt/ros/humble/setup.bash
nros build --all                             # images: host, host_maximum_safety, host_freertos, s32z2
nros run autoware_safety_island_bringup:host
```

| Path | Content |
| :-- | :-- |
| `crates/` | Shared `no_std` Rust: `si-core`, `si-hal`, `si-nodes` (node logic as pure step functions), `si-sensors` |
| `platforms/` | Per-platform Rust: `host`, `s32z2` |
| `src/` | nano-ros packages: messages (`autoware_safety_island_msgs`), node packages (new nodes in Rust, ported Autoware nodes in C++), the bringup with images, launch, contract and branch parameters |
| `design/` | The design baseline, submodule at tag `v1.0` |
| `ci/` | Self-hosted integration lane |

Contributing: [CONTRIBUTING.md](CONTRIBUTING.md) (pull requests, merge queue,
DCO, gates). Conventions: [docs/design/implementation-conventions.md](docs/design/implementation-conventions.md).
Testing: [docs/testing.md](docs/testing.md).

## Progress

GitHub keeps the live state: an open issue is work still to do, a closed one is done. Each
milestone has a tracking issue whose sub-issues show its progress
([docs/MILESTONES.md](docs/MILESTONES.md)); #1 tracks the import of the Autoware nodes. See the
[milestones](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/milestones) and the
[open issues](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues).

### Imported Autoware nodes (S32Z2)

In dependency order (see [PLAN-S32Z2.md](docs/PLAN-S32Z2.md) §2.2). Each issue shows whether it is
open or closed, and its task checklist shows how far it has got.

| Node | Package | WP | Partition | Issue |
| :-- | :-- | :-- | :-- | :-- |
| `I9–I12` | Trajectory follower (MPC + PID) | WP1 | P3 | [#2](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/2) |
| `I16` | Vehicle interface (golf cart) | WP2 | P0 | [#3](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/3) |
| `I5` | stop_mode_operator | WP2 | P0 | [#4](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/4) |
| `I2 + I4` | command_mode_decider + types | WP3 | P0 | [#5](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/5) |
| `I7` | mrm_handler | WP3 | P0 | [#6](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/6) |
| `I3` | command_mode_switcher | WP3 | P0 | [#7](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/7) |
| `I6` | mrm_emergency_stop_operator | WP3 | P0 | [#8](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/8) |
| `I1` | control_command_gate | WP3 | P0 | [#9](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/9) |
| `I14` | shift_decider *(optional)* | WP4 | P3 | [#12](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/12) |
| `I13` | pure_pursuit | WP4 | P1 | [#10](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/10) |
| `I19` | autonomous_emergency_braking | WP5 | P2 | [#13](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/13) |
| `I20` | collision_detector | WP5 | P2 | [#14](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/14) |
| `I8` | control_validator | WP7 | P1 | [#11](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/11) |

New island nodes (`N*`) and HPC-side nodes (`H*`) do not have issues yet.

## Platforms

Two safety-island platforms, one shared code base (design §9.5, D10):

| Design branch | Platform |
| :-- | :-- |
| Least-Cost | NXP S32Z2: S32Z280 on the S32Z280-400EVB, FreeRTOS + nano-ros ([platform choice](docs/PLATFORM-CHOICE.md)) |
| Maximum-Safety | Renesas R-Car X5H |

## Automation

GitHub Actions run the checks and the repository's housekeeping
([docs/design/work-management.md](docs/design/work-management.md)).

| Workflow | Runs | Does |
| :-- | :-- | :-- |
| [gate](.github/workflows/gate.yaml) | On pull requests, the merge queue and pushes to `main` | Every code check; the one required status check ([CONTRIBUTING.md](CONTRIBUTING.md)) |
| [integration](.github/workflows/integration.yaml) | By hand, nightly, the merge queue, pull requests labelled `run:integration` | Autoware + island + scenario world bridge on a self-hosted runner ([docs/testing.md](docs/testing.md)) |
| [auto-arm](.github/workflows/auto-arm.yaml) | When a pull request is opened or marked ready | Arms auto-merge (squash) for a maintainer's pull request |
| [claim](.github/workflows/claim.yaml), [claim-expiry](.github/workflows/claim-expiry.yaml) | On `/claim` and `/unclaim` comments; daily | Assigns a ready issue to the commenter; drops claims idle for 14 days |
| [pr-labeler](.github/workflows/pr-labeler.yaml) | On pull requests | Labels by path ([labeler.yaml](.github/labeler.yaml)) |
| [sync-files](.github/workflows/sync-files.yaml), [update-codeowners-from-packages](.github/workflows/update-codeowners-from-packages.yaml) | Daily, once the App secrets are set | Autoware's shared files; CODEOWNERS from `package.xml` |

## Planning documents

| Document | Content |
| :-- | :-- |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | How the two platforms share code: layers, rules, node placement, open points |
| [docs/WP0-WP3-SafetyFloor.md](docs/WP0-WP3-SafetyFloor.md) | WP0–WP3 node by node: messages, the WP1 partition map and board setup, inputs/outputs, build tasks, tests, demo D1 |
| [docs/MILESTONES.md](docs/MILESTONES.md) | Index of the milestones M0-M7 and their tracking issues on GitHub |
| [docs/PLATFORM-CHOICE.md](docs/PLATFORM-CHOICE.md) | Why the S32Z280-400EVB is the Least-Cost board; the S32K389EVB-Q437 assessed and not chosen |
| [docs/design/implementation-conventions.md](docs/design/implementation-conventions.md) | How Autoware's conventions apply to the Rust and nano-ros code base: language rule, layout, sources of truth, gates (proposal) |
| [docs/PLAN-S32Z2.md](docs/PLAN-S32Z2.md) | Implementation and demo plan on NXP S32Z2: node inventory, node dependency order, work packages, tests, demos |
| [slides/L4SafetyIsland-ImplementationPlan.pdf](slides/L4SafetyIsland-ImplementationPlan.pdf) | Slide deck presenting the plan: phases, work packages, partitions, issues; every slide links back to these documents ([source](slides/L4SafetyIsland-ImplementationPlan.tex)) |

## License

Apache License 2.0 ([LICENSE](LICENSE)). Copyright 2026 The Autoware Foundation ([NOTICE](NOTICE)).
