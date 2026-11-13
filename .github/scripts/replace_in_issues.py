#!/usr/bin/env python3
"""Replace a phrase in the titles and bodies of issues, for example after a platform change.

    python3 .github/scripts/replace_in_issues.py "X-S32Z27X-DC" "S32Z280-400EVB" --keep "evaluated on the X-S32Z27X-DC" --dry-run
    python3 .github/scripts/replace_in_issues.py "X-S32Z27X-DC" "S32Z280-400EVB" --keep "evaluated on the X-S32Z27X-DC"

Every issue (open and closed) is scanned. Lines containing a --keep phrase are left unchanged, so
history can be kept where it is meant as history. --dry-run prints each changed line, old then new,
and writes nothing. Needs the GitHub CLI (`gh`), authenticated with write access.
"""
import argparse
import json
import os
import subprocess
import sys

REPO = os.environ.get("GITHUB_REPOSITORY", "autowarefoundation/L4SafetyIsland_Implementation")


def gh(*args, stdin=None):
    r = subprocess.run(["gh", "api", *args], input=stdin, capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f"gh api {' '.join(args)} failed:\n{r.stderr or r.stdout}")
    return json.loads(r.stdout) if r.stdout.strip() else None


def replace(text, old, new, keep):
    out = []
    for line in (text or "").split("\n"):
        out.append(line if any(k in line for k in keep) else line.replace(old, new))
    return "\n".join(out)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("old")
    ap.add_argument("new")
    ap.add_argument("--keep", action="append", default=[])
    ap.add_argument("--dry-run", action="store_true")
    a = ap.parse_args()
    issues = gh(f"repos/{REPO}/issues?state=all&per_page=100")
    changed = 0
    for i in sorted(issues, key=lambda i: i["number"]):
        if "pull_request" in i:
            continue
        title = replace(i["title"], a.old, a.new, a.keep)
        body = replace(i["body"], a.old, a.new, a.keep)
        if title == i["title"] and body == (i["body"] or ""):
            continue
        changed += 1
        print(f"#{i['number']} {i['title']}")
        for o, n in zip(((i["title"] or "") + "\n" + (i["body"] or "")).split("\n"),
                        (title + "\n" + body).split("\n")):
            if o != n:
                print(f"   - {o.strip()[:150]}\n   + {n.strip()[:150]}")
        if not a.dry_run:
            gh(f"repos/{REPO}/issues/{i['number']}", "-X", "PATCH", "--input", "-",
               stdin=json.dumps({"title": title, "body": body}))
    print(f"{changed} issue(s) {'would change' if a.dry_run else 'changed'}")


if __name__ == "__main__":
    main()
