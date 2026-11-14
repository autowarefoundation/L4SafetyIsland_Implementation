# si-nodes

## Purpose

Node logic of the new island nodes (`N*`) as pure step functions.

## Contents

The `StepNode` trait: `step(&mut self, input, now) -> output`.

## Design notes

A node here does no I/O, makes no RTOS call and names no middleware type (ARCHITECTURE.md rule 2). Its nano-ros node package under `src/` owns the endpoints and calls `step` from its callbacks, so the logic is tested on the host with plain `cargo test`.

## Build and test

```bash
cargo test -p si-nodes
```
