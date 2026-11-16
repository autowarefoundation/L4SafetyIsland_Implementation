# autoware_safety_island_msgs

## Purpose

The message types of the safety island: the HPC link of design V1.0 sections
8.1 and 8.2 and the island-internal types of
[WP0-WP3-SafetyFloor.md](../../docs/WP0-WP3-SafetyFloor.md) section 1.1.

These `.msg` files are the only definition of each type. nano-ros generates
the Rust and C++ types from them; ROS 2 on the HPC builds the same package
with colcon. Every string and sequence carries a bound (`string<=N`,
`T[<=N]`), because the island sizes its pools from it.

## Messages

| Message        | Direction     | Producer -> consumer | Rate   | Age limit | E2E |
| :------------- | :------------ | :------------------- | :----- | :-------- | :-- |
| `HpcHeartbeat` | HPC -> island | `H7` -> `N1` -> `N2` | 100 Hz | 20 ms     | /B  |

The remaining types of issue #16 follow in their own pull request.

## Build

```bash
colcon build --packages-select autoware_safety_island_msgs   # ROS 2
nros interface show autoware_safety_island_msgs/msg/HpcHeartbeat
```
