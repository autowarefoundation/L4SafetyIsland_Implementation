# Repository setup for administrators

The workflows are in the repository; the settings below are not, because they
need administrator rights. Apply them once.
[design/work-management.md](../design/work-management.md) explains why each
one exists.

## 1. Settings

- **Settings > General > Pull Requests**: allow squash merging only (turn
  merge commits and rebase merging off), with the pull request title as the
  default commit message; enable "Allow auto-merge" (`auto-arm` and the bot
  pull requests use it) and "Always suggest updating pull request branches".
- **Settings > General > Features**: enable Discussions. Keep the categories
  Q&A (`q-a`) and Ideas (`ideas`), and add Feature requests (slug
  `feature-requests`) and Design: the issue forms' `config.yml` links to the
  first three.
- **Settings > Actions > General**: allow GitHub Actions and the actions of
  `autowarefoundation/*`, `actions/*`, `pre-commit/*`,
  `streetsidesoftware/*`, `amannn/*`, `EmbarkStudios/*`, `tibdex/*` and
  `peter-evans/*`, or all actions. Require approval for first-time
  contributors' workflow runs.
- **Settings > Actions > General > Workflow permissions**: check "Allow
  GitHub Actions to create and approve pull requests". The default token
  permission may stay read-only; each workflow asks for what it needs.
- **Settings > Collaborators and teams**: give outside participants the
  Triage role (label, assign, close duplicates); Write and Maintain stay with
  the core team.

## 2. Protect `main` with the two rulesets

| Ruleset                                                          | Rules                                                                                                                                                                                                                         | Bypass                           |
| :--------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------- |
| A, [`main.json`](../../.github/rulesets/main.json)               | Pull request required, review threads resolved; merge queue: squash, all-green grouping, up to 5 entries, 5 min wait to fill a group, 60 min check timeout; required check `gate`; linear history; no force push; no deletion | none                             |
| B, [`main-review.json`](../../.github/rulesets/main-review.json) | One approving review from a code owner; stale approvals dismissed on push                                                                                                                                                     | Maintain role, for pull requests |

Apply them with the GitHub CLI:

```bash
repo=autowarefoundation/L4SafetyIsland_Implementation
gh api -X POST "repos/$repo/rulesets" --input .github/rulesets/main-review.json
```

and, for `main`, create it (`-X POST` as above) or, if it exists already,
update it in place:

```bash
id=$(gh api "repos/$repo/rulesets" --jq '.[] | select(.name == "main") | .id')
gh api -X PUT "repos/$repo/rulesets/$id" --input .github/rulesets/main.json
```

Or in the web interface: **Settings > Rules > Rulesets > New ruleset > Import
a ruleset**, choose the file, check the rules listed above, and save with
enforcement **Active**.

**The Maintain role's id.** A bypass actor of type `RepositoryRole` names a
base role by number. GitHub's REST reference does not list the numbers; the
GitHub Terraform provider documents them as `maintain` = 2, `write` = 4,
`admin` = 5
([`docs/resources/repository_ruleset.md`](https://github.com/integrations/terraform-provider-github/blob/main/docs/resources/repository_ruleset.md),
section `bypass_actors`). `main-review.json` uses 2. After applying it,
check that the web interface lists "Maintain" in the ruleset's bypass list.

**Before relying on ruleset B**, test that auto-merge honours the bypass:
open a scratch pull request as a maintainer, let `auto-arm` arm it, and check
that the queue takes it without an approval. Auto-arm enables auto-merge
with the workflow token, so GitHub may judge the bypass for
`github-actions[bot]` rather than the author. If the pull request waits for
a review, disable ruleset B (enforcement **Disabled**); maintainers then
review outside pull requests and arm them by hand, and ruleset A still
requires no approval.

## 3. Labels

[`.github/labels.yaml`](../../.github/labels.yaml) lists the labels and the
old names each replaces. Check the plan, then apply it; renames keep the
labels on their issues:

```bash
python3 .github/scripts/sync_labels.py           # dry run
python3 .github/scripts/sync_labels.py --apply
```

It needs `gh` with a token that may edit labels, and PyYAML. Labels the file
does not know (`duplicate`, `invalid`) are left as they are.

## 4. Organization App secrets

`sync-files` and `update-codeowners-from-packages` open their pull requests
with a GitHub App token, as in every Autoware repository, and skip until it
exists. Ask the organization's administrators to install the Autoware bot App
on this repository (contents and pull requests: write) and to make the
organization secrets `APP_ID` and `PRIVATE_KEY` available to it. Their pull
requests carry `tag:bot` and arm auto-merge themselves; ruleset B makes them
wait for a code owner's approval.

The stale bot reads [`.github/stale.yml`](../../.github/stale.yml) where the
organization has the Stale app installed; include this repository there.

## 5. Register the integration runner

The `integration` workflow runs on a self-hosted runner and is skipped until
one exists.

1. **Settings > Actions > Runners > New self-hosted runner**: register the
   machine with the extra label `l4si-integration`. The runner container and
   its setup are in [`ci/`](../../ci/README.md).
2. **Settings > Secrets and variables > Actions > Variables**: add
   `L4SI_INTEGRATION_RUNNER` = `true`.
3. To run it on every merge-queue batch too, add
   `L4SI_INTEGRATION_IN_MERGE_QUEUE` = `true`, then add `integration` to
   ruleset A's required checks once it has been green for a week.

A pull request runs it when a maintainer adds `run:integration` and its
branch is in this repository; fork pull requests run it in the merge queue.

## 6. Code owners and DCO

- [`.github/CODEOWNERS`](../../.github/CODEOWNERS) is generated from the
  `package.xml` maintainers and
  [`.github/CODEOWNERS-manual`](../../.github/CODEOWNERS-manual); edit the
  latter for paths without a package. A maintainer named by email must have
  that address verified on their GitHub account.
- The DCO is checked by the `dco` job of the gate. Installing the
  [DCO GitHub App](https://github.com/apps/dco), as other Autoware
  repositories do, is optional; it duplicates that job.
