#!/usr/bin/env python3
"""Create or update the GitHub milestones listed in milestones.json and attach issues.

Idempotent: a milestone is matched by title; an existing one gets its description
refreshed, a missing one is created. Each listed issue is then set to its milestone.
Every description ends with a link to docs/MILESTONES.md. No due dates are set.

Needs the GitHub CLI (`gh`), authenticated with write access to the repository.

    python3 .github/milestones/create_milestones.py            # apply
    python3 .github/milestones/create_milestones.py --dry-run  # show what would change
"""
import json
import os
import subprocess
import sys

REPO = os.environ.get("GITHUB_REPOSITORY", "autowarefoundation/L4SafetyIsland_Implementation")
HERE = os.path.dirname(os.path.abspath(__file__))
DRY = "--dry-run" in sys.argv


def gh(*args, fields=None):
    cmd = ["gh", "api", *args]
    for k, v in (fields or {}).items():
        cmd += ["-F" if isinstance(v, int) else "-f", f"{k}={v}"]
    out = subprocess.run(cmd, capture_output=True, text=True, check=True).stdout
    return json.loads(out) if out.strip() else None


def main():
    spec = json.load(open(os.path.join(HERE, "milestones.json"), encoding="utf-8"))
    link = f"https://github.com/{REPO}/blob/main/{spec['doc']}"
    existing = {m["title"]: m for m in
                gh(f"repos/{REPO}/milestones?state=all&per_page=100")}
    for m in spec["milestones"]:
        body = f"{m['description']}\n\nDetails: {link}"
        cur = existing.get(m["title"])
        if cur is None:
            print(f"create  {m['title']}")
            if not DRY:
                cur = gh(f"repos/{REPO}/milestones", "-X", "POST",
                         fields={"title": m["title"], "description": body})
        elif cur.get("description") != body:
            print(f"update  {m['title']}")
            if not DRY:
                gh(f"repos/{REPO}/milestones/{cur['number']}", "-X", "PATCH",
                   fields={"description": body})
        else:
            print(f"ok      {m['title']}")
        for n in m["issues"]:
            print(f"        #{n} -> {m['title']}")
            if not DRY and cur is not None:
                gh(f"repos/{REPO}/issues/{n}", "-X", "PATCH",
                   fields={"milestone": cur["number"]})


if __name__ == "__main__":
    main()
