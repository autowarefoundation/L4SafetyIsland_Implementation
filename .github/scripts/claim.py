"""Claim ready issues with /claim, release them with /unclaim, expire stale claims.

Two modes, each run by a workflow with the GitHub CLI (gh) and GH_TOKEN:

  claim.py comment            claim.yaml, on an issue comment: GITHUB_EVENT_PATH
  claim.py expire [--dry-run] claim-expiry.yaml, daily

A claim assigns the commenter to an issue labelled status:ready that has no
assignee. The daily job drops a claim the bot made when it is CLAIM_DAYS old
and no open pull request is linked to the issue (`Closes #N`, or linked by
hand). Assignments made by people are never touched.
"""

import argparse
import datetime
import json
import os
import subprocess
import sys

READY_LABEL = "status:ready"
CLAIM_DAYS = 14
# The workflow token acts as this bot; REST reports "github-actions[bot]",
# GraphQL "github-actions".
BOT_LOGIN = "github-actions"


# --- decisions (no I/O) -------------------------------------------------------


def parse_command(body):
    """Return "claim", "unclaim" or None for the first non-empty line of a comment."""
    for line in (body or "").splitlines():
        word = line.strip().lower()
        if not word:
            continue
        if word == "/claim":
            return "claim"
        if word == "/unclaim":
            return "unclaim"
        return None
    return None


def decide_claim(state, labels, assignees, commenter):
    """Return (assign, message) for a /claim on an issue."""
    if state != "open":
        return False, f"@{commenter}, this issue is closed."
    if READY_LABEL not in labels:
        return False, (
            f"@{commenter}, only issues labelled `{READY_LABEL}` can be claimed. "
            "Ask on the issue what it still needs."
        )
    if assignees:
        owners = ", ".join(f"@{login}" for login in assignees)
        return False, f"@{commenter}, this issue is already assigned to {owners}."
    return True, (
        f"Assigned to @{commenter}. The claim expires after {CLAIM_DAYS} days unless a pull "
        "request linked to this issue (`Closes #<number>` in its description) is open. "
        "Comment `/unclaim` to release it earlier."
    )


def decide_unclaim(assignees, commenter):
    """Return (unassign, message) for an /unclaim on an issue."""
    if commenter not in assignees:
        return False, f"@{commenter}, you are not assigned to this issue."
    return True, f"@{commenter} released this issue."


def bot_login(login):
    """Return a login without the "[bot]" suffix REST adds to bots."""
    return login.removesuffix("[bot]")


def expired_claims(assignees, assigned_events, has_open_pr, now, days=CLAIM_DAYS):
    """Return the assignees whose claim expired.

    `assigned_events` holds {"assignee", "actor", "created_at"} (datetime) for
    the issue's assignment events. Only the latest event of each assignee
    counts, and only when the bot made it: a person's assignment never
    expires.
    """
    if has_open_pr:
        return []
    latest = {}
    for event in assigned_events:
        login = event["assignee"]
        if login not in latest or event["created_at"] > latest[login]["created_at"]:
            latest[login] = event
    limit = datetime.timedelta(days=days)
    expired = []
    for login in assignees:
        event = latest.get(login)
        if event is None or bot_login(event["actor"]) != BOT_LOGIN:
            continue
        if now - event["created_at"] >= limit:
            expired.append(login)
    return expired


# --- GitHub ------------------------------------------------------------------


def gh_api(path, method="GET", body=None):
    """Call the REST or GraphQL API through gh and return the parsed JSON."""
    cmd = ["gh", "api", "--method", method, path]
    if body is not None:
        cmd += ["--input", "-"]
    out = subprocess.run(
        cmd,
        input=json.dumps(body) if body is not None else None,
        check=True,
        capture_output=True,
        text=True,
    ).stdout
    return json.loads(out) if out.strip() else None


def post_comment(repo, number, message):
    gh_api(f"repos/{repo}/issues/{number}/comments", "POST", {"body": message})


def handle_comment(repo, event):
    """Act on a /claim or /unclaim comment of an issue_comment event."""
    issue = event["issue"]
    comment = event["comment"]
    if "pull_request" in issue or comment["user"]["type"] == "Bot":
        return
    command = parse_command(comment["body"])
    if command is None:
        return
    number = issue["number"]
    commenter = comment["user"]["login"]
    labels = [label["name"] for label in issue["labels"]]
    assignees = [user["login"] for user in issue["assignees"]]
    path = f"repos/{repo}/issues/{number}/assignees"

    if command == "claim":
        assign, message = decide_claim(issue["state"], labels, assignees, commenter)
        if assign:
            result = gh_api(path, "POST", {"assignees": [commenter]})
            # GitHub ignores an assignee it may not assign, without an error.
            if commenter not in [user["login"] for user in result["assignees"]]:
                message = f"@{commenter}, GitHub did not accept you as an assignee of this issue."
    else:
        unassign, message = decide_unclaim(assignees, commenter)
        if unassign:
            gh_api(path, "DELETE", {"assignees": [commenter]})
    post_comment(repo, number, message)


ISSUES_QUERY = """
query($owner: String!, $name: String!, $after: String) {
  repository(owner: $owner, name: $name) {
    issues(states: OPEN, first: 50, after: $after) {
      pageInfo { hasNextPage endCursor }
      nodes {
        number
        assignees(first: 20) { nodes { login } }
        closedByPullRequestsReferences(first: 10, includeClosedPrs: false) {
          nodes { state }
        }
        timelineItems(itemTypes: [ASSIGNED_EVENT], last: 50) {
          nodes {
            ... on AssignedEvent {
              createdAt
              actor { login }
              assignee { ... on User { login } }
            }
          }
        }
      }
    }
  }
}
"""


def open_assigned_issues(repo):
    """Yield the open issues that have an assignee, with their assignment events."""
    owner, name = repo.split("/")
    after = None
    while True:
        body = {"query": ISSUES_QUERY, "variables": {"owner": owner, "name": name, "after": after}}
        page = gh_api("graphql", "POST", body)["data"]["repository"]["issues"]
        for node in page["nodes"]:
            if node["assignees"]["nodes"]:
                yield node
        if not page["pageInfo"]["hasNextPage"]:
            return
        after = page["pageInfo"]["endCursor"]


def parse_time(text):
    return datetime.datetime.fromisoformat(text.replace("Z", "+00:00"))


def handle_expiry(repo, dry_run):
    """Drop the claims that expired, with a comment on each issue."""
    now = datetime.datetime.now(datetime.timezone.utc)
    for issue in open_assigned_issues(repo):
        assignees = [user["login"] for user in issue["assignees"]["nodes"]]
        prs = issue["closedByPullRequestsReferences"]["nodes"]
        has_open_pr = any(pr["state"] == "OPEN" for pr in prs)
        events = [
            {
                "assignee": event["assignee"]["login"],
                "actor": event["actor"]["login"] if event["actor"] else "",
                "created_at": parse_time(event["createdAt"]),
            }
            for event in issue["timelineItems"]["nodes"]
            if event.get("assignee") and event["assignee"].get("login")
        ]
        for login in expired_claims(assignees, events, has_open_pr, now):
            number = issue["number"]
            print(f"#{number}: claim of @{login} expired" + (" (dry run)" if dry_run else ""))
            if dry_run:
                continue
            gh_api(f"repos/{repo}/issues/{number}/assignees", "DELETE", {"assignees": [login]})
            post_comment(
                repo,
                number,
                f"@{login}, this claim expired: {CLAIM_DAYS} days passed without an open pull "
                "request linked to this issue. Comment `/claim` to take it again.",
            )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    sub = parser.add_subparsers(dest="mode", required=True)
    sub.add_parser("comment", help="act on the issue comment in GITHUB_EVENT_PATH")
    expire = sub.add_parser("expire", help="drop expired claims")
    expire.add_argument("--dry-run", action="store_true", help="only print what would change")
    args = parser.parse_args()

    repo = os.environ["GITHUB_REPOSITORY"]
    if args.mode == "comment":
        with open(os.environ["GITHUB_EVENT_PATH"], encoding="utf-8") as f:
            handle_comment(repo, json.load(f))
    else:
        handle_expiry(repo, args.dry_run)
    return 0


if __name__ == "__main__":
    sys.exit(main())
