cmake_minimum_required(VERSION 3.14)
project(autoware_safety_island_msgs)

# ROS 2 (colcon) build, for the HPC side and the contract checker. nano-ros
# does not run this file: it generates the Rust and C++ types from msg/.

find_package(ament_cmake_auto REQUIRED)
ament_auto_find_build_dependencies()

rosidl_generate_interfaces(${PROJECT_NAME}
  # Design section 8.1, HPC -> island
  msg/HpcHeartbeat.msg
  DEPENDENCIES
    builtin_interfaces
)

ament_auto_package()
