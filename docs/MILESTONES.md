# Milestones — Safety Island on NXP S32Z2

| | |
| :-- | :-- |
| Design baseline | *L4 Safety Island for Autoware — Software Design*, **Release V1.0** (version 20261023) |
| Plan | [PLAN-S32Z2.md](PLAN-S32Z2.md) (work packages, tests, demos), [WP0-WP3-SafetyFloor.md](WP0-WP3-SafetyFloor.md) |
| Status | Proposed. Milestones have **no due dates yet**; dates are set once WP0 has a measured pace |
| On GitHub | Kept in [`milestones.json`](../.github/milestones/milestones.json). The [Sync milestones](../.github/workflows/sync-milestones.yml) workflow applies it on every push to `main` that changes it, through [`create_milestones.py`](../.github/milestones/create_milestones.py). Edit the JSON and push; there is nothing to run by hand |

## 1. How the milestones are cut

A **phase** (design §12) is a capability the island reaches; a **demo** (D1–D5) is the evidence that
it reached it. A milestone closes when **one capability is reached and shown**. It groups the work
packages that build that capability, and its exit is the phase's capability plus the demo. Where the
plan has no demo, the exit is the work package's exit test.

The order is:

1. the **minimal pull-over island** of design §9.4 (phases 1, 3 and 4 on their mandatory nodes:
   D1, D3);
2. the **guardian role and ODD degradation** (phases 2 and 5: D4);
3. **local control** (D5);
4. **lane hold** after HPC loss (B2: D2), postponed to last among the behaviours;
5. audit, before any pilot.

The work packages are numbered in this order (PLAN §3.1).

| Milestone | Phase(s) reached | Demo / exit | Work packages | HPC side | Imported-node issues (others: §4) |
| :-- | :-- | :-- | :-- | :-- | :-- |
| **M0** Foundations and phase 0 on the board | 0 (on target) | S32Z280-400EVB brought up; phase 0 reproduced on it; contracts and E2E threat table clean in CI | WP0, WP1 | — | #2 |
| **M1** Safety floor — **D1** | 1 (autonomous fallback) | **D1**: HPC loss and island blindness on the bench, B4 inside 70 ms | WP2, WP3 | H1 H7 H2 H5 | #3 #4 #5 #6 #7 #8 #9 |
| **M2** Obstacle-aware bounded stop | 3 (obstacle-aware MRM) + phase-2 fallback tier | In-lane bounded stop after HPC loss, shortened for obstacles; first T5 run on the golf cart | WP4, WP5 | — | #10 #12 #13 #14 |
| **M3** Minimal pull-over island — **D3** | 4 (pull-over MRM) | **D3**: HPC lost and not returning; the golf cart pulls over to a nominated, then an island-detected refuge | WP6 | H8 | — |
| **M4** Guardian role and ODD degradation — **D4** | 2 (complete) + 5 | **D4**: fogged lens → ODD-switch demand, answer verified or escalated; `I8` reject → fallback tier | WP7 | H4 H6 H3 | #11 |
| **M5** Local control — **D5** | V1.0 addition (B6; B5 stubs) | **D5**: wired joystick drives the stopped cart off the road at creep speed | WP8 | — | — |
| **M6** Lane hold after HPC loss — **D2** | V1.0 addition (B2) | **D2**: HPC fails on a city road, the island holds the lane, the HPC comes back; D3 and D4 re-run through B2 | WP9 | — | — |
| **M7** Pilot readiness | V1.0 addition (audit) | WP10 exit: event recorder meets graded-reaction step 4; required before any pilot | WP10 | — | — |

```
M0 ── M1 (D1) ── M2 ── M3 (D3) ── M4 (D4) ── M5 (D5) ── M6 (D2) ── M7
                 │        ▲
        WP5 ─────┘        │          WP8 (D5) may start after M1: it needs only the I1 ports
```

## 2. What the order changes, and why it stays inside the design

1. **Pull-over before lane hold.** §9.4 excludes `N5` from the minimal set, because pull-over
   deliberately leaves the lane. So B3 does not need the B2 machinery. Until WP9, `N3` takes the B3
   branch directly on confirmed HPC loss, because the `LANE_KEEP_HOLD` guard stays false. `N7`'s
   lateral path runs on WP4's bounded longitudinal ramp, not on `v_hold`. This stages the design's
   behaviour; it does not change it.
2. **The guardian role is promoted ahead of lane hold, and `I8` moves from WP4 to WP7.** WP7 carries
   all of phase 2 that the minimal set excludes: `I8`, `N5`, `N10`, and `N2` compliance supervision.
   It also carries phase 5 (`N19`). `I8`'s source row in `N8` is declared from M1. WP4's two-tier
   switch is tested on an `I10` overrun in M2, and on an `I8` reject in M4.
3. **`H6` and `H3` move forward with WP7.** The ODD-switch demand and `SafetyEnvelopeCommand` reach
   the HPC through `H1 → H6` (edge F7). `H3` is `N5`'s preferred corridor source. WP9 therefore
   needs no new HPC node.
4. **B1 before B2.** Until WP9, `N19`'s `T_odd` successor for *HPC lost* is B3 instead of B2. WP9
   restores the §3.2 order (blackout → B4, HPC lost → B2, otherwise → B3) and re-runs D4.
5. **Lane hold is last.** WP9 adds `LANE_KEEP_HOLD`, `T_hold` with its B3 / B4 fork, `v_hold` and the
   recovery verdict. All of it sits on top of `N5` (WP7) and the B3 / B4 paths (WP3, WP6), which are
   already proven.

Vehicle tests (T5) start in **M2** (WP4).

## 3. Milestone details

Each milestone lists its scope, its gates, and its exit. A gate is a decision or design gap that must
be settled before the milestone can close (PLAN §2.4, §5.2). A milestone closes when all of its
issues are closed and its exit is shown.

### M0 — Foundations and phase 0 on the board

- **Scope.**
  - WP0: `autoware_safety_island_msgs`, `I4`, SI-E2E/A and /B configured on nano-ros E2E (Q1),
    per-node `*.contract.yaml`, test harness.
  - WP1: S32Z280-400EVB bring-up (retargeted from the X-S32Z27X-DC evaluation), partition map with
    synthetic loads, Cyclone DDS to the HPC, provisional intra-island transport, tracing, HIL rig;
    `I9`–`I12` retargeted (#2).
- **Gates.** Q8 (migrated `I*` nodes stay C++); Q3 provisional choice recorded; the S32Z280-400EVB
  delivered; Q10 (its Ethernet ports) answered for the HPC link and the camera.
- **Exit.**
  - Contracts check clean in CI, and E2E passes the §8.4 threat table.
  - Trajectory following on the board against unmodified Autoware.
  - All partitions at their §9.1 periods.

### M1 — Safety floor, demo D1 (design phase 1)

- **Scope.**
  - WP2: `I16` (#3), `N12`, `N1`, `N4`, `N2`, `N9`, `I5` (#4).
  - WP3: `N8` (11 ports declared), `N3` (all states, B1–B3 guards false), `I2`+`I4` (#5),
    `I7` (#6), `I3` (#7), `I6` (#8), `I1` with 12 ports declared (#9), `N17` reduced.
  - HPC: `H1 H7 H2 H5`.
- **Gates.** Q3 final (before WP3 timing); Q5 blackout owner and debounce; odometry arbiter (Q6).
- **Exit.** **D1** (WP0-WP3-SafetyFloor.md §5):
  - HPC loss detected ≤ 30 ms.
  - Camera blackout → B4, with fault-to-first-brake ≤ 70 ms measured on the board.
  - The `I7` ↔ `I6` `OperateMrm` path works (§9.3 phase-1 exit criterion).

### M2 — Obstacle-aware bounded stop (design phase 3, phase-2 fallback tier)

- **Scope.**
  - WP4: `N7` bounded trajectory, `I13` pure pursuit (#10), `I11`/`I9` as the MRM longitudinal law,
    `I14` optional (#12).
  - WP5, which runs in parallel from M0: `N14`, `N13` on the 3D LiDAR, `N15`, `N17` full, `N16`,
    `I19` (#13), `I20` (#14).
- **Gates.**
  - Hazard-lamp owner and MRM longitudinal owner (Q6).
  - Q4 camera; Q7 sensor placement.
  - Q9: the LiDAR's data, which sizes `N13`.
- **Exit.**
  - In-lane bounded stop on a remembered trajectory after HPC loss, shortened for an in-path
    obstacle.
  - `N13` holds its output bound at the full point rate.
  - Blackout distinguishable from an empty road.
  - P3 shed before P2 under overload.

### M3 — Minimal pull-over island, demo D3 (design phase 4)

- **Scope.**
  - WP6: `N18` (refuge go / no-go, abort ladder, island-detected refuge with ultrasonic and LiDAR
    disqualification), `N7` lateral path, `I7` selecting the pull-over MRM.
  - `N3`: direct HPC-loss → B3 transition (§2).
  - HPC: `H8`.
- **Exit.** **D3** on the golf cart, closed course, ≤ 30 km/h. This is the §9.4 pull-over island:
  21 mandatory nodes plus the 3 recommended.

### M4 — Guardian role and ODD degradation, demo D4 (phase 2 complete, phase 5)

- **Scope.**
  - WP7, guardian role: `I8` control validator (#11) and the two-tier switch on reject; `N5`
    corridor monitor; `N10` and `SafetyEnvelopeCommand`; `N2` compliance supervision.
  - WP7, B1: `N19`.
  - HPC: `H4`, `H6`, `H3`.
- **Exit.** **D4**:
  - Visibility loss detected.
  - An HPC-declared fog domain fails verification.
  - `T_odd` expiry picks its successor (HPC lost → B3 until M6).
  - The HPC is constrained before it is overridden.
  - `N5` flags a trajectory that leaves the corridor.

### M5 — Local control, demo D5

- **Scope.** WP8: `N23` wired joystick; `N21`, `N22` as stubs against a test RCC (V1.0 defers RCC
  communication).
- **Gates.** The remote-command interface row (§8.1) defined.
- **Exit.** **D5**, and every row of the §6.6 failure table. Work can start in parallel from M1.

### M6 — Lane hold after HPC loss, demo D2

- **Scope.** WP9: `N3` `LANE_KEEP_HOLD` with `T_hold` and the fork to B3 / B4, `N7` `v_hold`, `N2`
  recovery verdict and hand-back, and the B2 successor for `T_odd`.
- **Exit.**
  - **D2** in SIL, then on the golf cart.
  - Descent to `v_odd` = 30 km/h inside the hold window.
  - Hand-back only after the confirmation window.
  - D3 re-run via `T_hold` expiry, and D4 re-run with HPC lost → B2.

### M7 — Pilot readiness

- **Scope.** WP10: `N11` event recorder, `N20` object digest (optional).
- **Exit.** Graded-reaction step 4 (audit) met. **Required before any pilot** (§9.4).

## 4. The issues behind each milestone

Every milestone has its issues. The imported Autoware nodes are #2–#14 (the table in §1 lists them).
The rest are defined in [`issues.json`](../.github/milestones/issues.json), and the Sync milestones
workflow opens them with their labels and milestone:

| Milestone | Issues from `issues.json` |
| :-- | :-- |
| M0 | WP0: message package, E2E configuration, contracts and checker, test harness · WP1: board bring-up, partition map, HPC boundary and transport, tracing and HIL rig. Also #1 (tracking) and #15 (WG discussion) |
| M1 | `N12 N1 N9 N2 N4`, reduced `N17`, `N8 N3`; HPC `H1 H7 H2 H5`; demo D1 |
| M2 | `N7` bounded ramp, `N14 N13 N15`, full `N17`, `N16` |
| M3 | `N18`, `N7` pull-over path; HPC `H8`; demo D3 |
| M4 | `N5 N10 N19`, `N2` compliance supervision; HPC `H6 H3 H4`; demo D4 |
| M5 | `N23`, `N21` and `N22` as stubs; demo D5 |
| M6 | B2 lane hold (`N3`, `N7` `v_hold`, `N2` recovery verdict); demo D2 |
| M7 | `N11`, `N20` (optional); HPC `H9` (optional) |

A node built in two stages has one issue per stage, so each issue closes inside one milestone:
`N17` (M1, M2), `N7` (M2, M3, M6) and `N2` (M1, M4, M6). Each issue carries a hidden key, so a
second run never opens it twice. Edits made on GitHub are kept; only the milestone is kept in step
with the file.
