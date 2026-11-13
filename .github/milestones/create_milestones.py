#!/usr/bin/env python3
"""Create, rename or update the GitHub milestones listed in milestones.json, attach
issues, apply the label moves listed under "relabel", and open the issues listed in
issues.json.

Idempotent: a milestone is matched by its title or any of its "previous_titles";
a match is renamed and its description refreshed, a missing one is created. Each
listed issue is then set to its milestone. Every description ends with a link to
docs/MILESTONES.md. No due dates are set. A label that does not exist yet is
created with the colour of the label it replaces.

issues.json: each issue carries a stable "key", written into its body as a hidden
marker (<!-- issue-key: KEY -->). An issue whose marker already exists is never
re-created and its title, body and labels are left alone (people edit issues);
only its milestone is kept in step. Labels listed under "labels" are created, or
their colour and description corrected.

Needs the GitHub CLI (`gh`), authenticated with write access to the repository.

    python3 .github/milestones/create_milestones.py            # apply
    python3 .github/milestones/create_milestones.py --dry-run  # show what would change
"""
import json
import os
import re
import subprocess
import sys
import time

REPO = os.environ.get("GITHUB_REPOSITORY", "autowarefoundation/L4SafetyIsland_Implementation")
HERE = os.path.dirname(os.path.abspath(__file__))
DRY = "--dry-run" in sys.argv


def gh(*args, fields=None, ok404=False):
    cmd = ["gh", "api", *args]
    for k, v in (fields or {}).items():
        for item in (v if isinstance(v, list) else [v]):
            key = f"{k}[]" if isinstance(v, list) else k
            cmd += ["-F" if isinstance(item, int) else "-f", f"{key}={item}"]
    r = subprocess.run(cmd, capture_output=True, text=True)
    if r.returncode != 0:
        if ok404 and "404" in (r.stderr + r.stdout):
            return None
        sys.exit(f"gh api {' '.join(args)} failed:\n{r.stderr or r.stdout}")
    return json.loads(r.stdout) if r.stdout.strip() else None


def milestones(spec):
    link = f"https://github.com/{REPO}/blob/main/{spec['doc']}"
    existing = {m["title"]: m for m in gh(f"repos/{REPO}/milestones?state=all&per_page=100")}
    for m in spec["milestones"]:
        body = f"{m['description']}\n\nDetails: {link}"
        names = [m["title"], *m.get("previous_titles", [])]
        cur = next((existing[n] for n in names if n in existing), None)
        if cur is None:
            print(f"create  {m['title']}")
            if not DRY:
                cur = gh(f"repos/{REPO}/milestones", "-X", "POST",
                         fields={"title": m["title"], "description": body})
        elif cur["title"] != m["title"] or cur.get("description") != body:
            what = f"rename  {cur['title']} -> " if cur["title"] != m["title"] else "update  "
            print(f"{what}{m['title']}")
            if not DRY:
                gh(f"repos/{REPO}/milestones/{cur['number']}", "-X", "PATCH",
                   fields={"title": m["title"], "description": body})
        else:
            print(f"ok      {m['title']}")
        for n in m["issues"]:
            print(f"        #{n} -> {m['title']}")
            if not DRY and cur is not None:
                gh(f"repos/{REPO}/issues/{n}", "-X", "PATCH",
                   fields={"milestone": cur["number"]})


def relabel(spec):
    for r in spec.get("relabel", []):
        n = r["issue"]
        have = {l["name"] for l in gh(f"repos/{REPO}/issues/{n}/labels")}
        for name in r.get("add", []):
            if name in have:
                continue
            print(f"label   #{n} + {name}")
            if DRY:
                continue
            if gh(f"repos/{REPO}/labels/{name}", ok404=True) is None:
                src = next((gh(f"repos/{REPO}/labels/{o}", ok404=True)
                            for o in r.get("remove", [])), None)
                fields = {"name": name}
                if src:
                    fields["color"] = src["color"]
                    fields["description"] = src.get("description") or ""
                gh(f"repos/{REPO}/labels", "-X", "POST", fields=fields)
            gh(f"repos/{REPO}/issues/{n}/labels", "-X", "POST", fields={"labels": [name]})
        for name in r.get("remove", []):
            if name not in have:
                continue
            print(f"label   #{n} - {name}")
            if not DRY:
                gh(f"repos/{REPO}/issues/{n}/labels/{name}", "-X", "DELETE")


MARKER = re.compile(r"<!-- issue-key: (\S+) -->")


def all_issues():
    out, page = [], 1
    while True:
        batch = gh(f"repos/{REPO}/issues?state=all&per_page=100&page={page}")
        out += [i for i in batch if "pull_request" not in i]
        if len(batch) < 100:
            return out
        page += 1


def ensure_labels(labels):
    have = {l["name"]: l for l in gh(f"repos/{REPO}/labels?per_page=100")}
    for name, want in labels.items():
        cur = have.get(name)
        if cur is None:
            print(f"label   create {name}")
            if not DRY:
                gh(f"repos/{REPO}/labels", "-X", "POST",
                   fields={"name": name, "color": want["color"], "description": want["description"]})
        elif cur["color"] != want["color"] or (cur.get("description") or "") != want["description"]:
            print(f"label   update {name}")
            if not DRY:
                gh(f"repos/{REPO}/labels/{name}", "-X", "PATCH",
                   fields={"color": want["color"], "description": want["description"]})


def issues(path):
    if not os.path.exists(path):
        return
    spec = json.load(open(path, encoding="utf-8"))
    ensure_labels(spec.get("labels", {}))
    ms = {m["title"].split(" ")[0]: m for m in gh(f"repos/{REPO}/milestones?state=all&per_page=100")}
    by_key = {}
    for i in all_issues():
        m = MARKER.search(i.get("body") or "")
        if m:
            by_key[m.group(1)] = i
    created = 0
    for s in spec["issues"]:
        milestone = ms.get(s["milestone"])
        if milestone is None and not DRY:
            sys.exit(f"milestone {s['milestone']} not found for issue {s['key']}")
        cur = by_key.get(s["key"])
        if cur is None:
            print(f"issue   create [{s['milestone']}] {s['title']}")
            created += 1
            if not DRY:
                body = s["body"].rstrip() + f"\n\n<!-- issue-key: {s['key']} -->\n"
                payload = {"title": s["title"], "body": body, "labels": s["labels"],
                           "milestone": milestone["number"]}
                r = subprocess.run(["gh", "api", f"repos/{REPO}/issues", "-X", "POST", "--input", "-"],
                                   input=json.dumps(payload), capture_output=True, text=True)
                if r.returncode != 0:
                    sys.exit(f"creating {s['key']} failed:\n{r.stderr or r.stdout}")
                time.sleep(2)   # stay under GitHub's content-creation rate limit
        elif milestone and (cur.get("milestone") or {}).get("number") != milestone["number"]:
            print(f"issue   #{cur['number']} -> {milestone['title']}")
            if not DRY:
                gh(f"repos/{REPO}/issues/{cur['number']}", "-X", "PATCH",
                   fields={"milestone": milestone["number"]})
    print(f"{created} issue(s) {'would be' if DRY else ''} created".replace("  ", " "))


def main():
    spec = json.load(open(os.path.join(HERE, "milestones.json"), encoding="utf-8"))
    milestones(spec)
    relabel(spec)
    issues(os.path.join(HERE, "issues.json"))


if __name__ == "__main__":
    main()
