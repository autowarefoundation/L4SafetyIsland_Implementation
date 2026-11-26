# Issue and work management

|            |                                                                                                                                                                            |
| :--------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Status     | **Decided** (2026-11-26, by the user); the open points are listed in section 8                                                                                             |
| Supersedes | C7 of [implementation-conventions.md](implementation-conventions.md) (the README progress counts)                                                                          |
| Inputs     | Autoware's contribution guidelines, its AI contribution policy, `autowarefoundation/sync-file-templates`, the `.github/` of `autoware_universe`, nano-ros's agent workflow |

This note records how the repository manages its work: where work is
written down, how people and coding agents pick it up, and how a change
reaches `main`. It adopts Autoware's workflow and adds what open
participation and large-scale AI contribution need.

## 1. Summary

- **GitHub is the one source for work.** Issues replace the issue files
  (`.github/milestones/*.json`); tracking issues with native sub-issues
  replace the phase documents and the README progress counts.
- **Autoware's workflow, adopted as is.** Namespaced labels, the synced issue
  and pull request templates, Conventional Commit titles, DCO, the path
  labeler, the stale bot, CODEOWNERS from `package.xml`, `sync-files` from
  `sync-file-templates`, and squash merges.
- **Open participation.** Issues move from `status:needs-design` to
  `status:ready`; anyone claims a ready issue with `/claim`; claims expire;
  outside participants get Triage.
- **Large-scale AI contribution inside Autoware's AI contribution policy.**
  Agents work on ready issues with acceptance tests; the human operator
  reviews every item before it is posted, signs off and discloses; a cap per
  operator keeps the queue reviewable.
- **Auto-arm for maintainers.** A pull request a maintainer marks ready is
  armed for auto-merge at once; the merge queue runs the full gate (and
  integration) on the merged result and squash-merges it.

## 2. Where the repository stood

Before this change:

| Area             | State                                                                                                                                                                                                                                   |
| :--------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Issues           | 63 GitHub issues in milestones M0-M7, generated from `issues.json` + `issue_patches.json` by `create_milestones.py`; each carries an `issue-key` comment. Edits on GitHub drift from the JSON                                           |
| Phase tracking   | `docs/MILESTONES.md`, `docs/PLAN-S32Z2.md`, one tracking issue (#1, `I*` import), README progress counts kept by `update-progress.yml` (bot branch, dispatched gate, guard)                                                             |
| Labels           | Not namespaced: `wp0`-`wp10`, `P0`-`P4`, `island-node`, `hpc-node`, `demo`, `s32z2`, `tracking`, `work-item`, `imported-node`, GitHub defaults                                                                                          |
| Already Autoware | Lint configuration and pre-commit, spell-check-differential, json-schema-check, semantic pull request title (in `gate`), DCO (in `gate`), PR template with "AI usage", CODEOWNERS, dependabot, C6 (no AI trailers; the human signs off) |
| Merge            | Ruleset: merge queue with `REBASE`, one required check `gate`, 0 approvals. The repository settings allow squash only. The two disagreed                                                                                                |

## 3. GitHub as the single source of work

### 3.1 Issues

- Every `issue-key` in `issues.json` exists as an issue (checked
  2026-11-26: 48 of 48, and every edit of `issue_patches.json` applied), so
  `issues.json`, `issue_patches.json`, `milestones.json`,
  `create_milestones.py`, `sync-milestones.yml`, `edit-issues.yml` and
  `replace_in_issues.py` are deleted. Bulk edits after that are plain
  `gh issue edit` runs.
- Milestones stay as GitHub milestones, managed by hand (eight of them; dates
  only). The narrative moves to tracking issues.
- Issue forms (`.github/ISSUE_TEMPLATE/`): Autoware's synced `bug.yaml` and
  `task.yaml`, plus an L4 `work-item.yaml`: behaviour, nodes, work package,
  acceptance tests (checklist), RFC references. `config.yml` turns blank
  issues off and sends questions, feature requests and ideas to this
  repository's Discussions, as Autoware does.
- Design decisions stay in `docs/design/` (conventions, C1-C7, ASI reuse).
  Issues track work; documents record decisions. A design question opens as a
  Discussion and ends as a document change.

### 3.2 Tracking issues instead of phase documents

Modelled on rust-lang's tracking issues:

- **One tracking issue per milestone** (M0-M7) and per cross-cutting area
  (`I*` import = #1, CI and runner, S32Z2 board bring-up, X5H platform after
  C5).
- **Body:** goal, exit criteria, design links, open questions. Kept current by
  the owner.
- **Children:** GitHub sub-issues, not markdown checklists. Closing a child
  updates the parent's progress bar.
- **Status:** comments on the tracking issue ("this week: N3 merged, N7
  blocked on #18"). The history a phase document kept becomes the thread.
- **Label:** `tag:tracking`; only maintainers edit the body.
- **Views:** a GitHub Project over the issues (board by `status:`, table by
  milestone) for planning; optional.

`docs/MILESTONES.md` shrinks to an index of tracking-issue links. The README
progress counts are dropped: sub-issue progress shows the same numbers live.
That removes `update-progress.yml`, `update_progress_counts.py`, the bot
branch, the `workflow_dispatch` entry of `gate.yaml` and its guard, and with
them the last gap the guard left open (a branch could edit its own
`gate.yaml`). C7 is superseded.

## 4. Autoware's workflow, folded in

| Autoware convention                                              | Here                                                                                                                                                                                                                                                                                                                                                                |
| :--------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `sync-files` from `autowarefoundation/sync-file-templates`       | Adopted with `.github/sync-files.yaml` listing the files that apply unchanged (below). Its pull requests carry `tag:bot`, `tag:sync-files` and go through the queue. Needs the organization's `APP_ID` / `PRIVATE_KEY` secrets; it skips without them                                                                                                               |
| Synced issue templates (`bug`, `task`, `config`)                 | Adopted, plus the L4 `work-item` form. `config.yml` is synced with its Discussions links rewritten to this repository                                                                                                                                                                                                                                               |
| Synced PR template                                               | Kept as is: L4's already matches and adds "Effects on system behavior" and "AI usage"                                                                                                                                                                                                                                                                               |
| `semantic-pull-request` (Conventional Commit title)              | Kept, reported through `gate`. With squash merges the title becomes the commit subject on `main`                                                                                                                                                                                                                                                                    |
| Scope = component (advisory)                                     | `feat(si_hpc_supervisor): ...`, `fix(si-hal): ...`, `ci: ...`                                                                                                                                                                                                                                                                                                       |
| DCO                                                              | Kept in `gate`. Squash keeps the authors' `Signed-off-by` lines in the commit body                                                                                                                                                                                                                                                                                  |
| Squash merge; avoid force-push during review                     | The merge queue squashes (the settings already allow squash only). Force-push only before review starts; after that, add commits                                                                                                                                                                                                                                    |
| `pr-labeler` (`actions/labeler`, path to label)                  | Adopted: `src/autoware_si_*` to `component:island`, `src/autoware_safety_island_msgs` to `component:msgs`, the bringup to `component:bringup`, its contract to `component:contracts`, `crates/`, `platforms/` to `component:platform`, `ci/`, `.github/` to `type:ci`, `docs/` to `type:documentation`                                                              |
| `update-codeowners-from-packages`                                | Adopted: CODEOWNERS generated from `package.xml` maintainers; Rust crates, platforms, CI in `.github/CODEOWNERS-manual`                                                                                                                                                                                                                                             |
| `stale.yml` (`status:stale`, never auto-close)                   | Adopted for pull requests and issues; the claim expiry (section 5) is separate                                                                                                                                                                                                                                                                                      |
| `run:` labels gate costly CI (`run:build-and-test-differential`) | `run:integration` starts the CARLA job on the self-hosted runner for a pull request, through Autoware's `require-label` workflow. Triage can apply labels, so the pull request trigger runs only for branches of this repository; fork pull requests get integration in the merge queue, after review. The queue runs integration on every merge group once enabled |
| Review by code owners; anyone with write access merges           | Section 7: outside pull requests need one code-owner approval; maintainers are armed on ready                                                                                                                                                                                                                                                                       |
| Discussions for questions, ideas, large changes                  | Discussions enabled on the repository (Q&A, Ideas, Design); `config.yml` links there                                                                                                                                                                                                                                                                                |
| `github-release`, `bump-version`                                 | Later, with the first tagged release                                                                                                                                                                                                                                                                                                                                |
| AI contribution policy                                           | Binding; section 6 shows how the AI lane fits it                                                                                                                                                                                                                                                                                                                    |

The synced set is the files L4 uses unchanged: the issue templates,
`stale.yml`, `sync-files.yaml`, `update-codeowners-from-packages.yaml`,
`.markdownlint.yaml`, `.prettierrc.yaml`, `.prettierignore`, `.yamllint.yaml`
and `CODE_OF_CONDUCT.md`. Not synced, because L4 changed them on purpose:
`.pre-commit-config.yaml` (Rust formatter, excluded plan files), the
`pre-commit`, `semantic-pull-request` and `spell-check-differential`
workflows (called from `gate` with `workflow_call`, not on `pull_request`),
`CONTRIBUTING.md` (the L4 section) and the pull request template.

### 4.1 Labels

Autoware's namespaces, with the L4 terms mapped onto them. Renames keep the
labels on their issues (`gh label edit --name`).
[`.github/labels.yaml`](../../.github/labels.yaml) is the list and
[`.github/scripts/sync_labels.py`](../../.github/scripts/sync_labels.py)
applies it (dry run by default).

| Namespace    | Labels                                                                                                                                 | Replaces                                                   |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------- |
| `type:`      | `bug`, `new-feature`, `improvement`, `documentation`, `ci`, `testing`, `build`, `task`, `demo`; `github-actions`, `cargo` (dependabot) | `bug`, `enhancement`, `documentation`, `demo`, `work-item` |
| `component:` | `island`, `hpc`, `msgs`, `platform`, `bringup`, `contracts`                                                                            | `island-node`, `hpc-node`                                  |
| `status:`    | `needs-design`, `ready`, `blocked`, `info-needed`, `help-wanted`, `stale`, `wont-fix`                                                  | `help wanted`, `wontfix`, `question`                       |
| `partition:` | `p0` ... `p4`, `p3s` (the plan's partitions)                                                                                           | `P0` ... `P4`, `P3s`                                       |
| `wp:`        | `0` ... `10`                                                                                                                           | `wp0` ... `wp10`                                           |
| `platform:`  | `s32z2`, `x5h`, `host`; `simulator:carla`                                                                                              | `s32z2`                                                    |
| `tag:`       | `tracking`, `imported-node`, `good-first-issue`, `bot`, `sync-files`, `update-codeowners-from-packages`, `no-auto-merge`               | `tracking`, `imported-node`, `good first issue`            |
| `run:`       | `integration`                                                                                                                          | new                                                        |

`P0`-`P4` and `P3s` name the plan's partitions ("Partition P0: safety
core"), not priorities, so they move to `partition:` rather than Autoware's
`priority:`. `duplicate` and `invalid` stay as GitHub defines them.

## 5. Open participation

```text
Discussion      -> issue               -> design decided, -> status:ready -> /claim     -> pull request
(idea, design)     status:needs-design    tests written                     (assigned)     Closes #N
```

- **Ready means checkable.** A maintainer sets `status:ready` only when the
  issue names its acceptance tests and design references. Ready issues are
  the only ones open to claiming, by people and by agents alike.
- **Claims.** A `/claim` comment on a ready, unassigned issue makes a bot
  assign the commenter (`claim.yaml`, on `issue_comment`); `/unclaim`
  releases it. A daily job (`claim-expiry.yaml`) drops a claim that has no
  linked open pull request 14 days after it was made, and comments why. It
  touches only assignments the claim bot made; maintainers' own assignments
  stay. `tag:good-first-issue` and `status:help-wanted` mark entry points.
- **Roles.** Outside participants get Triage (label, assign, close
  duplicates). Write and Maintain stay with the core team; code ownership
  follows `package.xml`.
- **Docs.** `CONTRIBUTING.md` (Autoware's guidelines, with an L4 section): the
  label meanings, the claim flow, the AI lane, the pull request cap.
  `getting-started.md` and `testing.md` exist.

## 6. Large-scale AI contribution

Autoware's AI contribution policy binds every Autoware Foundation repository,
maintainers included. It welcomes agents and protects reviewer attention.
The L4 lane keeps agents productive inside it:

| Policy rule                                     | How the L4 lane meets it                                                                                                                                                                                                                                            |
| :---------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Understand everything; review your own PR first | Agents work locally ("working autonomously in private is fine"). The operator reads the diff and runs a clean-context review agent before the pull request leaves draft                                                                                             |
| Do not let agents post unattended               | Agents push branches. The operator opens the pull request, or reviews the agent's draft and marks it ready. Ready for review is the human attestation, and it is also what arms auto-merge. Comments and issues posted by agents are reviewed per item the same way |
| Disclose; verification statement                | The PR template's "AI usage" section: tool, extent, self-review, tests run. The agent drafts it; the operator edits and owns it                                                                                                                                     |
| Authors are humans; never sign as the agent     | C6: no AI trailers. The operator signs off (`git commit -s`) and is the author; an agent adds the operator's sign-off only on the operator's instruction                                                                                                            |
| Small PRs                                       | One issue per pull request; an issue too large for one reviewable pull request is split into sub-issues first                                                                                                                                                       |
| Do not mass-produce; agree on bulk campaigns    | Cap: at most 5 non-draft pull requests per operator at a time. Bulk work runs as sub-issues of a tracking issue the maintainers agreed on, which is the campaign agreement                                                                                          |
| Maintainable, concise output                    | Review rejects noise. `CLAUDE.md` / `AGENTS.md` tell agents the repository's rules (layout, sources of truth, commit style, minimal text)                                                                                                                           |

What carries over from nano-ros's agent workflow: one claim per issue (no
duplicate work), acceptance tests as the agent's stopping condition, a local
run of the same checks before marking ready, required checks that cover
everything the local gate covers (a pull request lane that skipped builds
once broke nano-ros main), and the merge queue re-testing the merged result.
What does not carry over: agents opening and arming pull requests on their
own, which the policy forbids.

## 7. Pull request flow and auto-arm

```text
branch           -> operator review, -> PR ready      -> auto-arm     -> approval        -> queue: gate     -> squash to main,
(agent or human)    local checks        (title, DCO)     (maintainer)    (outside PRs)      + integration      issue closes
```

### 7.1 Auto-arm workflow

- [`.github/workflows/auto-arm.yaml`](../../.github/workflows/auto-arm.yaml)
  runs on `pull_request_target` (`opened`, `ready_for_review`, `reopened`).
- It arms a pull request that is not a draft, has no `tag:no-auto-merge`, and
  whose author has the `maintain` or `admin` role (collaborator permission
  API, `role_name`). Action: `gh pr merge --auto --squash`.
- It never checks out pull request code, so `pull_request_target` is safe;
  permissions `contents: write`, `pull-requests: write`.
- Draft is the opt-out: open as draft, mark ready when it is reviewed.
  Converting back to draft does not disarm by itself; add `tag:no-auto-merge`
  or run `gh pr merge --disable-auto`.

### 7.2 Rulesets

| Ruleset                                                                          | Rules                                                                                                                                                                 | Bypass                           |
| :------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------- |
| `main` (A, [`main.json`](../../.github/rulesets/main.json))                      | Pull request required; merge queue (`SQUASH`, all-green grouping, up to 5); required check `gate`; linear history; no force push or deletion; review threads resolved | nobody                           |
| `main-review` (B, [`main-review.json`](../../.github/rulesets/main-review.json)) | One approving review from a code owner; stale approvals dismissed on push                                                                                             | Maintain role, for pull requests |

Result: outside pull requests wait for a code owner's approval and merge on
it; maintainer pull requests merge when `gate` and the queue pass.

**To verify before relying on B:** that auto-merge honours a ruleset bypass
when the review rule is unmet. Auto-arm enables auto-merge with the workflow
token, so the actor GitHub checks may be `github-actions[bot]` rather than the
maintainer. Test on a scratch pull request. If the bypass does not apply,
keep ruleset B off, keep 0 required approvals, and let `auto-arm` skip
non-maintainers: a maintainer reviews an outside pull request and arms it by
hand.

**Autoware's default is review for every pull request.** Maintainer
self-merge without review is the bootstrap mode for a two-maintainer project.
Once each component has two code owners, turn the bypass off; auto-arm still
saves the round trip, because an armed pull request merges the moment it is
approved.

## 8. Changes and decisions

| #   | Change                                                                                                                                                                                                                                       | Who                  |
| :-- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------- |
| 1   | This note; conventions sections 4-6, C7 superseded; `CONTRIBUTING.md` L4 section; `CLAUDE.md` / `AGENTS.md`                                                                                                                                  | pull request         |
| 2   | `.github/`: issue forms, `labeler.yaml` + `pr-labeler.yaml`, `stale.yml`, `sync-files.yaml` + workflow, `update-codeowners-from-packages.yaml`, `auto-arm.yaml`, `claim.yaml` + `claim-expiry.yaml`; `integration.yaml` on `run:integration` | pull request         |
| 3   | Remove the issue JSON, its scripts and workflows, `update-progress.yml`, the progress script, the dispatched-gate guard, the README counts; `MILESTONES.md` becomes an index                                                                 | pull request         |
| 4   | Labels: rename and create per section 4.1 (`sync_labels.py --apply`)                                                                                                                                                                         | maintainer           |
| 5   | Tracking issues M0-M7 and areas; attach existing issues as sub-issues; `status:ready` on issues with acceptance tests                                                                                                                        | maintainer, scripted |
| 6   | Settings: auto-merge on, squash only, Discussions on, Actions may create pull requests; rulesets A and B; organization App secrets for sync-files and CODEOWNERS ([repository-setup.md](../ci/repository-setup.md))                          | maintainer           |

| Question                                                                                                          | State                                                                                          |
| :---------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------- |
| Issues on GitHub, tracking issues replace phase documents                                                         | **Decided**                                                                                    |
| Drop the README progress counts                                                                                   | **Decided**                                                                                    |
| Outside pull requests need one code-owner approval; maintainers auto-armed                                        | **Decided**, pending the bypass test (section 7.2)                                             |
| `/claim` bot with 14-day expiry                                                                                   | **Decided**                                                                                    |
| Adopt Autoware's workflow: labels, sync-files, labeler, stale, CODEOWNERS, squash, Discussions, `run:integration` | **Decided**                                                                                    |
| Per-operator cap of 5 ready pull requests                                                                         | **Decided**                                                                                    |
| Agents push branches and the operator opens the pull request, versus agents open drafts the operator marks ready  | Open: confirm the reading of "do not let agents post unattended" with the Autoware maintainers |
| GitHub Project board                                                                                              | Optional                                                                                       |
