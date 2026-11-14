# Rules for coding agents

These rules apply to any coding agent working in this repository. They
restate [CONTRIBUTING.md](CONTRIBUTING.md) for agents; where the two differ,
CONTRIBUTING.md wins and this file is out of date.

## Flow

- Every change goes through a pull request and the merge queue. Never push to
  `main`, never merge `main` into a branch that is in the queue, never
  force-push a branch whose pull request is under review without the human
  author's agreement.
- One required check, `gate`. Run the fast lane before pushing:
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  the Cortex-R52 clippy line of [docs/testing.md](docs/testing.md),
  `pre-commit run --all-files`, and `nros build --all` when `src/` changed.
- Sign off every commit with the human operator's identity (`git commit -s`)
  only when the operator has told you to. Never sign off as yourself.
- Pull request titles: Conventional Commits, lower-case description. Fill in
  the template, including "AI usage" with a verification statement.
- Keep pull requests small; open only what the operator can shepherd. Do not
  post issues, comments or reviews without the operator reviewing each one.

## Sources of truth

One source per fact (table in
[docs/design/implementation-conventions.md](docs/design/implementation-conventions.md)
section 5). Edit the source, regenerate, never patch the generated side:

- message types: `src/autoware_safety_island_msgs/msg/*.msg`, bounded;
  no hand-written message structs in Rust or C++;
- parameters: the node package's `schema/` and `config/`; branch values in
  the bringup's `config/<branch>/`;
- deployments: `src/autoware_safety_island_bringup/system.toml` and its launch
  file; a branch is a launch argument, never a `cfg` or `#ifdef`;
- rates, budgets, FTTIs, priorities, monitors: the bringup's contract;
- toolchains: `rust-toolchain.toml`, `nros-toolchain.toml`.

Never commit `build/`, `generated/`, `.cargo/` or `src/*/metadata/`; nano-ros
writes them.

## Language and layout

- Ported Autoware packages (`I*`) stay C++, on nano-ros's rclcpp-compatible
  API, under their Autoware name in `src/`.
- New island nodes (`N*`) are Rust: logic as a `si_nodes::StepNode` in
  `crates/si-nodes` (`no_std`, no heap after init, no I/O, no middleware
  type), bound to endpoints by a package in `src/`.
- `crates/` never depends on `platforms/` or `src/`. Platform code lives only
  in `platforms/`.
- `#![forbid(unsafe_code)]` holds in the workspace; a crate that needs
  `unsafe` (a driver) says why in its README and keeps it in `platforms/`.

## Content

- English, keyboard ASCII in commits, pull requests and docs.
- Cite the design by section of `design/RFC_L4_SafetyIslandDesign` (tag
  `v1.0`) and the plan by file and section.
- Do not edit the plan documents, slides, or the milestone and progress
  automation unless the change is asked for; the progress workflow owns the
  counts in `README.md`.
