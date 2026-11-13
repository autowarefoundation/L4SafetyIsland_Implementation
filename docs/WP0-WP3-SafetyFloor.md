# WP0–WP3: The Safety Floor, Node by Node (S32Z2)

| | |
| :-- | :-- |
| Parent plan | [PLAN-S32Z2.md](PLAN-S32Z2.md), work packages WP0–WP3 |
| Design baseline | Release V1.0 (version 20261023) |
| Goal | The board structure and phase 0 (WP1), then the nine-node B4 path plus the authority core, running on the S32Z280-400EVB, demonstrated as demo D1 |
| Edge IDs | `A*`–`F*` are rows of the edge catalogue in `messageflow/L4SafetyIsland-MessageFlow.md` §3 |

**Language.** New nodes (`N*`) are `no_std` Rust on nano-ros (decision Q2). Migrated nodes (`I*`)
are **proposed to stay C++ ports** of the Autoware packages (decision Q8, §6): the design's reuse
argument rests on running Autoware's own code, and `I5`–`I7` are already ported near verbatim in the
evaluation island. Each node below is built in this order, which is the dependency order of the
parent plan's §2.2.

---

## 1. WP0: what every node needs first

### 1.1 Message types to define (`autoware_safety_island_msgs`)

Every type needs a fixed capacity, because nano-ros sizes its pools from it. Types consumed in WP2–WP3
are marked **●**; the rest are defined now so the contract set is complete, and filled in later.

| Type | Direction | Producer → consumer | Rate | Age limit | E2E | WP3? |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- |
| `HpcHeartbeat` | HPC → island | H7 → `N1` → `N2` | 100 Hz | **20 ms** | /B | ● |
| `HpcCapability` (incl. `remote_takeover_requested`) | HPC → island | H2 → `N1` → `N2` | 10 Hz | 300 ms | /B | ● |
| `OddStatus` (incl. `active_odd`, `odd_switch_state`) | HPC → island | H4 → `N1` → `N19`, `N3` (ODD latch) | 10 Hz | 300 ms | /B | ● (latch only) |
| `SafetyCorridor` | HPC → island | H3 → `N1` → `N5` | 10 Hz | 300 ms | /A | |
| `PullOverRefuge` | HPC → island | H8 → `N1` → `N18` | 2 Hz | 2 s | /A | |
| `IslandState` | island → HPC | `N3` → `N1` → H6 | 50 Hz | — | /A | ● |
| `SafetyWorldState` | island → HPC | `N16` → `N1` | 20 Hz | — | /A | |
| `IslandSensorHealth` | island → HPC | `N17` → `N1` | 5 Hz | — | /A | ● |
| `OddVerdict` | island → HPC | `N19` → `N1` | 2–5 Hz | — | /A | |
| `FallbackAvailability` | island → HPC | `N18` → `N1` | 2 Hz | — | /A | |
| `SafetyEnvelopeCommand` | island → HPC | `N10` → `N1` | 50 Hz | — | /A | |

Autoware types reused unchanged: `Trajectory` (≤ 250 points), `Odometry`, `AccelWithCovarianceStamped`,
`OperationModeState`, `Control`, `MrmState`, `MrmBehaviorStatus`, `OperateMrm` (service), and the
`autoware_vehicle_msgs` status and command types.

**Internal island types** (not on the HPC link, so no §8 row exists for them; define in WP0):
`FaultEvent` (source, code, severity, timestamp; the one type every `N8` source publishes),
`FaultVerdict` (`N8` → `N3`, `I2`), `IslandMode` (`N3` → `I2`, `I7`), `SensorValidity`
(`N17` → `N8`, `N16`), and the recovery verdict (`N2` → `N3`).

### 1.2 Other WP0 deliverables

| Deliverable | Content |
| :-- | :-- |
| E2E configuration | nano-ros built-in E2E (decision Q1): a Data ID per interface row of §8.1–§8.3, profile A or B per row, and the age limits above |
| Contracts | One `*.contract.yaml` per WP2–WP3 node: endpoints, rates, age limits, partition (P0/P1), and the hazard `hpc_loss` with FTTI 70 ms. The checker must pass on the §9.2 budget: detection 30 ms + decision 20 ms + command 20 ms |
| Test harness | Host-build node runner, scripted publishers for every input in §3 below, trace recorder, golden-trace comparison; an HPC stub that publishes H1/H2/H5/H7 traffic and can stop, freeze or corrupt it |
| Fault-code table | The `N8` codes used in WP3: `HPC_LIVENESS_LOST`, `HPC_COMMAND_INVALID`, `E2E_VERIFY_FAIL`, `TIME_BASE_INVALID`, `ACTUATOR_NOT_TRACKING`, `SENSING_BLACKOUT` (kept distinct from the later in-lane-stop code), `FSI_SOC_ERROR` |

---

## 2. WP1: phase 0 on the S32Z280-400EVB

FreeRTOS + nano-ros were evaluated on another S32Z2 board, the X-S32Z27X-DC. WP1 first **brings up
the S32Z280-400EVB**: the evaluated setup is retargeted to this board's pin-out, Ethernet PHYs, CAN
transceivers and flash boot. It is the same processor family, so this is a board step, not a
platform port. WP1 then puts the island's own structure on the board: the partitions every later node is
placed into, the HPC boundary, and the measurement setup the 70 ms budget is checked with. Then it
moves the phase-0 controller onto that structure. WP1 writes **no new node**: `I9`–`I12` are the only
nodes it runs, and they are already ported.

### 2.1 Partition and core map

The executors of design §9.1, created now and left empty until their nodes arrive. A partition with
no node yet runs a **synthetic load** (a periodic task with the partition's period and a sized busy
loop), so isolation is measured before the safety nodes exist (§2.3).

| Partition | Nodes, eventually | Period | Band | S32Z2 placement | Runs after WP1 |
| :-- | :-- | :-- | :-- | :-- | :-- |
| **P0** safety core | `I1 I2 I3 I5 I6 I7 I16 N1 N2 N3 N4 N8` | 10 ms | 1 | R52 lockstep | synthetic load |
| **P1** supervision + fallback | `I8 I13 N5 N7 N9 N10 N12 N18`, `N21` command path, `N23` | 20 ms | 2 | R52 lockstep | synthetic load |
| **P2** safety sensing | `I19 I20 N13–N17 N19` (`N20`) | 20–50 ms | 3 | own core; `N14`'s pixel stages on DSP or Neon | synthetic load |
| **P3** nominal control | `I9` (hosting `I10 I11 I12`), `I14` | 20–30 ms | 4 | R52 split | **`I9`–`I12`** |
| **P3s** tele-op streaming | `N22` | 33–100 ms | 4b | R52 split | — |
| **P4** non-real-time | `N11`, telemetry, `N21` status | 100–1000 ms | 5 | R52 split | — |

The exact core numbers and the SRAM region per partition are written into the platform file.
That includes a reservation for `N13`'s 3D LiDAR frame buffer (plan §1.3), to be sized when Q9 is
answered.

### 2.2 Deliverables

| Deliverable | Content |
| :-- | :-- |
| Platform configuration | `platforms/s32z2` + `config/` (ARCHITECTURE.md §3): the executors above, their periods, priority bands, core assignment and memory regions, as nano-ros `system.toml` and launch files |
| Branch profile | Least-Cost on S32Z2: `N6` absent, `N13` present (3D LiDAR). Later WPs instantiate their nodes from this profile, not from code switches |
| Scheduling-matrix check | Design §9.3 risk 3: the P0/P1/P2 bands checked against the nano-ros scheduling wiring matrix for FreeRTOS on Cortex-R52, **before** any timing figure is claimed |
| HPC boundary | Cyclone DDS over Ethernet to the HPC PC, wire-compatible with unmodified Autoware |
| Intra-island transport | Open decision 7 (plan Q3). WP1 picks a **provisional** backend, recorded and flagged, so WP2 can start. It is a `system.toml` choice, so revisiting it later touches no node source |
| Allocation baseline | `rmw-alloc-sites.py` run on the board build: the first entry of the T0 allocation-site report (plan §4.3) |
| Target tracing | Trace points at executor release and completion and at CAN FD transmit. This is the instrument WP3 uses to measure detection 30 ms + decision 20 ms + command 20 ms |
| Board bring-up | FreeRTOS + nano-ros on the S32Z280-400EVB, retargeted from the X-S32Z27X-DC evaluation; Ethernet port allocation per plan Q10 (HPC link and camera on the two on-board ports); CAN FD to the golf cart; boot from the on-board flash |
| HIL rig (T4) | S32Z280-400EVB + HPC PC running the Autoware planning simulator + CAN FD vehicle simulator; the one rig every later T4 test uses |
| Host build (T2/T3) | `platforms/host` built from the **same** partition configuration (FreeRTOS POSIX port or nano-ros on POSIX), so the WP0 harness and the software-in-the-loop runs match the board |

### `I9`–`I12` trajectory_follower_node (hosting `mpc_lateral_controller`, `pid_longitudinal_controller`, `trajectory_follower_base`) · C++ (already ported, phase 0) · P3 · L3

| | |
| :-- | :-- |
| In | **WP1, phase-0 wiring:** `Trajectory` and `OperationModeState` straight from the HPC over Cyclone DDS; odometry and vehicle status from the planning simulator. **From WP2:** the trajectory and mode via `N1` (B1, B2), ego state from `N4` (badge E), the steering report from `I16` (badge V) |
| Out | `Control` (E1). In WP1 it returns to Autoware on the HPC; from WP3 it feeds `I1`'s nominal port |
| Build | Retarget the evaluation-island port into P3 on the board; **OSQP iteration cap and a time budget on `I10`**, so an MPC overrun is bounded; `KEEP_LAST` depth 1 on every input and the 250-point trajectory bound (design §9.3) |
| Tests | Closed loop against the planning simulator, with tracking error comparable to the evaluation island; `I10`'s worst case stays inside the P3 period over recorded hard trajectories with the cap active; interoperation with unmodified Autoware; no allocation after init outside the reported sites |

### 2.3 Partition isolation before any safety node exists

| Inject | Expected | Measured |
| :-- | :-- | :-- |
| CPU hog in P3 | Synthetic P0, P1 and P2 tasks keep their periods | Release jitter and deadline misses per partition |
| `I10` overrun (OSQP cap disabled on purpose) | Same; P3 alone misses | As above |
| HPC link flood on the Cyclone DDS port | P3 input queues stay at depth 1; P0–P2 synthetic periods intact | As above |

The same tests are repeated with the real nodes in WP3 (P0) and WP4 (P3 shed before P2).

**WP1 exit.** **Phase 0 reproduced on the board**: `I9`–`I12` follow Autoware's trajectory in closed
loop against the planning simulator. In addition, all six partitions run at their §9.1 periods,
P3 overload leaves P0–P2 untouched, tracing works end to end on the HIL rig, and the provisional
intra-island transport is recorded.

---

## 3. WP2: the base layer (levels L0–L2)

### `I16` vehicle interface (golf cart) · C++ port pattern, new per vehicle · P0 · L0

| | |
| :-- | :-- |
| In | `I1` selected command (F3, feedback edge; scripted in node tests); actuator feedback over CAN FD (A13) |
| Out | CAN FD commands to the actuator gateway (F4, SI-E2E/B, 100 Hz); `/vehicle/status/{velocity,steering,gear,control_mode}_status` (badge V); achieved steering / brake / torque to `N9` (F5) |
| Build | Golf-cart command set (steering angle + velocity; no `I15`); E2E/B on commands and feedback; hazard-lamp and gear outputs |
| Tests | Command-to-frame mapping; 100 Hz cadence and jitter on the board; E2E/B corruption and loss on the bus detected and raised to `N8`; feedback decoding against recorded golf-cart traffic |

### `N12` si_time_quality_monitor · Rust · P1 · L0

| | |
| :-- | :-- |
| In | gPTP sync state (A2) |
| Out | time-base valid / invalid to `N1` (A3, 1 Hz) |
| Tests | Valid on lock; invalid on offset beyond bound or sync lost; the transition is raised as `TIME_BASE_INVALID` |

### `N17` si_sensor_health, **reduced** · Rust · P2 · L0

WP3 needs only the blackout declaration. The full node comes in WP5.

| | |
| :-- | :-- |
| In | Camera stream: frame arrival, frame rate, a basic validity check (A10) |
| Out | `SensorValidity` (camera); `SENSING_BLACKOUT` to `N8` (C20); `IslandSensorHealth` |
| Tests | Camera unplugged, frozen, black or saturated frames ⇒ blackout declared after the debounce (**open decision 15**: use a provisional value, flagged); blackout is not lifted by any lower-ranked sensor |

### `N1` si_safety_gateway · Rust · P0 · L1

| | |
| :-- | :-- |
| In | The §8.1 signals from H1 (A1); time-base validity from `N12` (A3) |
| Out | Verified signals to their consumers (B1–B15); egress of the §8.2 signals (F7); `E2E_VERIFY_FAIL` events to `N8` (B15) |
| Build | Thin adapter over nano-ros E2E: per-row Data ID, age check against §8.1, **every failure becomes a `FaultEvent`, never a silent drop** |
| Tests | Every §8.4 threat row per interface (corruption, repetition, reordering, loss, delay, masquerade, wrong interface, truncation); age limit at the boundary (accept just under, reject just over); `N12` invalid ⇒ age checks reported invalid, not passed |

### `N9` si_actuator_supervisor · Rust · P1 · L1

| | |
| :-- | :-- |
| In | Achieved values from `I16` (F5); commanded values from `I1` (F6, feedback edge; scripted) |
| Out | `ACTUATOR_NOT_TRACKING` and non-response events to `N8` (D22) |
| Tests | Commanded vs achieved mismatch beyond tolerance; response-time violation; brake-pressure / EPS-availability loss; no false positive during normal tracking on recorded data |

### `I5` stop_mode_operator · C++ (already ported) · P0 · L1

| | |
| :-- | :-- |
| In | Vehicle status (badge V) |
| Out | Stop command, gear P at standstill (E10) |
| Build | Retarget the evaluation-island port to the board; `enable_auto_parking` on |
| Tests | Autoware's own tests for the package; gear P only at standstill; publishes without localization |

### `N2` si_hpc_supervisor · Rust · P0 · L2

| | |
| :-- | :-- |
| In | `HpcHeartbeat`, `HpcCapability` and per-topic arrival times via `N1` (B7); vehicle status (badge V) |
| Out | Island-local `OperationModeAvailability` to `I7` (D1, 100 Hz); `HPC_LIVENESS_LOST`, `HPC_COMMAND_INVALID` to `N8` (D3). Compliance supervision comes in WP7, the recovery verdict (D2) in WP9 |
| Tests | **Three missed 100 Hz heartbeats detected in ≤ 30 ms**; per-topic minimum rate; latency against budget; availability published even with the HPC silent; no flapping on a single late heartbeat |

### `N4` si_vehicle_state_estimator · Rust · P0 · L2

| | |
| :-- | :-- |
| In | Wheel speed, steering, ESC yaw rate and acceleration (A4, via `I16`); island IMU/GNSS; HPC reference odometry via `N1` (B9) |
| Out | Island `Odometry` + `AccelWithCovarianceStamped` (badge E), consumed by every controller and sensing node |
| Build | Dead reckoning on chassis signals; IMU fusion where fitted; **cross-check against the HPC estimate, island-only on divergence or loss** (decision Q6: `N4` as the odometry arbiter) |
| Tests | Drift against a reference trajectory over the B4 stopping distance; IMU loss ⇒ chassis-only with wider covariance; HPC estimate diverging ⇒ switch with an event to `N8` |

**WP2 exit.** With the HPC stub running, the island sees HPC liveness, publishes its own ego state,
and every fault above reaches a stub `N8` as a `FaultEvent`.

---

## 4. WP3: the authority core and B4 (levels L5–L11)

### `N8` si_fault_manager · Rust · P0 · L5

| | |
| :-- | :-- |
| In | **All 11 source ports declared now** (§2.3 of the plan): connected in WP3 are `N1`, `N2`, `N9`, `N17` and the FSI line (A12); `I8`, `I19`, `I20`, `N5`, `N16`, `N21`/`N23` are declared but have no producer yet |
| Out | `FaultVerdict` to `N3` (D13) and `I2` (D14), 100 Hz; DTC store |
| Build | Static fault tree and reaction table as data, not code; blackout and the later "no refuge, in-lane stop" kept as distinct codes; a blackout record names which lower-ranked sensors were still alive |
| Tests | Table-driven: every row of the reaction table; FSI line is highest priority (≤ 1 ms path); an unconnected port never produces a verdict; DTCs persist |

### `N3` si_state_machine · Rust · P0 · L6

| | |
| :-- | :-- |
| In | `FaultVerdict` (D13); `N2` (D2, recovery, WP9); `N16` snapshot (C16, stubbed: corridor invalid); `N19` (C24, WP7) |
| Out | `IslandMode` to `I2` (D15), MRM request to `I7` (D16), `IslandState` (badge H) |
| Build | All states of §6.1: `BOOT → SELF_TEST → STANDBY → ADS_ACTIVE → {DEGRADED, LANE_KEEP_HOLD, MRM_PREPARE, MRM_EXECUTE, MRM_COMPLETE, EMERGENCY_STOP} → SAFE_STOP`. Guards for B2, B3 and B1 are present but **stubbed false** in WP3, so HPC loss goes straight to the bounded stop |
| Tests | Exhaustive transition table; **`EMERGENCY_STOP` reachable from every state, with no guard**; its only successor is `SAFE_STOP`; no edge back up from B4 even if the camera returns; one-way transitions cannot be reversed; property tests over random fault sequences |

### `I2` command_mode_decider · C++ port, **plugins replaced by a static table** · P0 · L7

| | |
| :-- | :-- |
| In | `IslandMode` (D15), `FaultVerdict` (D14), `I8` reject (D19, WP7) |
| Out | Decided command mode to `I3` (D20) |
| Tests | Every row of the static decision table; the rank order of §10.4 holds; no plugin loading at runtime |

### `I7` mrm_handler · C++ (already ported) · P0 · L7

| | |
| :-- | :-- |
| In | `OperationModeAvailability` from `N2` (D1); MRM request from `N3` (D16) |
| Out | `OperateMrm` to `I6` (D12); hazard / turn commands to `I1` (E9); `MrmState` (badge H) |
| Build | Availability re-sourced from `N2`. **`timeout_operation_mode_availability` re-parameterised**: the stock 0.5 s gives 540 ms against the 70 ms FTTI (finding of the launch-contracts work) |
| Tests | Autoware's own tests; the `OperateMrm` service path end to end (**§9.3 phase-1 exit criterion**); the contract checker accepts the new timeout against the 70 ms budget |

### `I3` command_mode_switcher · C++ port, static configuration · P0 · L8

| | |
| :-- | :-- |
| In | Decided mode (D20) |
| Out | Source selection and smooth transition to `I1` (D21) |
| Tests | Each mode transition; transition time; no source change mid-transition |

### `I6` mrm_emergency_stop_operator · C++ (already ported) · P0 · L8

| | |
| :-- | :-- |
| In | `OperateMrm` from `I7` (D12) |
| Out | Emergency-stop control command to `I1` (E8, 100 Hz); `MrmBehaviorStatus` |
| Build | Deceleration `a_emg` (**open decision 15**: provisional value, flagged); steering held and relaxed toward straight under the rate limit (B4 does not steer) |
| Tests | Autoware's own tests; deceleration profile; steering never commanded away from its current angle |

### `I1` control_command_gate · C++ port · P0 · L11

| | |
| :-- | :-- |
| In | **All command-source ports declared now**: connected in WP3 are `I3` (D21), `I6` (E8), `I7` (E9), `I5` (E10), `I9` (E1, the phase-0 nominal controller) and `N1` (E3); reserved are `N7`, `I13`, `I14`, `N10`, the remote command (E2, no §8.1 row yet) and `N23` |
| Out | Selected, filtered command to `I16` (F3), echo (badge H), commanded values to `N9` (F6) |
| Build | Rank order of §10.4; built-in stop on timeout of the selected source |
| Tests | Arbitration by rank; a reserved port never wins; **built-in stop when the selected source times out, with no other node running**; filter limits |

---

## 5. WP3 integration and demo D1

**Integration order.** Close the feedback edges one at a time, each with a T3 (software-in-the-loop) run before
the next: `I1 → I16` (the vehicle loop), then the `I6 → I7` status reply, then the whole B4 path on
the board (T4).

| D1 scenario | Inject | Expected | Measured |
| :-- | :-- | :-- | :-- |
| HPC loss | HPC stub stops the heartbeat | `HPC_LIVENESS_LOST` ≤ 30 ms; authority to the island; bounded stop (B2/B3 guards are still false) | Detection, decision and command times against 30 / 20 / 20 ms |
| Camera blackout | Camera unplugged while driving | `SENSING_BLACKOUT` → B4: hazard lamps at entry, `I6` brakes at `a_emg`, `I5` gear P at standstill | Fault-to-first-brake-command ≤ 70 ms |
| Blackout while already stopping | Camera lost during the HPC-loss stop | B4 entered from the stopping state; no exit back up | Transition trace |
| Actuators not tracking during B4 | CAN FD simulator ignores brake | `N9` → gate built-in stop + FSI line | Path taken outside the node graph |
| Corrupted HPC traffic | Frame mutator | `E2E_VERIFY_FAIL` events, not drops; behaviour unchanged when within limits | Event count = injected count |

**Exit of WP3.** The nine-node B4 path (`N4 N3 N8 N17 I7 I6 I1 I16 I5`) works on the board,
consumes no exteroceptive topic, and is measured inside the 70 ms budget.

---

## 6. Decision added by this breakdown

| # | Decision | Status |
| :-- | :-- | :-- |
| Q8 | Migrated Autoware nodes (`I*`) stay **C++ ports**; only the new `N*` nodes are Rust | **Proposed**. Reason: the design's provenance argument, and the existing C++ ports of `I5`–`I7` and `I9`–`I12`. The alternative (all-Rust island) redoes those ports and needs Autoware's tests as an equivalence check |
