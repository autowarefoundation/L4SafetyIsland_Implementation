# si-sensors

## Purpose

Sensor decoding: radar, lidar, ultrasonic, IMU and camera.

## Contents

Nothing yet. WP5 fills it.

## Design notes

Pure logic, `no_std`. An absent sensor is an absent input, not a code path (ARCHITECTURE.md rule 3).

## Build and test

```bash
cargo test -p si-sensors
```
