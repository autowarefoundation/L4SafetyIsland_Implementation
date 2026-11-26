# Rules for coding agents

These rules apply to any coding agent working in this repository. They
restate [CONTRIBUTING.md](CONTRIBUTING.md) for agents; where the two differ,
CONTRIBUTING.md wins and this file is out of date.

## Flow

- Work on an issue labelled `status:ready` that your operator claimed
  (`/claim`). Its acceptance tests are your stopping condition; an issue
  without them is not ready. One issue per pull request.
- Every change goes through a pull request and the merge queue, which
  squash-merges it. Never push to `main`, never merge `main` into a branch
  that is in the queue, never force-push a branch once its review has
  started; add commits instead.
- One required check, `gate`. Run the fast lane before pushing:
  `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  the Cortex-R52 clippy line of [docs/testing.md](docs/testing.md),
  `pre-commit run --all-files`, and `nros build --all` when `src/` changed.
- Never sign off as yourself and add no `Co-authored-by` or session trailer
  for an AI tool. Add the operator's sign-off (`git commit -s` with the
  operator's git identity) only when the operator instructs it; the
  certification is theirs.
- Pull request titles: Conventional Commits, lower-case description; the
  title becomes the commit subject on `main`. Fill in the template, including
  "AI usage" with a verification statement (tool, extent, self-review,
  tests run) for the operator to edit.
- Nothing is posted unattended. Push branches; the operator opens the pull
  request, or reviews a draft you opened at their request, and marks it
  ready, which also arms auto-merge (whether agents may open drafts is still
  open: [work-management.md](docs/design/work-management.md) section 8).
  Never mark a pull request ready or arm auto-merge yourself.
  Draft issues, comments and reviews for the operator to review before they
  are posted.
- An operator has at most 5 ready pull requests at a time; keep further work
  as drafts or local branches. Bulk work runs only as sub-issues of a
  tracking issue the maintainers agreed on.

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
- Keep generated text minimal: no comments that narrate the code, no
  narration of your process in commits or pull requests.
- Do not edit the plan documents or the slides unless the change is asked
  for.
