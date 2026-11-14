# si-hal

## Purpose

Hardware abstraction traits shared by every platform: the monotonic clock today; watchdog, CAN FD, Ethernet and camera as WP1-WP5 need them.

## Contents

`Instant` (nanoseconds since boot) and the `Clock` trait.

## Design notes

Implemented by each crate under `platforms/`. Shared crates use the traits only (ARCHITECTURE.md rule 1).

## Build and test

```bash
cargo test -p si-hal
```
