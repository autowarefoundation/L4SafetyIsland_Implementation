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

### HPC -> island (design V1.0 section 8.1)

| Message                        | Topic               | Producer -> consumer        | Rate   | Age limit | E2E |
| :----------------------------- | :------------------ | :-------------------------- | :----- | :-------- | :-- |
| `HpcHeartbeat`                 | `/si/in/heartbeat`  | `H7` -> `N1` -> `N2`        | 100 Hz | 20 ms     | /B  |
| `HpcCapability`                | `/si/in/capability` | `H2` -> `N1` -> `N2`        | 10 Hz  | 300 ms    | /B  |
| `OddStatus` (with `ActiveOdd`) | `/si/in/odd_status` | `H4` -> `N1` -> `N19`, `N3` | 10 Hz  | 300 ms    | /B  |
| `SafetyCorridor`               | `/si/in/corridor`   | `H3` -> `N1` -> `N5`        | 10 Hz  | 300 ms    | /A  |
| `PullOverRefuge`               | `/si/in/refuge`     | `H8` -> `N1` -> `N18`       | 2 Hz   | 2 s       | /A  |

### Island -> HPC (design V1.0 section 8.2)

| Message                                     | Topic                           | Producer | Rate   | E2E |
| :------------------------------------------ | :------------------------------ | :------- | :----- | :-- |
| `IslandState`                               | `/si/out/state`                 | `N3`     | 50 Hz  | /A  |
| `SafetyWorldState`                          | `/si/out/safety_world`          | `N16`    | 20 Hz  | /A  |
| `IslandSensorHealth` (items `SensorHealth`) | `/si/out/sensor_health`         | `N17`    | 5 Hz   | /A  |
| `OddVerdict`                                | `/si/out/odd_verdict`           | `N19`    | 2-5 Hz | /A  |
| `FallbackAvailability`                      | `/si/out/fallback_availability` | `N18`    | 2 Hz   | /A  |
| `SafetyEnvelopeCommand`                     | `/si/out/constraints`           | `N10`    | 50 Hz  | /A  |

### Island-internal (WP0-WP3-SafetyFloor.md section 1.1)

| Message                                      | Producer -> consumer       |
| :------------------------------------------- | :------------------------- |
| `FaultEvent` (with the WP3 fault-code table) | every fault source -> `N8` |
| `FaultVerdict`                               | `N8` -> `N3`, `I2`         |
| `IslandMode`                                 | `N3` -> `I2`, `I7`         |
| `SensorValidity`                             | `N17` -> `N8`, `N16`       |
| `RecoveryVerdict`                            | `N2` -> `N3`               |

Rates, age limits and E2E profiles are listed here for reading; the node
contract in `autoware_safety_island_bringup` states them for the checker.

Not here: the `I4` command-mode types, which come with the `I2` port, and
the Autoware types the design reuses unchanged (`Trajectory`, `Odometry`,
`Control`, ...), which come from Autoware's own message packages.

## Capacities

Every sequence is bounded (`T[<=N]`); `scripts/check_msg_bounds.py` refuses
an unbounded field in pre-commit. The bounds:

| Field                                              | Bound     | Why                                                  |
| :------------------------------------------------- | :-------- | :--------------------------------------------------- |
| `SafetyCorridor.left_boundary`, `right_boundary`   | 32 points | A fixed-capacity strip around the current trajectory |
| `SafetyWorldState.corridor_left`, `corridor_right` | 32 points | Same strip, perceived                                |
| `IslandSensorHealth.sensors`                       | 8         | One item per D8-ranked sensor kind, with room        |

## Build

```bash
colcon build --packages-select autoware_safety_island_msgs   # ROS 2
nros interface show autoware_safety_island_msgs/msg/HpcHeartbeat
```
