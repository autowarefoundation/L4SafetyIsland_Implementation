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

//! Node logic of the safety island as pure step functions
//! (ARCHITECTURE.md section 3, rule 2).
//!
//! A node here does no I/O, makes no RTOS call and names no middleware type.
//! Its nano-ros package under `src/` owns the publishers and subscriptions and
//! calls [`StepNode::step`] from its callback.

#![no_std]

use si_hal::Instant;

/// One node's logic: inputs and the current time in, outputs out.
pub trait StepNode {
    /// What the node reads in one step.
    type Input;
    /// What the node produces in one step.
    type Output;

    /// Advance the node by one step.
    fn step(&mut self, input: &Self::Input, now: Instant) -> Self::Output;
}
