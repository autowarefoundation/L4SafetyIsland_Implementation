# Repository setup for administrators

The workflows are in the repository; four settings are not, because they need
administrator rights. Apply them once.

## 1. Protect `main` with the ruleset

[`.github/rulesets/main.json`](../../.github/rulesets/main.json) protects the
default branch:

| Rule                  | Setting                                                                                                                 |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------- |
| Pull request          | required; no approving review required; stale approvals dismissed on push; review threads resolved                      |
| Merge queue           | rebase, all-green grouping, up to 5 entries built and merged together, 5 min wait to fill a group, 60 min check timeout |
| Required status check | `gate` from GitHub Actions                                                                                              |
| History               | linear; no force push; no deletion                                                                                      |
| Bypass                | none, the progress workflow included (below)                                                                            |

Apply it with the GitHub CLI:

```bash
gh api -X POST repos/autowarefoundation/L4SafetyIsland_Implementation/rulesets \
  --input .github/rulesets/main.json
```

To update it later, find its id and send the file again:

```bash
id=$(gh api repos/autowarefoundation/L4SafetyIsland_Implementation/rulesets --jq '.[] | select(.name == "main") | .id')
gh api -X PUT "repos/autowarefoundation/L4SafetyIsland_Implementation/rulesets/$id" \
  --input .github/rulesets/main.json
```

Or in the web interface: **Settings > Rules > Rulesets > New ruleset > Import
a ruleset**, choose `main.json`, check the rules listed above, and save with
enforcement **Active**.

**The progress workflow.** `update-progress.yml` does not push to `main`. When
a count changed, it rebuilds the branch `bot/update-progress-counts` from
`main` with the new README, opens a pull request from it (or updates the open
one), and enables auto-merge on it, so the README goes through the gate and
the merge queue like any other change. The ruleset needs no bypass actor.

A pull request opened or pushed with the workflow token (`GITHUB_TOKEN`)
starts no `pull_request` workflow, so the gate would never report on it. The
workflow token may still start a `workflow_dispatch` run, so the progress
workflow dispatches the gate on the bot branch. That run's `gate` check
attaches to the pull request's head commit and satisfies the required check;
the pull-request-only checks (title, DCO, spelling) are skipped there, as in
a merge group, and the gate accepts them as skipped. The bot's commit carries
no `Signed-off-by:`; the DCO check does not run on it for that reason.

If the merge queue does not start a `merge_group` run for the bot's pull
request, the queue times out after 60 minutes and removes it. Then give the
workflow a token from a GitHub App instead
(`actions/create-github-app-token`, with contents and pull requests write
access): pull requests and pushes made with an App token start workflows as a
person's do, and the dispatch step can stay or go. The App needs no place on
the ruleset's bypass list.

## 2. Allow the merge queue's prerequisites

- **Settings > General > Pull Requests**: allow rebase merging (the queue's
  method), enable "Allow auto-merge" (the progress workflow queues its pull
  request with it) and enable "Always suggest updating pull request
  branches".
- **Settings > Actions > General**: allow GitHub Actions and the actions of
  `autowarefoundation/*`, `actions/*`, `pre-commit/*`,
  `streetsidesoftware/*`, `amannn/*` and `EmbarkStudios/*`, or all actions.
  Require approval for first-time contributors' workflow runs.
- **Settings > Actions > General > Workflow permissions**: check "Allow
  GitHub Actions to create and approve pull requests", or the progress
  workflow cannot open its pull request. The default token permission may
  stay read-only; the workflow asks for what it needs.

## 3. Register the integration runner

The `integration` workflow runs on a self-hosted runner and is skipped until
one exists.

1. **Settings > Actions > Runners > New self-hosted runner**: register the
   machine with the extra label `l4si-integration`. The runner container and
   its setup are in [`ci/`](../../ci/README.md).
2. **Settings > Secrets and variables > Actions > Variables**: add
   `L4SI_INTEGRATION_RUNNER` = `true`.
3. To run it on every merge-queue batch too, add
   `L4SI_INTEGRATION_IN_MERGE_QUEUE` = `true`, then add `integration` to the
   ruleset's required checks once it has been green for a week.

## 4. Code owners and DCO

- Fill in [`.github/CODEOWNERS`](../../.github/CODEOWNERS) when the working
  group assigns owners, then set `require_code_owner_review` to `true` in the
  ruleset.
- The DCO is checked by the `dco` job of the gate. Installing the
  [DCO GitHub App](https://github.com/apps/dco), as other Autoware
  repositories do, is optional; it duplicates that job.
