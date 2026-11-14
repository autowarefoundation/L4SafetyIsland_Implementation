# Repository setup for administrators

The workflows are in the repository; four settings are not, because they need
administrator rights. Apply them once.

## 1. Protect `main` with the ruleset

[`.github/rulesets/main.json`](../../.github/rulesets/main.json) protects the
default branch:

| Rule                  | Setting                                                                                                                 |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------- |
| Pull request          | required; 1 approving review; stale approvals dismissed on push; review threads resolved                                |
| Merge queue           | rebase, all-green grouping, up to 5 entries built and merged together, 5 min wait to fill a group, 60 min check timeout |
| Required status check | `gate` from GitHub Actions                                                                                              |
| History               | linear; no force push; no deletion                                                                                      |
| Bypass                | none; see the progress workflow below                                                                                   |

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

**The progress workflow.** `update-progress.yml` pushes its README commit to
`main` with the workflow token, which the ruleset blocks. GitHub does not
accept GitHub Actions as a bypass actor ("Actor GitHub Actions integration
must be part of the ruleset source or owner organization"). Before applying
the ruleset, either:

- create a GitHub App with contents write access, install it on the
  repository, add it to the ruleset's `bypass_actors`
  (`{"actor_id": <app id>, "actor_type": "Integration", "bypass_mode": "always"}`)
  and let the workflow push with a token from
  `actions/create-github-app-token`, as Autoware's own workflows do; or
- change the workflow to open a pull request with the new counts.

## 2. Allow the merge queue's prerequisites

- **Settings > General > Pull Requests**: allow rebase merging (the queue's
  method) and enable "Always suggest updating pull request branches".
- **Settings > Actions > General**: allow GitHub Actions and the actions of
  `autowarefoundation/*`, `actions/*`, `pre-commit/*`,
  `streetsidesoftware/*`, `amannn/*` and `EmbarkStudios/*`, or all actions.
  Require approval for first-time contributors' workflow runs.

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
