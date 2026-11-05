# Implementation Architecture — one code base, two platforms

Design baseline: version **20261023**, §9.5 *Two design branches* and D10.

## 1. What has to be shared, and what may differ

The design fixes the split. The two branches **share** the six behaviours, the node set, the
interface contract and the state machine (§§3–8). They **differ** only in the sensors fitted and the
processor:

| | Least-Cost | Maximum-Safety |
| :-- | :-- | :-- |
| Platform | **NXP S32Z2** — 8 × Cortex-R52 (lockstep/split), DSP, 19 MB SRAM | **Renesas R-Car X5H** — 6 × Cortex-R52 lockstep + 32 × Cortex-A720AE |
| Forward camera | 1, low-res mono, 100BASE-T1 Ethernet | 1, GMSL / CSI-2, higher resolution |
| Radar | none → **`N6` absent** | 4 (front pair, rear pair) → **`N6` present** |
| Lidar (rank 5, optional) | 1 → **`N13` present**. Design: 2D lidar; **the S32Z2 lab build uses a solid-state 3D LiDAR** in the same confirming-only role ([PLAN-S32Z2.md](PLAN-S32Z2.md) §1.3) | none → **`N13` absent** |
| Ultrasonic, IMU/GNSS | 5, 1 | 5, 1 |
| `I19` AEB input | `N13` lidar `PointCloud2` | `N6` radar point cloud |
| `N19` visibility test | contrast + lidar-vs-camera | contrast + 2 × radar-vs-camera |
| `N22` video | forwards camera's encoded stream | encodes on the A720AE cluster |
| Island ODD | city class (~30 km/h), fair weather | highway + city, weather included |

So the code must keep everything in §§3–8 identical, and confine these rows to configuration and a
thin platform layer.

## 2. Principle: branch and platform are two independent axes

- **Branch** (Least-Cost / Maximum-Safety) is a *sensor-set and parameter* choice. It selects which
  sensing nodes are instantiated and which `v_odd` classes the island may claim. It is
  **configuration, not code**.
- **Platform** (S32Z2 / X5H / host) is a *hardware* choice. It supplies drivers, core mapping,
  memory layout and the RTOS. It is the **only place platform code lives**.

Keeping them independent means the S32Z2 board can run the Maximum-Safety sensor set on the bench,
and the host simulator can run both branches. That is what keeps the shared code honest, because each
combination exercises it.

These are **not git branches**. Both live on `main`; a git branch per platform would make sharing
code a merge problem.

## 3. Layers

```
┌──────────────────────────────────────────────────────────────────────┐
│ config/            branch profiles · deployments (system.toml, launch) │  per branch × platform
├──────────────────────────────────────────────────────────────────────┤
│ crates/si-runtime  nano-ros node shells, executors, P0–P4 partitions   │  shared
├──────────────────────────────────────────────────────────────────────┤
│ crates/si-nodes    I*/N* node logic as pure step functions             │  shared
│ crates/si-core     state machine (§3.2), v_hold ramp, timers, faults   │  shared
│ crates/si-msgs     message types, fixed capacity                       │  shared
│ (E2E)              SI-E2E/A, /B = nano-ros built-in E2E, configured    │  middleware
│ crates/si-sensors  radar / lidar / ultrasonic / IMU / camera decoding  │  shared
├──────────────────────────────────────────────────────────────────────┤
│ crates/si-hal      traits: clock, watchdog, CAN FD, Ethernet, camera…  │  shared (traits only)
├──────────────────────────────────────────────────────────────────────┤
│ platforms/s32z2    platforms/x5h/{r52,a720}    platforms/host          │  per platform
└──────────────────────────────────────────────────────────────────────┘
```

> **Language: Rust on the safety island** (decision Q2 in [PLAN-S32Z2.md](PLAN-S32Z2.md)). The new
> island nodes and the shared crates are `no_std` Rust on nano-ros. **E2E** is not a crate of ours:
> nano-ros's built-in protocol satisfies SI-E2E/A and /B (decision Q1) and is configured, not
> implemented.

Rules that keep it that way:

1. **`crates/` never depends on `platforms/`.** Dependencies point down only; `platforms/`
   implements `si-hal` and is chosen at link time.
2. **Node logic is pure.** Each node is `step(&mut self, inputs, now) -> outputs`, with no I/O,
   no RTOS calls and no middleware types. That makes it testable on the host, identical on both
   R52 clusters, and the natural unit for the Part 4 fault-injection matrix.
3. **An absent sensor is an absent input, not a code path.** `N16`, `N19` and `I19` take
   `Option<…>` channels. The branch profile decides which are wired. No `#[cfg(feature = "radar")]`
   inside node logic.
4. **`no_std`, no heap after init** in every shared crate (R4; §9.3's allocation-policy decision).
   `std` is only in `platforms/host` and `platforms/x5h/a720`.
5. **One RTOS on both safety clusters if possible.** If both R52 clusters run the same RTOS
   (**FreeRTOS**, which already runs nano-ros on the S32Z2 board; see §5), the nano-ros glue is shared as well, and the only per-platform
   code is drivers and memory maps.

## 4. Where each node runs

| Partition | S32Z2 | X5H |
| :-- | :-- | :-- |
| P0–P2 (authority, MRM, sensing) | R52 lockstep clusters | R52 lockstep cluster |
| P3 nominal controller (`I10` MPC) | R52 split cores | **A720AE** (QM), bounded by `I1` on R52 |
| P4, `N11`, `N21`, `N22` | R52 split cores (low priority) | **A720AE** |

The X5H's A720AE side is the one real asymmetry. It runs the same `si-nodes` code, built for
`std`, behind a cross-cluster transport. On the S32Z2 the same nodes run as low-priority tasks.

## 5. Open points for the implementation

| # | Question | Why it matters |
| :-- | :-- | :-- |
| P1 | RTOS on the X5H R52 cluster. The S32Z2 runs **FreeRTOS + nano-ros** (evaluated on the X-S32Z27X-DC), so FreeRTOS on the X5H is preferred; is a FreeRTOS port for the X5H R52 cluster available? | Determines whether rule 5 holds |
| P2 | X5H A720AE side: Linux + ROS 2, or nano-ros `std`? | Cross-cluster transport and freedom from interference (open decision 23(d)) |
| P3 | Hardware on hand: **X-S32Z27X-DC in the lab**; X5H evaluation board still to be confirmed | What the first X5H work can run on |
| P4 | Start from `autoware-safety-island` (nano-ros branch, already `I9`–`I12`) or fresh | Reuse of the existing controller port |

## 6. Measuring "as much shared code as possible"

Track it rather than assert it: lines in `crates/` + `config/` over total lines, reported per
platform. Target **≥ 85 % shared** on each platform. A change that adds platform-specific code
outside `platforms/` should fail review.
