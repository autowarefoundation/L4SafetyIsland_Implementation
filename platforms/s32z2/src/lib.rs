//! NXP S32Z2 platform (S32Z280 on the S32Z280-400EVB, Cortex-R52, FreeRTOS).
//!
//! Will implement `si-hal` on the board. FreeRTOS, the network stack and the
//! nano-ros transport come from the nano-ros board crate
//! `nros-board-s32z270-freertos`, selected by the image row in
//! `config/autoware_safety_island_bringup/system.toml`, not from here.
//!
//! Empty in the scaffold: it type-checks for `armv8r-none-eabihf`, which is
//! the WP1 starting point.

#![no_std]
