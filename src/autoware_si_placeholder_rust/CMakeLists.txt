cmake_minimum_required(VERSION 3.22)
project(autoware_si_placeholder_rust VERSION 0.1.0 LANGUAGES C CXX)

# The image mixes C++ and Rust packages, so nano-ros builds it with CMake and
# this package registers its Rust component there; cargo builds the crate.
if(NOT COMMAND nano_ros_workspace_pkg_guard)
  message(FATAL_ERROR "Build this package through its image: nros build <image>")
endif()
nano_ros_workspace_pkg_guard()

nano_ros_node_register(
  NAME si_placeholder_rust
  CLASS autoware_si_placeholder_rust::Placeholder
  LANGUAGE RUST
  SOURCES Cargo.toml)
