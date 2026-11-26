"""Bring the repository's labels in line with .github/labels.yaml.

A label that exists is updated (color, description). A label that does not
exist yet but has an old name listed under `replaces` is renamed from it
(`gh label edit --name`), so the issues and pull requests keep it. Any other
label is created. Labels missing from the file are reported, never deleted.

Dry run by default; pass --apply to change the repository. Needs the GitHub
CLI (gh), logged in with a token that may edit the repository's labels, and
PyYAML.
"""

import argparse
import json
from pathlib import Path
import subprocess
import sys

import yaml

LABELS_FILE = Path(__file__).resolve().parent.parent / "labels.yaml"


def load_defined(path):
    """Return the labels of labels.yaml as a list of dicts."""
    with open(path, encoding="utf-8") as f:
        data = yaml.safe_load(f)
    labels = data["labels"]
    names = [label["name"] for label in labels]
    duplicates = sorted({name for name in names if names.count(name) > 1})
    if duplicates:
        raise ValueError(f"defined twice in {path}: {', '.join(duplicates)}")
    return labels


def plan(defined, existing):
    """Return the steps that make `existing` match `defined`.

    `existing` maps a label name to {"color", "description"}. Each step is a
    tuple: ("create", label), ("rename", old_name, label),
    ("update", label), or ("conflict", label, reason), which needs a person.
    Labels already in line produce no step.
    """
    steps = []
    for label in defined:
        name = label["name"]
        old_present = [old for old in label.get("replaces", []) if old in existing]
        if name in existing:
            current = existing[name]
            if (
                current["color"].lower() != label["color"].lower()
                or current["description"] != label["description"]
            ):
                steps.append(("update", label))
            if old_present:
                reason = (
                    f"both {name!r} and {', '.join(map(repr, old_present))} exist; merge by hand"
                )
                steps.append(("conflict", label, reason))
            continue
        if len(old_present) > 1:
            reason = f"several old labels exist: {', '.join(map(repr, old_present))}"
            steps.append(("conflict", label, reason))
            continue
        if old_present:
            steps.append(("rename", old_present[0], label))
        else:
            steps.append(("create", label))
    return steps


def leftovers(defined, existing):
    """Return the existing labels that labels.yaml neither defines nor replaces."""
    known = set()
    for label in defined:
        known.add(label["name"])
        known.update(label.get("replaces", []))
    return sorted(name for name in existing if name not in known)


def describe(step):
    """Return one line that says what a step does."""
    kind = step[0]
    if kind == "rename":
        return f"rename {step[1]!r} -> {step[2]['name']!r}"
    if kind == "conflict":
        return f"CONFLICT {step[1]['name']!r}: {step[2]}"
    return f"{kind} {step[1]['name']!r}"


def gh_command(step, repo):
    """Return the gh command line that applies a step (not for conflicts)."""
    kind = step[0]
    label = step[-1]
    style = ["--color", label["color"], "--description", label["description"]]
    if kind == "create":
        return ["gh", "label", "create", label["name"], "--repo", repo, *style]
    if kind == "rename":
        return ["gh", "label", "edit", step[1], "--repo", repo, "--name", label["name"], *style]
    return ["gh", "label", "edit", label["name"], "--repo", repo, *style]


def read_existing(repo):
    """Return the repository's labels as {name: {"color", "description"}}."""
    out = subprocess.run(
        [
            "gh",
            "label",
            "list",
            "--repo",
            repo,
            "--limit",
            "1000",
            "--json",
            "name,color,description",
        ],
        check=True,
        capture_output=True,
        text=True,
    ).stdout
    return {
        item["name"]: {"color": item["color"], "description": item["description"] or ""}
        for item in json.loads(out)
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--repo",
        default="autowarefoundation/L4SafetyIsland_Implementation",
        help="owner/name of the repository (default: %(default)s)",
    )
    parser.add_argument("--apply", action="store_true", help="change the labels (default: dry run)")
    args = parser.parse_args()

    defined = load_defined(LABELS_FILE)
    existing = read_existing(args.repo)
    steps = plan(defined, existing)

    for step in steps:
        print(describe(step))
    for name in leftovers(defined, existing):
        print(f"not in labels.yaml, left as is: {name!r}")
    if not steps:
        print("Labels are in line with labels.yaml.")

    conflicts = [step for step in steps if step[0] == "conflict"]
    if not args.apply:
        print("Dry run; pass --apply to change the labels.")
        return 1 if conflicts else 0

    for step in steps:
        if step[0] != "conflict":
            subprocess.run(gh_command(step, args.repo), check=True)
    return 1 if conflicts else 0


if __name__ == "__main__":
    sys.exit(main())
