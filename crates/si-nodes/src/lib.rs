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
