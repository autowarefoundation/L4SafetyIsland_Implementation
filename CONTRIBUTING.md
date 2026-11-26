# Contributing

This repository follows the
[Autoware contribution guidelines](https://autowarefoundation.github.io/autoware-documentation/main/contributing/).
This page lists what is specific to it. The conventions behind it are in
[docs/design/implementation-conventions.md](docs/design/implementation-conventions.md).

## Pull request flow

1. Pick an issue labelled `status:ready` and claim it with a `/claim`
   comment (below), or open one with an issue form and agree on the approach
   there. Questions and ideas go to
   [Discussions](https://github.com/autowarefoundation/L4SafetyIsland_Implementation/discussions).
2. Fork the repository and branch from `main`. Start the branch name with the
   issue number, for example `16-msgs-hpc-link`.
3. Make the change in small commits, each signed off (below).
4. Run the fast lane locally ([docs/testing.md](docs/testing.md)).
5. Open a pull request with the template filled in and `Closes #<number>` in
   its description. Open it as a draft until it is ready for review.
6. Mark it ready for review. A maintainer's pull request is armed for
   auto-merge at once; an outside pull request needs one approval from a code
   owner, and a maintainer then arms it. The merge queue runs the gate on the
   pull request merged with `main` and squash-merges it if the gate passes.

Nothing reaches `main` except through the merge queue. Do not merge `main`
into a queued branch; the queue updates it. A draft is never armed; add
`tag:no-auto-merge` to keep a ready pull request out of the queue.

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
- **Squash merges.** The queue squashes a pull request into one commit whose
  subject is the pull request title, so the title must be a good Conventional
  Commit subject; the authors' `Signed-off-by` lines stay in its body.
- Commit messages say why the change is made; the diff shows what.
- Keep a pull request to a size a reviewer can follow in one sitting.
- Force-push only before review starts; after that, add commits.

## Review

- Code owners ([`.github/CODEOWNERS`](.github/CODEOWNERS), generated from
  the `package.xml` maintainers) review the paths they own. An outside pull
  request needs one approval from a code owner; maintainers' own pull
  requests may merge on the gate alone while the project has few code owners.
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

## Labels

| Label                                        | Meaning                                                                                 |
| :------------------------------------------- | :-------------------------------------------------------------------------------------- |
| `status:needs-design`                        | The design or the acceptance tests are still open; not claimable                        |
| `status:ready`                               | Acceptance tests and design references are named; anyone may claim it                   |
| `status:blocked`, `status:info-needed`       | Waiting on another issue, a decision or an answer                                       |
| `status:help-wanted`, `tag:good-first-issue` | Entry points for new contributors                                                       |
| `status:stale`                               | No recent activity (set by the stale bot; nothing is closed automatically)              |
| `type:*`                                     | Kind of change: `bug`, `new-feature`, `improvement`, `documentation`, `ci`, `task`, ... |
| `component:*`                                | Part of the system: `island`, `hpc`, `msgs`, `platform`, `bringup`, `contracts`         |
| `wp:*`, `partition:*`, `platform:*`          | The plan's work package, partition and target                                           |
| `tag:tracking`                               | A tracking issue: its sub-issues are the work, its comments the status                  |
| `tag:no-auto-merge`                          | Keep this pull request out of auto-merge                                                |
| `run:integration`                            | Run the integration lane on this pull request (maintainers apply it)                    |

[`.github/labels.yaml`](.github/labels.yaml) is the full list.

## Claiming an issue

- Comment `/claim` on an issue labelled `status:ready` that has no assignee;
  a bot assigns you. Only ready issues can be claimed, by people and by
  agents' operators alike.
- Open a pull request that closes the issue within 14 days. A claim with no
  linked open pull request after 14 days is released, with a comment; claim
  it again if you are still on it.
- Comment `/unclaim` to release an issue you will not finish.

## AI-assisted contributions

The [Autoware AI contribution policy](https://autowarefoundation.github.io/autoware-documentation/main/contributing/ai-contribution-policy/)
applies to everyone, maintainers included. In this repository that means:

- **The operator is the author.** You review every line an agent wrote
  before it leaves your machine, can explain it unaided, and sign off
  (`git commit -s`). No AI `Co-authored-by` or session trailers.
- **Nothing is posted unattended.** Agents work locally and push branches.
  You open the pull request, or review the agent's draft and mark it ready;
  marking it ready is your attestation, and it is also what arms auto-merge.
  Every issue, comment or review an agent drafts is reviewed by you before it
  is posted.
- **Disclose and verify.** Fill in the pull request's "AI usage" section:
  the tool, the extent, what you reviewed, and the tests you ran.
- **At most 5 ready pull requests per operator.** Keep further work as
  drafts or local branches until one merges. One issue per pull request; an
  issue too large for one reviewable pull request is split into sub-issues
  first. Bulk work runs as sub-issues of a tracking issue the maintainers
  agreed on.
- **Answer reviews yourself.** AI-generated analysis in a reply is quoted
  and marked as such, under your own conclusion.

Coding agents working in this repository follow [CLAUDE.md](CLAUDE.md)
([AGENTS.md](AGENTS.md) points to it).

## Files you do not format

The plan documents (`docs/ARCHITECTURE.md`, `docs/PLAN-S32Z2.md`, ...),
`README.md` and the slides are maintained by the plan owner; pre-commit skips
them.
