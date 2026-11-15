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

//! Placeholder of a new island node (`N*`): the shape every Rust node package
//! takes. The logic is a `si_nodes::StepNode`; this package only binds it to
//! nano-ros endpoints.
//!
//! Publishes `autoware_safety_island_msgs/HpcHeartbeat` on
//! `/si/placeholder/heartbeat` every `period_ms` (parameter, default 100 ms).

#![no_std]

use autoware_safety_island_msgs::msg::HpcHeartbeat;
use nros::{
    Callback, CallbackCtx, Component, DeclarativeNode, ExecutableNode, NodeContext, NodeOptions,
    NodeResult, TimerDuration,
};
use nros_log::{Logger, log_info};
use si_hal::Instant;
use si_nodes::StepNode;

static LOGGER: Logger = Logger::new("si_placeholder_rust");

const TOPIC: &str = "/si/placeholder/heartbeat";

/// The node's logic: a sequence counter, as a pure step function.
#[derive(Default)]
pub struct Sequencer {
    next: u32,
}

impl StepNode for Sequencer {
    type Input = ();
    type Output = u32;

    fn step(&mut self, (): &(), _now: Instant) -> u32 {
        let sequence = self.next;
        self.next = self.next.wrapping_add(1);
        sequence
    }
}

/// The nano-ros component.
pub struct Placeholder;

impl Component for Placeholder {
    const NAME: &'static str = "si_placeholder_rust";
    const ENTITY_BOUNDS: nros::EntityBounds = nros::EntityBounds::exact(1, 0, 0, 0, 0);

    fn register(ctx: &mut NodeContext<'_>) -> NodeResult<()> {
        nros_log::register_logger(&LOGGER);
        let period_ms: u64 = ctx
            .param("period_ms")
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        let mut node = ctx.create_node(NodeOptions::new("si_placeholder_rust"))?;
        let publisher = node.create_publisher_for_topic::<HpcHeartbeat>(TOPIC)?;
        let _timer =
            node.create_timer_for_callback_name("on_tick", TimerDuration::from_millis(period_ms))?;
        node.callback_for_name("on_tick")
            .publishes_entity(&publisher)?;
        log_info!(&LOGGER, "Publishing every {} ms", period_ms);
        Ok(())
    }
}

impl ExecutableNode for Placeholder {
    type State = Sequencer;
    const HAS_TICK: bool = false;

    fn init() -> Self::State {
        Sequencer::default()
    }

    fn on_callback(state: &mut Self::State, callback: Callback<'_>, ctx: &mut CallbackCtx<'_>) {
        if callback.as_str() == "on_tick" {
            let msg = HpcHeartbeat {
                sequence: state.step(&(), Instant::default()),
                self_test: HpcHeartbeat::SELF_TEST_PASS,
                ..HpcHeartbeat::default()
            };
            let _ = ctx.publish_to_topic::<HpcHeartbeat, 64>(TOPIC, &msg);
        }
    }
}

nros::node!(Placeholder);
