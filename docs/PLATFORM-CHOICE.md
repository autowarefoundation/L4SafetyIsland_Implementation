# Platform choice for the Least-Cost safety island

| | |
| :-- | :-- |
| Decision | The Least-Cost island is built on the **NXP S32Z280-400EVB** (S32Z280, 400 MAPBGA). It replaces the X-S32Z27X-DC as the target board |
| Design baseline | *L4 Safety Island for Autoware — Software Design*, **Release V1.0** (version 20261023): §4.1, §9.1–§9.5 |
| Plan baseline | [PLAN-S32Z2.md](PLAN-S32Z2.md), [WP0-WP3-SafetyFloor.md](WP0-WP3-SafetyFloor.md) |
| Status | Decided 2026-11-13, from published NXP material. No measurement on the S32Z280-400EVB yet |

## 1. Decision

| Board | Processor | Outcome |
| :-- | :-- | :-- |
| **S32Z280-400EVB** | S32Z280, 400 MAPBGA | **Target board** for the Least-Cost island |
| X-S32Z27X-DC | S32Z2 | Replaced as the target. FreeRTOS + nano-ros were evaluated on it (phase 0); that evaluation is retargeted in WP1 |
| S32Z280-594EVB | S32Z280, 594 MAPBGA | Not chosen: one Gigabit Ethernet port instead of two, and LPDDR4 the island does not need |
| S32K389EVB-Q437 | S32K389, 4 × Cortex-M7 at 320 MHz, 2.3 MB SRAM | Not chosen: conditionally capable only (§4) |

The S32Z280 is the S32Z2 silicon that design §9.5 recommends and the plan is sized for. The released
design names the processor (S32Z2), not a board, so **the design is unchanged**. Only the
implementation plan changes.

## 2. Why the S32Z280-400EVB

- **The design's own platform.** 8 × Cortex-R52 at up to 1 GHz with lockstep or split clusters, Neon,
  the 25 GFLOPS DSP/ML processor, and 19 MB of on-chip SRAM. `N14`'s pixel stages stay on the DSP or
  Neon, and the 3D LiDAR's frame buffer fits next to the camera's (PLAN §1.3). Neither holds on the
  S32K389 (§4).
- **Same family as the evaluated setup.** FreeRTOS + nano-ros and the phase-0 controller port carry
  over. WP1 needs a board bring-up step (pin-out, Ethernet PHYs, CAN transceivers, flash boot), not a
  platform port.
- **Two Gigabit Ethernet ports on the board**, against one on the 594EVB. The island needs links to the
  HPC, the camera and the LiDAR (Q10).
- **No LPDDR4, and none needed.** §9.5 keeps every frame buffer in on-chip SRAM. The 400 MAPBGA package
  is also the cheaper one to build a production ECU around, which fits the Least-Cost branch.
- **Safety and security as the design assumes**: ASIL D lockstep clusters for P0 and P1, the FSI
  hardware fault line, the HSE for secure boot and keys, and 24 CAN FD interfaces on chip.

## 3. The S32Z280-400EVB

| Item | Published (NXP board page) | Use on the island |
| :-- | :-- | :-- |
| Processor | S32Z280, "production-grade 16 nm", 400 MAPBGA | All six §9.1 partitions, as in WP0-WP3-SafetyFloor.md §2.1 |
| Ethernet | 2 × 1 Gbps | HPC link (Cyclone DDS) and camera; the LiDAR needs a third link (Q10) |
| CAN | 2 × CAN FD, 2 × CAN XL; 1 × LIN | `I16` golf cart (SI-E2E/B); ultrasonics, IMU / GNSS |
| Memory on board | HyperFlash, serial flash, SDHC, EEPROM; no LPDDR4 | Boot and execute-in-place from flash; `N11`'s event ring in flash |
| Debug | JTAG, Aurora trace | Target tracing for the 70 ms budget (WP1, WP3) |
| Expansion | S32X-MB I/O extension board (CAN FD, Ethernet, FlexRay, LIN, motor control) | A third Ethernet port if Q10 needs one |
| Kit | Board, 12 V supply, standoffs; user guide UG10272 | |
| Price, stock | USD 1,000; 1 in stock (NXP store, 2026-11-13) | Order early. Until delivery, WP0 and the T1–T3 host work proceed without it |

**Still to confirm in UG10272 and the S32Z2 reference manual:**

- the Ethernet PHY types fitted (the camera is 100BASE-T1), and whether the S32Z280's internal TSN
  switch reaches a third port;
- the boot sources the boot ROM supports on this board;
- the internal flash size of this S32Z280 variant.

## 4. The S32K389EVB-Q437 alternative (not chosen)

The assessment below asked whether the S32K389 could carry the island instead. It is kept as the
record of why it was not chosen, and as the starting point if a cheaper production MCU is
reconsidered.

### 4.1 Verdict

**Conditionally capable.** The S32K389 can carry the safety-critical part of the island: the B4 path,
the authority core, fallback control, and the B3 / B2 / B6 behaviours. It is ASIL D, has lockstep
Cortex-M7 cores, a hardware security engine (HSE_B), many CAN FD channels and two Gigabit Ethernet
MACs. Three things do not fit as the plan stands:

1. **The camera node `N14` is near saturation.** No DSP, ML unit or Neon, so the ≈ 0.3 Gop/s classical
   lane and free-space extraction runs on one 320 MHz Cortex-M7 core. That is the same situation the
   design calls "the tightest" for ST Stellar (§9.5), and the design's remedy applies: a certified
   smart camera (open decision 8(b)), or a measured proof that `N14` fits.
2. **The 3D LiDAR does not fit.** Its frame buffer (≥ 1 MB, PLAN §1.3) would not fit next to the camera
   buffers (≈ 1 MB) in 2.3 MB of SRAM. It would also need a third Ethernet port. Use the design's
   **2D lidar** on this board, or no lidar (rank 5 is optional).
3. **It is not a candidate the design lists.** §9.5 names S32Z2, AURIX TC4x and Stellar, and §9.1
   already says that P2 "is the partition most likely to force a larger MCU than the current
   S32Z-class target". The S32K389 is smaller than all three. Adopting it is an implementation
   deviation for the WG to record, as the 3D LiDAR was.

The safety argument does not get weaker on this part. Each omission or deferral narrows what the
island can *see*. The nine-node B4 path consumes no exteroceptive topic, so it does not depend on
`N14`, `N13` or the LiDAR at all.

### 4.2 The two parts side by side

| Resource | S32Z2 (S32Z280), design §9.5 | S32K389 (S32K389EVB-Q437) | Effect |
| :-- | :-- | :-- | :-- |
| CPU | 8 × Cortex-R52, up to 1 GHz, lockstep or split | 3 × Cortex-M7 lockstep or split-lock + 1 × Cortex-M7 in permanent lockstep, 320 MHz | Roughly a fifth of the scalar throughput. Lockstep remains available for P0 and P1 |
| Vector / DSP | Neon; 25 GFLOPS DSP/ML processor | None (Armv7E-M DSP instructions only) | `N14` pixel work on a scalar core (§4.3.1) |
| On-chip SRAM | 19 MB | 2.3 MB (2304 KB) | Fits the design's camera buffers; no room for a 3D LiDAR frame (§4.3.2) |
| Flash | 0–64 MB internal NVM by variant; HyperFlash / QuadSPI; LPDDR4 interface (not on the 400EVB) | 12 MB P-Flash + 256 KB D-Flash, RWW, A/B swap | Code fits; `N11`'s event ring needs care (§4.3.2) |
| Ethernet | TSN-capable, multiple ports | 2 × 1 Gbps MACs; TSN named for the S32K3 family, not confirmed for S32K389 | HPC + camera use both MACs; a lidar needs a switch or a non-Ethernet link (§4.3.3) |
| CAN FD | yes | up to 12 on chip; 2 on the board, plus 4 × LIN | Ample for `I16`, IMU/GNSS, ultrasonics |
| Safety | ASIL D, FSI fault line | ASIL D compliant; HW redundancy, MBIST/LBIST, clock and voltage monitors | P0/P1 on lockstep as the design requires |
| Security | HSE | HSE_B, AES/RSA/ECC acceleration | Usable for keyed SI-E2E if the WG wants masquerade protection beyond CRC |
| Camera input | Ethernet camera (no MIPI CSI-2 on any §9.5 candidate) | Ethernet only, no MIPI CSI-2 | Same as the design: automotive Ethernet camera (open decision 21(b)) |
| nano-ros | Evaluated on this board (phase 0) | Not yet ported; FreeRTOS supported by NXP for S32K3; Rust target `thumbv7em-none-eabihf` | WP1 regains a bring-up step |
| Board price | — | USD 640 (NXP store, November 2026) | |

### 4.3 Workload against the part

#### 4.3.1 Compute

| Workload (design §9.5, Least-Cost) | Load | On the S32K389 |
| :-- | :-- | :-- |
| `N14` camera, 640 × 400 mono, 15 Hz, classical | ≈ 0.3 Gop/s | ≈ 0.94 operations per cycle on one 320 MHz core, sustained. Dual issue and 8/16-bit SIMD make it possible in principle, but a 256 KB frame streamed through a small data cache means **most or all of one core**. **Gate: measure it in WP1** |
| `N13`, 2D lidar, ~1080 points at 40 Hz | ≈ 5 Mop/s | Comfortable |
| `N13`, solid-state 3D LiDAR (PLAN §1.3) | 10⁵–10⁶ points/s to crop, segment and downsample | Compute is possible, but memory and the Ethernet port are not (§4.3.2, §4.3.3) |
| `N15`, `N16`, `N17`, `N19` | tens of Mop/s | Fits on the P2 core, but only if `N14` leaves room |
| P0 and P1 nodes (authority, supervision, fallback) | small; 10 ms and 20 ms periods | Fits on two lockstep cores |
| `I10` MPC (OSQP), P3 | measured on S32Z2 in phase 0 | About 3× slower per core at 320 MHz. **Check whether the S32K389's M7 FPU is double precision**: if it is single only, OSQP in `double` falls back to software floating point. The two-tier design (§4.2) already tolerates an MPC overrun: `I13` takes over |

Ways to bring `N14` into budget, if the WP1 benchmark says it does not fit:

- **Certified smart camera** (open decision 8(b)): `N14` becomes a bus parser. This is the design's own
  remedy for the Stellar.
- **Region of interest:** process only the road below the horizon, roughly half the frame.
- **Lower frame rate:** 10 Hz instead of 15 Hz.

The region of interest and the rate are design parameters (§9.5, `N14`), so they need the WG's
agreement.

#### 4.3.2 Memory (2.3 MB SRAM)

The estimates below are for sizing only; WP1 replaces them with the linker map of the board build.

| Consumer | Estimate |
| :-- | :-- |
| Camera frame buffers (raw, rectified, gradient; design §9.5) | ≈ 1 MB |
| Ethernet DMA rings for two MACs, nano-ros / RMW buffers | 0.15–0.3 MB |
| RTOS tasks and stacks for about 40 nodes | 0.15–0.3 MB |
| Message pools: 250-point and 50-point trajectories, corridor, world state, multiple copies | ≈ 0.2 MB |
| OSQP workspace (`I10`) | 0.05–0.15 MB |
| 2D lidar scan buffers | < 0.05 MB |
| **Total** | **≈ 1.6–2.0 MB of 2.3 MB** |
| 3D LiDAR frame and working buffers (PLAN §1.3) | **≥ 1 MB more: does not fit** |

`N11`'s pre/post-event ring buffer (WP10) should go to flash: the 256 KB D-Flash, or external QuadSPI
flash. It should not sit in SRAM.

#### 4.3.3 Interfaces

| Link | Needs | On the S32K389EVB |
| :-- | :-- | :-- |
| HPC link (Cyclone DDS, heartbeat, trajectory) | 1 Ethernet MAC, TSN preferred | MAC 1 |
| Forward camera, ≈ 31 Mbit/s raw over 100BASE-T1 | 1 Ethernet MAC, 100BASE-T1 PHY | MAC 2. The board's two ports are Gigabit; a 100BASE-T1 PHY or media converter is needed. **Check the fitted PHYs in the board manual** |
| Lidar | Usually Ethernet | **No MAC left.** Options: an automotive Ethernet switch (for example an SJA1110 on the SABRE connector), or a 2D lidar on UART / CAN |
| `I16` golf cart, CAN FD + SI-E2E/B | 1 CAN FD | On board |
| Ultrasonics, IMU / GNSS | CAN FD, LIN or UART | 2nd CAN FD, 4 × LIN |
| `N23` wired joystick | ADC / GPIO | On board |
| Hardware fault line (design: FSI, ≤ 1 ms) | MCU fault collection and error output | Expected from the S32K3 safety architecture; **confirm the error-output pin in the reference manual** |

#### 4.3.4 Partition and core map

The six partitions of design §9.1 on the S32K389. The core count in split-lock mode is not stated in
the published material; the map below needs only four cores.

| Partition | Period | S32K389 core |
| :-- | :-- | :-- |
| P0 safety core | 10 ms | Permanent-lockstep M7 |
| P1 supervision + fallback | 20 ms | Lockstep M7 |
| P2 safety sensing (`N14` …) | 20–50 ms | Lockstep M7, entirely; `N14` dominates it |
| P3 nominal control (`I9`–`I12`), P3s (`N22`), P4 (`N11`, telemetry) | 20–1000 ms | Fourth M7. If split-lock yields more cores, give P3 its own core |

Two of the design's rules hold unchanged on this map. P0 completes without P2 or P3, and under
overload the MPC in P3 is shed first. The difference from the S32Z2 is that **no core is spare**.

### 4.4 What changes in the plan if the S32K389 is adopted

| Plan item | Change |
| :-- | :-- |
| PLAN §1.1, §1.3 | Board and processor; the lidar goes back to the design's **2D lidar** (or none). The 3D LiDAR deviation is withdrawn on this platform |
| WP1 / M0 | Regains **platform bring-up**: FreeRTOS + nano-ros on the S32K389 (Cortex-M7), RTD drivers, Ethernet and CAN FD. Phase 0 (`I9`–`I12`) is re-run on the new board. Two **benchmark gates** are added before M1: `N14`'s pixel kernels on one M7 core, and `I10` MPC solve time (with the FPU-precision check) |
| WP5 / M2 | `N14` follows the benchmark: classical CV, region of interest, or a smart camera (open decision 8). `N13` on a 2D lidar. The lidar's interface follows §4.3.3 |
| WP10 / M7 | `N11` records to flash |
| Decisions | New: the WG accepts the S32K389 as a Least-Cost island outside the §9.5 candidate list. Q9 (3D LiDAR data) is dropped on this platform. Q4 (camera) becomes the critical decision |
| Milestones | Order and demos unchanged. M0 grows (bring-up, two gates); M2 depends on the `N14` gate |
| GitHub | Issue #2 retitled to the S32K389; label `s32z2` renamed; the plan file `PLAN-S32Z2.md` renamed |

### 4.5 Facts still to confirm

- Lockstep pairing in split-lock mode: how many independent M7 cores the S32K389 offers.
- Whether the M7 FPU is double precision.
- TSN on the S32K389 (named for the S32K3 family only).
- The Ethernet PHYs fitted on the S32K389EVB-Q437, and what the SABRE connector carries.
- The fault-collection unit and error-output pin, as the equivalent of the S32Z2's FSI line.
- Per-core TCM sizes, which decide where P0's working set lives.

## 5. Sources

- [S32Z280-400EVB board page, NXP](https://www.nxp.com/design/design-center/development-boards-and-designs/S32Z280-400EVB)
- [S32Z280-594EVB board page, NXP](https://www.nxp.com/design/design-center/development-boards-and-designs/S32Z280-594EVB)
- [S32Z2 product page, NXP](https://www.nxp.com/products/S32Z2) and [S32Z2 fact sheet](https://www.nxp.com/docs/en/fact-sheet/S32Z2FS.pdf)

- [S32K389EVB-Q437 board page, NXP](https://www.nxp.com/design/design-center/development-boards-and-designs/S32K389EVB-Q437)
- [S32K389 fact sheet (via Farnell)](https://www.farnell.com/datasheets/4666656.pdf)
- [S32K3 family page, NXP](https://www.nxp.com/products/S32K3)
- Design V1.0, §4.1, §9.1–§9.5 (S32Z2, AURIX TC4x and Stellar resources and the Least-Cost workload)
