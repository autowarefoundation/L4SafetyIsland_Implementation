cmake_minimum_required(VERSION 3.14)
project(autoware_safety_island_msgs)

# ROS 2 (colcon) build, for the HPC side and the contract checker. nano-ros
# does not run this file: it generates the Rust and C++ types from msg/.

find_package(ament_cmake_auto REQUIRED)
ament_auto_find_build_dependencies()

rosidl_generate_interfaces(${PROJECT_NAME}
  # Design V1.0 section 8.1, HPC -> island
  msg/HpcHeartbeat.msg
  msg/HpcCapability.msg
  msg/ActiveOdd.msg
  msg/OddStatus.msg
  msg/SafetyCorridor.msg
  msg/PullOverRefuge.msg
  # Design V1.0 section 8.2, island -> HPC
  msg/IslandState.msg
  msg/SafetyWorldState.msg
  msg/SensorHealth.msg
  msg/IslandSensorHealth.msg
  msg/OddVerdict.msg
  msg/FallbackAvailability.msg
  msg/SafetyEnvelopeCommand.msg
  # Island-internal (WP0-WP3-SafetyFloor.md section 1.1)
  msg/FaultEvent.msg
  msg/FaultVerdict.msg
  msg/IslandMode.msg
  msg/SensorValidity.msg
  msg/RecoveryVerdict.msg
  DEPENDENCIES
    builtin_interfaces
    geometry_msgs
)

ament_auto_package()
