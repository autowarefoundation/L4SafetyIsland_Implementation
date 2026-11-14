"""Check that every commit in BASE..HEAD is signed off by its author (DCO).

Usage: check_dco.py BASE HEAD

A commit passes when one of its `Signed-off-by:` trailers names the commit
author's e-mail address. Merge commits are skipped, as the DCO app does.
"""

import re
import subprocess
import sys

SIGN_OFF = re.compile(r"^Signed-off-by:\s*(?P<name>.*?)\s*<(?P<email>[^>]+)>\s*$", re.MULTILINE)


def git(*args):
    return subprocess.run(["git", *args], check=True, capture_output=True, text=True).stdout


def unsigned_commits(base, head):
    """Return (sha, subject, author) for each commit lacking its author's sign-off."""
    shas = git("rev-list", "--no-merges", f"{base}..{head}").split()
    missing = []
    for sha in shas:
        email, subject = git("show", "-s", "--format=%ae%n%s", sha).split("\n", 1)
        body = git("show", "-s", "--format=%B", sha)
        signers = {m.group("email").lower() for m in SIGN_OFF.finditer(body)}
        if email.lower() not in signers:
            missing.append((sha, subject.strip(), email))
    return missing


def main(argv):
    if len(argv) != 3:
        print(__doc__)
        return 2
    missing = unsigned_commits(argv[1], argv[2])
    for sha, subject, email in missing:
        print(f"::error::{sha[:12]} {subject!r}: no 'Signed-off-by:' line for {email}")
    if missing:
        print("Sign off each commit with `git commit -s`; see CONTRIBUTING.md (DCO).")
        return 1
    print("DCO: every commit is signed off by its author.")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
