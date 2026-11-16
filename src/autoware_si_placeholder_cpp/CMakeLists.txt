cmake_minimum_required(VERSION 3.22)
project(autoware_si_placeholder_cpp VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(nano_ros REQUIRED)
find_package(autoware_safety_island_msgs REQUIRED)

# A typed component: no main(); the image entry constructs the class and
# calls configure(node).
nano_ros_add_node(si_placeholder_cpp
  CLASS autoware_si_placeholder_cpp::Placeholder
  HEADER autoware/si_placeholder_cpp/placeholder.hpp
  SOURCES src/placeholder.cpp)

target_include_directories(autoware_si_placeholder_cpp_si_placeholder_cpp_component
  PUBLIC include)
target_link_libraries(autoware_si_placeholder_cpp_si_placeholder_cpp_component
  PUBLIC autoware_safety_island_msgs__nano_ros_cpp)
