# Contributing

This repository follows the
[Autoware contribution guidelines](https://autowarefoundation.github.io/autoware-documentation/main/contributing/).
This page lists what is specific to it. The conventions behind it are in
[docs/design/implementation-conventions.md](docs/design/implementation-conventions.md).

## Pull request flow

1. Pick an issue, or open one and agree on the approach there. The tracking
   issue (#1) gives the pickup order.
2. Fork the repository and branch from `main`. Start the branch name with the
   issue number, for example `16-msgs-hpc-link`.
3. Make the change in small commits, each signed off (below).
4. Run the fast lane locally ([docs/testing.md](docs/testing.md)).
5. Open a pull request with the template filled in. Open it as a draft until
   it is ready for review.
6. A reviewer approves. Then add the pull request to the merge queue
   ("Merge when ready"). The queue rebases it onto `main`, runs the gate on the
   batch, and merges it only if the batch passes.

Nothing reaches `main` except through the merge queue. Do not merge `main`
into a queued branch; the queue rebases it.

## Required checks

One check is required: **`gate`**. It passes only when every check it calls
passes:

| Check                      | What fails it                                                                                               |
| :------------------------- | :---------------------------------------------------------------------------------------------------------- |
| `semantic-pull-request`    | Title not in Conventional Commits form                                                                      |
| `dco`                      | A commit without its author's `Signed-off-by:` line                                                         |
| `spell-check-differential` | An unknown word in a changed file; add real terms to `.cspell.json`                                         |
| `pre-commit`               | A formatting or lint hook (`pre-commit run --all-files` locally)                                            |
| `json-schema-check`        | A `config/*.param.yaml` that does not match its `schema/*.schema.json`                                      |
| `build-and-test`           | clippy (`-D warnings`), a test, rustdoc, cargo-deny, or an image that does not build                        |
| `contract-check`           | `play_launch check --locked --strict` on a contract, or a deployment parameter file against its node schema |

## Commits and titles

- **Sign off every commit** (Developer Certificate of Origin):
  `git commit -s`. The sign-off certifies that you may submit the change under
  the repository's license. The human author signs off; a tool never does.
- **No AI trailers.** Commits carry no `Co-authored-by` or session trailer
  for an AI tool; disclose AI use in the pull request instead (below).
- **Pull request titles** use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/):
  `type(scope): description`, the description in lower case. Types: `feat`,
  `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`,
  `revert`. The scope is the package or crate, for example
  `feat(autoware_safety_island_msgs): add the HPC link types`.
- Commit messages say why the change is made; the diff shows what.
- Keep a pull request to a size a reviewer can follow in one sitting.

## Review

- At least one approving review from someone other than the author.
- A new push dismisses earlier approvals; unresolved review threads block the
  merge.
- Reviewers check the change against the design section or issue it cites.

## Where things go

| Change                                | Place                                                                                                                                                |
| :------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| New island node (`N*`)                | Logic in `crates/si-nodes` (Rust, `no_std`, a `StepNode`); its package `src/autoware_si_<name>/`                                                     |
| Ported Autoware node (`I*`)           | `src/<autoware package name>/`, C++ as in Autoware, on the nano-ros rclcpp-compatible API                                                            |
| Message type                          | `.msg` in `src/autoware_safety_island_msgs/msg/`, every string and sequence bounded. Never a hand-written message struct                             |
| Parameter                             | `schema/<node>.schema.json` + `config/<node>.param.yaml` in the node package; branch values in `src/autoware_safety_island_bringup/config/<branch>/` |
| Rate, budget, FTTI, priority, monitor | The contract `src/autoware_safety_island_bringup/launch/system.contract.yaml`                                                                        |
| Image, board, node set                | `src/autoware_safety_island_bringup/system.toml` and its launch file                                                                                 |
| Platform code                         | `platforms/<platform>/` only. Shared crates never depend on it                                                                                       |

Each fact has one source; do not restate a number in code or docs that a
contract, schema or message file already states.

## AI-assisted contributions

The [Autoware AI contribution policy](https://autowarefoundation.github.io/autoware-documentation/main/contributing/ai-contribution-policy/)
applies. In short: you are the author of everything you submit; review it
yourself first; disclose substantial AI involvement and add a verification
statement in the pull request's "AI usage" section; answer reviews yourself;
keep pull requests small and few enough to shepherd. Coding agents working in
this repository also follow [CLAUDE.md](CLAUDE.md).

## Files you do not format

The plan documents (`docs/ARCHITECTURE.md`, `docs/PLAN-S32Z2.md`, ...), the
slides, `README.md`, and the milestone and progress automation under
`.github/` are maintained by the plan owner; pre-commit skips them. The
progress workflow rewrites the counts in `README.md` on `main`; edit outside
the `progress-counts` markers only.
