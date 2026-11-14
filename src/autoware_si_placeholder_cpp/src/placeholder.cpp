// Copyright 2026 The Autoware Contributors
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

#include "autoware/si_placeholder_cpp/placeholder.hpp"

#include <cstdio>

namespace autoware_si_placeholder_cpp
{

void Placeholder::on_heartbeat(const ::autoware_safety_island_msgs::msg::HpcHeartbeat & msg)
{
  ++received_;
  if (received_ % 10U == 1U) {
    std::printf("si_placeholder_cpp: heartbeat %u\n", static_cast<unsigned>(msg.sequence));
  }
}

::rclcpp::Result Placeholder::configure(::rclcpp::Node & node)
{
  return ::rclcpp::bind_subscription<
    ::autoware_safety_island_msgs::msg::HpcHeartbeat, Placeholder, &Placeholder::on_heartbeat>(
    node, "/si/placeholder/heartbeat", this);
}

}  // namespace autoware_si_placeholder_cpp
