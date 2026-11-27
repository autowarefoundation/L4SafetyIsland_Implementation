# si-core

## Purpose

The island state machine (design section 3.2), the `v_hold` ramp, timers and fault bookkeeping.

## Contents

- `fault_code`: the `N8` fault-code table of WP3 (WP0-WP3-SafetyFloor.md
  section 1.2): each code's number, name, reporting node and edge. The
  numbers come from the `CODE_*` constants of
  `src/autoware_safety_island_msgs/msg/FaultEvent.msg`;
  `platforms/host/tests/fault_code_table.rs` fails when the two disagree.

WP3 (`N3` si_state_machine, `N8` si_fault_manager) adds the rest.

## Design notes

Pure logic, `no_std`, no heap after init.

## Build and test

```bash
cargo test -p si-core
```
