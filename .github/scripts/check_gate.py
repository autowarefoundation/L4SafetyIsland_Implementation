"""Decide the merge gate from the results of the jobs it needs.

Reads the `needs` context (JSON) and the event name from the environment.
Every job must succeed, except the pull-request-only checks, which are
skipped by design outside a pull request. A skipped job counts as a failure
anywhere else, so a broken `if:` cannot turn the gate green.
"""

import json
import os
import sys

PULL_REQUEST_ONLY = {"semantic-pull-request", "dco", "spell-check-differential"}


def failures(needs, event):
    """Return the jobs whose result does not let the gate pass."""
    bad = []
    for job, info in sorted(needs.items()):
        result = info.get("result")
        if result == "success":
            continue
        if result == "skipped" and event != "pull_request" and job in PULL_REQUEST_ONLY:
            continue
        bad.append(f"{job}: {result}")
    return bad


def main():
    needs = json.loads(os.environ["NEEDS"])
    event = os.environ["EVENT"]
    bad = failures(needs, event)
    for job, info in sorted(needs.items()):
        print(f"{job}: {info.get('result')}")
    if bad:
        print("gate: FAILED -- " + ", ".join(bad))
        return 1
    print("gate: passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
