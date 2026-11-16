# Reusing autoware-safety-island for I9-I12

|        |                                                                                                                    |
| :----- | :----------------------------------------------------------------------------------------------------------------- |
| Status | **Decided** by the implementation team; answers open point P4 of [ARCHITECTURE.md](../ARCHITECTURE.md)             |
| Source | `autoware-safety-island` (Arm, Apache-2.0), branch `nano-ros-migration` of the NEWSLabNTU fork                     |
| Target | `I9`-`I12` in [PLAN-S32Z2.md](../PLAN-S32Z2.md): trajectory follower, MPC lateral, PID longitudinal, follower base |

## 1. Decision

The repository does not start from `autoware-safety-island`. It takes that
repository's controller port as packages and owns its copy from then on:

1. `I9`-`I12` come in as C++ packages under `src/`, with the libraries they
   need. They stay C++, as every ported Autoware package does.
2. They are rebased once, here, from their Autoware base to 2.0.0, and gain
   the solver bounds `PLAN-S32Z2.md` asks for (OSQP iteration cap, time
   budget on `I10`).
3. Nothing tracks the source repository after the import. It follows Autoware
   Foundation main on an older Autoware release; this repository follows the
   design V1.0 and Autoware 2.0.0.

## 2. What overlaps

| Area          | autoware-safety-island                       | This repository                                |
| :------------ | :------------------------------------------- | :--------------------------------------------- |
| Function      | Autoware's trajectory follower on the island | `I9`-`I12`, partition P3 (QM), bounded by `I1` |
| Autoware base | 2025.02 (Universe 0.40.0)                    | 2.0.0                                          |
| Language      | C++                                          | ported packages C++, new nodes Rust            |
| Runtime       | nano-ros, FreeRTOS and Zephyr                | nano-ros, FreeRTOS                             |
| Target        | S32Z270 (R52), Cortex-R52 on QEMU, POSIX     | S32Z2 (R52), X5H, host                         |
| Build         | `nros build` per image (FreeRTOS images)     | `nros build` per image                         |

What only this repository has: the safety core (`N*` nodes, the B1-B4
ladder, `I1`-`I7`, `I13`, `I16`, `I19`), the island-HPC messages, and the
rlm contract. `I5`-`I7` come from the evaluation island
(`simple-autoware-safety-island`), not from `autoware-safety-island`.

## 3. What is taken

| Package (source)                                                                                                                                    | Use here                                                                                                   |
| :-------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| from the `autoware` library package: `autoware_trajectory_follower_base`, `autoware_mpc_lateral_controller`, `autoware_pid_longitudinal_controller` | `I12`, `I10`, `I11`                                                                                        |
| `controller_pkg` (`autoware_trajectory_follower_node`)                                                                                              | `I9`                                                                                                       |
| `autoware_osqp_interface`, `autoware_interpolation`, `autoware_motion_utils`, `autoware_universe_utils`, `autoware_vehicle_info_utils`              | their dependencies, rebased to 2.0.0                                                                       |
| `asi_common` (the support code the source's root build used to add by hand, e.g. the pthread shim)                                                  | read for what the controllers need from the platform; replaced by this repository's HAL where they overlap |
| the S32Z270 board package and its devicetree                                                                                                        | input to `platforms/s32z2`                                                                                 |

Not taken: the Zephyr apps, the demo, the source repository's message
packages (this repository takes the Autoware 2.0.0 message set instead), and its build scripts.

The source is in the nano-ros form already: its phase-14 port made the
vendored Autoware sources one library package (`autoware`), the node one
package (`controller_pkg`, holding `autoware_trajectory_follower_node`), and
moved what the hand-written root build added into `asi_common`. They move
as packages with their `package.xml`, not as loose sources; splitting the
library into one package per Autoware package happens with the rebase.

## 4. Order

1. After M0 (workspace, messages): import the packages at a recorded source
   commit, unchanged, and build them for `host` and `s32z2`.
2. Rebase to Autoware 2.0.0 (interfaces, parameters, behaviour changes),
   one commit per package.
3. Add the OSQP iteration cap and the `I10` time budget; state the budget in
   the contract.
4. Replace platform shims with the HAL.

Each step is a pull request through the gate. The import commit names the
source repository, branch and commit, and keeps the upstream license and
copyright headers.

## 5. Open items

- `I10` on the X5H runs on the A720AE (QM) behind a cross-cluster transport;
  the source port has no such path.
- The 2.0.0 rebase size is not measured yet. If it is large, step 2 may land
  per controller, `I11` (PID) first, since the MRM longitudinal law needs it
  (PLAN-S32Z2.md, WP4 open question).
