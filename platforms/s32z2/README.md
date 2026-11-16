# si-platform-s32z2

## Purpose

The NXP S32Z2 platform: S32Z280 on the S32Z280-400EVB, Cortex-R52, FreeRTOS.

## Contents

Nothing yet; it type-checks for `armv8r-none-eabihf`.

## Design notes

FreeRTOS, lwIP and the nano-ros transport come from the nano-ros board crate `nros-board-s32z270-freertos`, selected by the `s32z2` image row of `config/autoware_safety_island_bringup/system.toml`. This crate will hold what is specific to the island on that board: the `si-hal` implementation, core map and memory regions (WP1).

## Build and test

```bash
cargo check -p si-platform-s32z2 --target armv8r-none-eabihf
```
