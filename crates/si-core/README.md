# si-core

## Purpose

The island state machine (design section 3.2), the `v_hold` ramp, timers and fault bookkeeping.

## Contents

Nothing yet. WP3 (`N3` si_state_machine, `N8` si_fault_manager) fills it.

## Design notes

Pure logic, `no_std`, no heap after init.

## Build and test

```bash
cargo test -p si-core
```
