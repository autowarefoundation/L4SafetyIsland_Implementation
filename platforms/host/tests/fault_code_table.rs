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

//! `FaultEvent.msg` is the source of the fault-code numbers; the `si-core`
//! table must name exactly the same codes with the same numbers.

use si_core::{FaultCode, WP3_FAULT_CODES};

const FAULT_EVENT_MSG: &str =
    include_str!("../../../src/autoware_safety_island_msgs/msg/FaultEvent.msg");

/// The `uint32 CODE_<NAME> = <number>` constants of the message, in file order.
fn codes_in_message() -> Vec<(String, u32)> {
    let mut codes = Vec::new();
    for line in FAULT_EVENT_MSG.lines() {
        let Some(rest) = line.trim().strip_prefix("uint32 CODE_") else {
            continue;
        };
        let (name, number) = rest
            .split_once('=')
            .unwrap_or_else(|| panic!("constant without a value: {line}"));
        let number = number
            .trim()
            .parse()
            .unwrap_or_else(|_| panic!("constant with a bad value: {line}"));
        codes.push((name.trim().to_owned(), number));
    }
    codes
}

#[test]
fn message_and_table_name_the_same_codes() {
    let from_message = codes_in_message();
    let mut from_table = vec![(FaultCode::None.name().to_owned(), 0)];
    for code in WP3_FAULT_CODES {
        from_table.push((code.name().to_owned(), code.as_u32()));
    }
    assert_eq!(
        from_message, from_table,
        "FaultEvent.msg and si_core::fault_code disagree; edit the message, then the table"
    );
}
