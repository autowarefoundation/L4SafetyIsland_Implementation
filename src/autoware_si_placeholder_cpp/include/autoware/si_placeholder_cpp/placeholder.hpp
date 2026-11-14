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

#ifndef AUTOWARE__SI_PLACEHOLDER_CPP__PLACEHOLDER_HPP_
#define AUTOWARE__SI_PLACEHOLDER_CPP__PLACEHOLDER_HPP_

#include "autoware_safety_island_msgs.hpp"

#include <rclcpp/rclcpp.hpp>

#include <cstdint>

namespace autoware_si_placeholder_cpp
{

// Placeholder of a ported Autoware node (I*): subscribes to the Rust
// placeholder's heartbeat and counts it.
class Placeholder
{
public:
  ::rclcpp::Result configure(::rclcpp::Node & node);

private:
  void on_heartbeat(const ::autoware_safety_island_msgs::msg::HpcHeartbeat & msg);

  std::uint32_t received_ = 0;
};

}  // namespace autoware_si_placeholder_cpp

#endif  // AUTOWARE__SI_PLACEHOLDER_CPP__PLACEHOLDER_HPP_
