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

**Importing the Autoware nodes onto the S32Z2 island** (X-S32Z27X-DC, FreeRTOS + nano-ros). The
plan is in [docs/](docs/); the work is tracked in GitHub issues (below).

## Progress

GitHub keeps the live state: an open issue is work still to do, a closed one is done. The links
below open the current lists.

<!-- progress-counts:start -->
The counts in brackets are (completed / open) issues, recounted by the
[progress workflow](.github/workflows/update-progress.yml) whenever an issue changes
(last update: <!-- progress-counts:date -->2026-11-05).

| View | Link (completed / open) |
| :-- | :-- |
| Tracking issue: pickup order, dependencies, open decisions | [#1](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/1) |
| **All open issues** | [open issues](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues?q=is%3Aissue+is%3Aopen) (0 / 14) |
| Open imported-node issues | [open `imported-node`](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues?q=is%3Aissue+is%3Aopen+label%3Aimported-node) (0 / 14) |
| Open issues by work package | [WP1](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues?q=is%3Aissue+is%3Aopen+label%3Awp1) (0 / 1) · [WP2](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues?q=is%3Aissue+is%3Aopen+label%3Awp2) (0 / 2) · [WP3](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues?q=is%3Aissue+is%3Aopen+label%3Awp3) (0 / 5) · [WP4](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues?q=is%3Aissue+is%3Aopen+label%3Awp4) (0 / 3) · [WP5](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues?q=is%3Aissue+is%3Aopen+label%3Awp5) (0 / 2) |
| Open issues in the safety core (partition P0) | [open `P0`](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues?q=is%3Aissue+is%3Aopen+label%3AP0) (0 / 7) |
| Completed | [closed issues](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues?q=is%3Aissue+is%3Aclosed) (0 / 14) |
<!-- progress-counts:end -->

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
| `I8` | control_validator | WP4 | P1 | [#11](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/11) |
| `I14` | shift_decider *(optional)* | WP4 | P3 | [#12](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/12) |
| `I13` | pure_pursuit | WP4 | P1 | [#10](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/10) |
| `I19` | autonomous_emergency_braking | WP5 | P2 | [#13](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/13) |
| `I20` | collision_detector | WP5 | P2 | [#14](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/issues/14) |

New island nodes (`N*`) and HPC-side nodes (`H*`) do not have issues yet.

## Platforms

Two safety-island platforms, one shared code base (design §9.5, D10):

| Design branch | Platform |
| :-- | :-- |
| Least-Cost | NXP S32Z2 (X-S32Z27X-DC board, FreeRTOS + nano-ros) |
| Maximum-Safety | Renesas R-Car X5H |

## Planning documents

| Document | Content |
| :-- | :-- |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | How the two platforms share code: layers, rules, node placement, open points |
| [docs/WP0-WP3-SafetyFloor.md](docs/WP0-WP3-SafetyFloor.md) | WP0–WP3 node by node: messages, inputs/outputs, build tasks, tests, demo D1 |
| [docs/PLAN-S32Z2.md](docs/PLAN-S32Z2.md) | Implementation and demo plan on NXP S32Z2: node inventory, node dependency order, work packages, tests, demos |
