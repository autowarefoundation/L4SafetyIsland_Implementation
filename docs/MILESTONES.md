# Milestones — Safety Island on NXP S32Z2

| | |
| :-- | :-- |
| Design baseline | *L4 Safety Island for Autoware — Software Design*, **Release V1.0** (version 20261023) |
| Plan | [PLAN-S32Z2.md](PLAN-S32Z2.md) (work packages, tests, demos), [WP0-WP3-SafetyFloor.md](WP0-WP3-SafetyFloor.md) |
| Status | Proposed. Milestones have **no due dates yet**; dates are set once WP0 has a measured pace |
| On GitHub | Created by [`.github/milestones/create_milestones.py`](../.github/milestones/create_milestones.py) from [`milestones.json`](../.github/milestones/milestones.json) |

## 1. How the milestones are cut

A **phase** (design §12) is a capability the island reaches; a **demo** (D1–D5) is the evidence that
it reached it. A milestone here closes exactly when **one capability is reached and shown**: it
groups the work packages that build that capability, and its exit is the phase's capability plus
the demo (or, where the plan has no demo, the WP exit test).

The order follows the **minimal pull-over island** of design §9.4 / PLAN §3.4: the shortest path to
a demonstrable L4 fallback (phases 1, 3, 4 on their mandatory nodes) comes first, so D1 and D3 are
reached before D2. B2 lane hold, the guardian role, local control and audit then follow.

| Milestone | Phase(s) reached | Demo / exit | Work packages | HPC side | Issues |
| :-- | :-- | :-- | :-- | :-- | :-- |
| **M0** Foundations and phase 0 on the board | 0 (on target) | Phase 0 reproduced on the X-S32Z27X-DC; contracts and E2E threat table clean in CI | WP0, WP1 | — | #2 |
| **M1** Safety floor — **D1** | 1 (autonomous fallback) | **D1**: HPC loss and island blindness on the bench, B4 inside 70 ms | WP2, WP3 | H1 H7 H2 H5 | #3 #4 #5 #6 #7 #8 #9 |
| **M2** Obstacle-aware bounded stop | 3 (obstacle-aware MRM) + phase-2 fallback tier | In-lane bounded stop after HPC loss, shortened for obstacles (WP4 + WP5 exits); first T5 run on the golf cart | WP4 (`N7` `I13` `I9`/`I11`/`I12`, `I14` optional), WP5 | — | #10 #12 #13 #14 |
| **M3** Minimal pull-over island — **D3** | 4 (pull-over MRM) | **D3**: HPC lost and not returning; the golf cart pulls over to a nominated, then an island-detected refuge | WP7 | H8 | — |
| **M4** Lane hold after HPC loss — **D2** | V1.0 addition (B2) | **D2**: HPC fails on a city road, the island holds the lane, the HPC comes back; D3 re-run through the B2 → B3 fork | WP6 | H6 H3 | — |
| **M5** Guardian role and ODD degradation — **D4** | 2 (complete) + 5 | **D4**: fogged lens → ODD-switch demand, answer verified or escalated | WP8, `I8` from WP4 | H4 | #11 |
| **M6** Local control — **D5** | V1.0 addition (B6; B5 stubs) | **D5**: wired joystick drives the stopped cart off the road at creep speed | WP9 | — | — |
| **M7** Pilot readiness | V1.0 addition (audit) | WP10 exit: event recorder meets graded-reaction step 4; required before any pilot | WP10 | — | — |

```
M0 ── M1 (D1) ── M2 ── M3 (D3) ── M4 (D2) ── M5 (D4) ── M7
                 │        ▲
        WP5 ─────┘        │          M6 (D5) may start after M1 (needs only the I1 ports)
```

## 2. Deviations from the WP order of PLAN §3.1

Taking the pull-over first changes two dependencies of the plan. Both stay inside the V1.0 design.

1. **WP7 before WP6.** PLAN §3.2 lists WP6 as WP7's dependency. In M3, on confirmed HPC loss `N3`
   goes straight to the B3 branch (the `LANE_KEEP_HOLD` guard stays stubbed false), and `N7`'s
   lateral displacement path is built without the `v_hold` ramp; `N7` uses the WP4 bounded ramp
   instead. `N2`'s recovery verdict and hand-back move with B2 into M4, where the `T_hold` fork to
   B3 / B4 is added and D3 is re-run through it.
2. **`I8` moves from WP4 to M5.** PLAN §3.4 defers the guardian role (`I8`, `N10`) past the
   pull-over island. `I8`'s port into `N8` stays declared from M1 (an unconnected source row). The
   two-tier switch test of WP4 runs on the `I10` overrun only in M2, and on `I8` reject in M5.

Vehicle tests (T5) are therefore needed from **M2**, not from WP6 as in PLAN §4.1.

## 3. Milestone details

Each milestone lists its scope, its gates (decisions or design gaps that must be settled before it
can close; PLAN §2.4, §5.2), and its exit. A milestone closes when all of its issues are closed and
its exit is shown.

### M0 — Foundations and phase 0 on the board

- **Scope.** WP0: `autoware_safety_island_msgs`, `I4`, SI-E2E/A and /B configured on nano-ros E2E
  (Q1), per-node `*.contract.yaml`, test harness. WP1: partition map with synthetic loads, Cyclone
  DDS to the HPC, provisional intra-island transport, tracing, HIL rig; `I9`–`I12` retargeted (#2).
- **Gates.** Q8 (migrated `I*` nodes stay C++); Q3 provisional choice recorded.
- **Exit.** Contracts check clean in CI; E2E passes the §8.4 threat table; trajectory following on
  the board against unmodified Autoware; all partitions at their §9.1 periods.

### M1 — Safety floor, demo D1 (design phase 1)

- **Scope.** WP2: `I16` (#3), `N12`, `N1`, `N4`, `N2`, `N9`, `I5` (#4). WP3: `N8` (11 ports
  declared), `N3` (all states, B1–B3 guards false), `I2`+`I4` (#5), `I7` (#6), `I3` (#7), `I6` (#8),
  `I1` with 12 ports declared (#9), `N17` reduced. HPC: `H1 H7 H2 H5`.
- **Gates.** Q3 final (before WP3 timing), Q5 blackout owner and debounce, odometry arbiter (Q6).
- **Exit.** **D1** (WP0-WP3-SafetyFloor.md §5): HPC loss detected ≤ 30 ms, camera blackout → B4,
  fault-to-first-brake ≤ 70 ms measured on the board; the `I7` ↔ `I6` `OperateMrm` path works
  (§9.3 phase-1 exit criterion).

### M2 — Obstacle-aware bounded stop (design phase 3, phase-2 fallback tier)

- **Scope.** WP4 minus `I8`: `N7` bounded trajectory, `I13` pure pursuit (#10), `I11`/`I9` as the
  MRM longitudinal law, `I14` optional (#12). WP5 (runs in parallel from M0): `N14`, `N13` on the
  3D LiDAR, `N15`, `N17` full, `N16`, `I19` (#13), `I20` (#14).
- **Gates.** Hazard-lamp owner and MRM longitudinal owner (Q6); Q4 camera; Q7 sensor placement;
  Q9 the LiDAR's data (sizes `N13`).
- **Exit.** In-lane bounded stop on a remembered trajectory after HPC loss, shortened for an
  in-path obstacle; `N13` holds its output bound at the full point rate; blackout distinguishable
  from an empty road; P3 shed before P2 under overload.

### M3 — Minimal pull-over island, demo D3 (design phase 4)

- **Scope.** WP7: `N18` (refuge go / no-go, abort ladder, island-detected refuge with ultrasonic
  and LiDAR disqualification), `N7` lateral path, `I7` selecting the pull-over MRM; direct HPC-loss
  → B3 transition in `N3` (§2). HPC: `H8`.
- **Exit.** **D3** on the golf cart, closed course, ≤ 30 km/h. This is the §9.4 pull-over island:
  21 mandatory nodes plus the 3 recommended.

### M4 — Lane hold after HPC loss, demo D2

- **Scope.** WP6: `N5`, `N3` `LANE_KEEP_HOLD` with `T_hold` and the fork to B3 / B4, `N7` `v_hold`,
  `N2` recovery verdict and hand-back. HPC: `H6`, `H3`.
- **Exit.** **D2** in SIL, then on the golf cart; descent to `v_odd` = 30 km/h inside the hold
  window; hand-back only after the confirmation window; D3 re-run via `T_hold` expiry.

### M5 — Guardian role and ODD degradation, demo D4 (phase 2 complete, phase 5)

- **Scope.** `I8` control validator (#11) and the two-tier switch on reject; WP8: `N19`, `N10` and
  `SafetyEnvelopeCommand`, `N2` compliance supervision. HPC: `H4`.
- **Exit.** **D4**: visibility loss detected; an HPC-declared fog domain fails verification; `T_odd`
  expiry picks its successor in the §3.2 order; the HPC is constrained before it is overridden.

### M6 — Local control, demo D5

- **Scope.** WP9: `N23` wired joystick; `N21`, `N22` as stubs against a test RCC (V1.0 defers RCC
  communication).
- **Gates.** The remote-command interface row (§8.1) defined.
- **Exit.** **D5**, and every row of the §6.6 failure table. Can run in parallel from M1.

### M7 — Pilot readiness

- **Scope.** WP10: `N11` event recorder, `N20` object digest (optional).
- **Exit.** Graded-reaction step 4 (audit) met. **Required before any pilot** (§9.4).

## 4. Issues still to be opened

Only the imported-node issues exist. Before a milestone starts, open one issue per new island node
(`N*`) and per HPC node (`H*`) in its scope, and attach it to the milestone; the demo itself gets
one issue per milestone (D1–D5) that holds the scenario table and the measured results.
