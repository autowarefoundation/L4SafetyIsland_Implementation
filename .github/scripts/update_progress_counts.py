#!/usr/bin/env python3
"""Refresh the (completed / open) issue counts in README.md's Progress section.

Only the text between the markers

    <!-- progress-counts:start -->  ...  <!-- progress-counts:end -->

is touched. Inside it, every link to an issue list of this repository
(`.../issues?q=...`) gets its count rewritten as ` (closed / open)`, where both
numbers are counted over the link's labels with the open/closed filter removed.
A new row written in the same style is therefore counted without changing this
script. The date in `<!-- progress-counts:date -->YYYY-MM-DD` is set to today
only when a count changed, so an unchanged run commits nothing.

Needs the GitHub CLI (`gh`) with GH_TOKEN set, and GITHUB_REPOSITORY
(`owner/name`), both of which GitHub Actions provides.
"""
import datetime
import json
import os
import re
import subprocess
import sys
import urllib.parse

README = sys.argv[1] if len(sys.argv) > 1 else "README.md"
REPO = os.environ["GITHUB_REPOSITORY"]
START, END = "<!-- progress-counts:start -->", "<!-- progress-counts:end -->"
LINK = re.compile(
    r"(\[[^\]]+\]\(https://github\.com/" + re.escape(REPO) + r"/issues\?q=([^)\s]+)\))"
    r"(?: \(\d+ / \d+\))?"
)
DATE = re.compile(r"(<!-- progress-counts:date -->)\d{4}-\d{2}-\d{2}")

_cache = {}


def count(state, labels):
    key = (state, labels)
    if key not in _cache:
        cmd = ["gh", "issue", "list", "--repo", REPO, "--state", state,
               "--limit", "1000", "--json", "number"]
        for label in labels:
            cmd += ["--label", label]
        out = subprocess.run(cmd, capture_output=True, text=True, check=True).stdout
        _cache[key] = len(json.loads(out))
    return _cache[key]


def labels_of(query):
    terms = urllib.parse.unquote_plus(query).split()
    return tuple(sorted(t[len("label:"):] for t in terms if t.startswith("label:")))


def refresh(section):
    def sub(m):
        labels = labels_of(m.group(2))
        return f"{m.group(1)} ({count('closed', labels)} / {count('open', labels)})"
    counted = LINK.sub(sub, section)
    if counted == section:
        return section          # nothing changed: keep the date, so no commit
    return DATE.sub(lambda m: m.group(1) + datetime.date.today().isoformat(), counted)


def main():
    text = open(README, encoding="utf-8").read()
    try:
        a = text.index(START) + len(START)
        b = text.index(END, a)
    except ValueError:
        sys.exit(f"{README}: progress-counts markers not found")
    new = text[:a] + refresh(text[a:b]) + text[b:]
    if new != text:
        open(README, "w", encoding="utf-8").write(new)
        print("README counts updated")
    else:
        print("README counts unchanged")


if __name__ == "__main__":
    main()
