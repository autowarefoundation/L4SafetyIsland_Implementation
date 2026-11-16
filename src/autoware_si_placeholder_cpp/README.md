# autoware_si_placeholder_cpp

## Purpose

Placeholder of a ported Autoware node (`I*`) in C++. A port keeps Autoware's
language and runs on nano-ros's rclcpp-compatible C++ API; this package shows
that shape and is removed when the first port lands.

## Inputs / Outputs

### Input

| Name                        | Type                                           | Description                     |
| :-------------------------- | :--------------------------------------------- | :------------------------------ |
| `/si/placeholder/heartbeat` | `autoware_safety_island_msgs/msg/HpcHeartbeat` | Counted; every tenth is printed |

## Parameters

None.
