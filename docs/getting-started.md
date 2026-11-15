# Getting started

This page takes a Linux machine to one that builds and tests the repository:
the Rust crates, the nano-ros images for the host and the S32Z2, and the node
contracts. No board is needed.

## Before you start

- Ubuntu 22.04 (ROS 2 Humble). Ubuntu 24.04 with Jazzy is expected to work
  but is not what CI runs.
- About 10 GB of free disk for the toolchains and build trees.

## 1. Clone with the design submodule

```bash
git clone --recurse-submodules https://github.com/autowarefoundation/L4SafetyIsland_Implementation.git
cd L4SafetyIsland_Implementation
```

`design/RFC_L4_SafetyIslandDesign` is the design baseline (tag `v1.0`).

## 2. Install ROS 2 Humble

Follow the [ROS 2 Humble installation](https://docs.ros.org/en/humble/Installation/Ubuntu-Install-Debs.html)
(`ros-humble-ros-base` is enough), then:

```bash
sudo apt-get install -y python3-colcon-common-extensions ninja-build zstd \
  python3-catkin-pkg python3-empy python3-lark python3-yaml ros-humble-rmw-zenoh-cpp
```

nano-ros generates the message bindings from this install; `nros run` starts the
zenoh router of `ros-humble-rmw-zenoh-cpp`.

## 3. Install Rust

```bash
curl -fsSL https://sh.rustup.rs | sh -s -- -y --default-toolchain none
```

`rust-toolchain.toml` pins the channel and the Cortex-R52 target
(`armv8r-none-eabihf`); rustup installs them on the first `cargo` call.

## 4. Install nano-ros

```bash
curl -fsSL https://raw.githubusercontent.com/NEWSLabNTU/nano-ros/main/scripts/install.sh \
  | sh -s -- --version "$(sed -n 's/^version = "\(.*\)"/\1/p' nros-toolchain.toml)"
export PATH="$HOME/.nros/bin:$PATH"
```

`nros-toolchain.toml` pins the release; every `nros` command run in this tree
uses it. Provision the two boards the images use:

```bash
source /opt/ros/humble/setup.bash
nros setup linux
nros setup freertos-posix
nros setup s32z270-freertos
```

## 5. Build and test

The shared Rust crates, with plain cargo:

```bash
cargo test --workspace
cargo clippy --workspace --exclude si-platform-host --target armv8r-none-eabihf -- -D warnings
```

The nano-ros images of `src/autoware_safety_island_bringup/system.toml`:

```bash
source /opt/ros/humble/setup.bash
nros build host                 # Linux process, Least-Cost branch
nros build host_maximum_safety  # Linux process, Maximum-Safety branch
nros configure --census autoware_safety_island_bringup:s32z2   # once; cross images need it
nros build host_freertos        # FreeRTOS POSIX port, as a Linux process
nros build s32z2                # S32Z2, Cortex-R52, FreeRTOS (no board needed)
```

Run the host image against a zenoh router:

```bash
nros run autoware_safety_island_bringup:host
```

The C++ placeholder prints the heartbeats the Rust placeholder publishes.

## 6. Check the contracts

```bash
pip install play_launch==0.35.0
source /opt/ros/humble/setup.bash
colcon build --base-paths src --packages-select autoware_safety_island_msgs \
  --build-base build/colcon --install-base build/install
source build/install/setup.bash
scripts/check_contracts.sh
```

## 7. Install the commit hooks

```bash
pip install pre-commit
pre-commit install
```

## What runs without hardware

| Target                                                      | Without the board                                                                                                                                                                            |
| :---------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `crates/`, `platforms/host`                                 | Built, tested                                                                                                                                                                                |
| `platforms/s32z2`, `no_std` crates for `armv8r-none-eabihf` | Type-checked with clippy                                                                                                                                                                     |
| Image `host`, `host_maximum_safety`                         | Built and run as Linux processes                                                                                                                                                             |
| Image `s32z2`                                               | Links (in-tree FreeRTOS `ARM_CRx_No_GIC` port); nano-ros 0.6.0-nros10's post-link stack-floor check then fails. Booting needs the NXP GIC port, NETC driver and RTD configuration (WP1, #20) |
| Image `host_freertos` (FreeRTOS POSIX)                      | Links; the same stack-floor check fails                                                                                                                                                      |
| Cortex-R52 in QEMU (`mps3-an536-freertos`)                  | Not wired yet; candidate for a hardware-free run lane                                                                                                                                        |

See [testing.md](testing.md) for the test levels and CI lanes.
