# L4 Safety Island for Autoware — Implementation and Demo

Implementation and demonstration of the L4 Safety Island software design for Autoware, built on
[nano-ros](https://github.com/NEWSLabNTU/nano-ros).

## Design baseline

| Item | Value |
| :-- | :-- |
| Design document | *L4 Safety Island for Autoware — Software Design*, Part 1 |
| Design version | **20261023** |
| Source | [`autowarefoundation/RFC_L4_SafetyIslandDesign`](https://github.com/autowarefoundation/RFC_L4_SafetyIslandDesign), commit `a585e95` |

Node identifiers used in this repository (`I*` migrated Autoware nodes, `N*` new island nodes,
`H*` new HPC nodes, behaviours `B1`–`B6`) follow the design document.

## Status

**Planning stage.** This repository currently holds the implementation and demo plan only; no code
is written here yet.

## Platforms

Two safety-island platforms, one shared code base (design §9.5, D10):

| Design branch | Platform |
| :-- | :-- |
| Least-Cost | NXP S32Z2 |
| Maximum-Safety | Renesas R-Car X5H |

## Planning documents

| Document | Content |
| :-- | :-- |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | How the two platforms share code: layers, rules, node placement, open points |
| [docs/WP0-WP3-SafetyFloor.md](docs/WP0-WP3-SafetyFloor.md) | WP0–WP3 node by node: messages, inputs/outputs, build tasks, tests, demo D1 |
| [docs/PLAN-S32Z2.md](docs/PLAN-S32Z2.md) | Implementation and demo plan on NXP S32Z2: node inventory, node dependency order, work packages, tests, demos |
