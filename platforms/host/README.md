# si-platform-host

## Purpose

`si-hal` on `std`, for unit tests, software-in-the-loop runs and the WP0 test harness.

## Contents

`HostClock`, and an integration test that drives a `StepNode` from it.

## Design notes

The only shared-code consumer allowed to use `std` besides the X5H A720AE side (ARCHITECTURE.md rule 4).

## Build and test

```bash
cargo test -p si-platform-host
```
