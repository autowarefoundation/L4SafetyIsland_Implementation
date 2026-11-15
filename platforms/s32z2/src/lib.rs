// Copyright 2026 The Autoware Foundation
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
