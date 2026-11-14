#!/usr/bin/env bash
# cspell:ignore rosbag
# shellcheck disable=SC1091 # lib.sh is sourced by path
# Gather what the run left into $L4SI_ARTIFACTS: the swb summary (summary.csv, merged
# junit.xml), every run directory (result.json, timeline, logs, rosbag where kept), the
# stack and island logs, and the versions that ran. Never fails the job by itself.
set -uo pipefail
source "$(dirname "$0")/lib.sh"
l4si_env

mkdir -p "$L4SI_ARTIFACTS"
cp /etc/l4si/versions "$L4SI_ARTIFACTS/versions.txt" 2>/dev/null || true
cp "$L4SI_STATE"/*.log "$L4SI_ARTIFACTS/" 2>/dev/null || true

if [ -d "$L4SI_PROJECT" ]; then
    cd "$L4SI_PROJECT" || exit 0
    cp swb.lock "$L4SI_ARTIFACTS/" 2>/dev/null || true
    ros2 run swb runs summary >"$L4SI_ARTIFACTS/summary.txt" 2>&1 || true
    ros2 run swb runs list >"$L4SI_ARTIFACTS/runs.txt" 2>&1 || true
    # runs/ links into the log root; copy the targets (-L), suites' tables beside them.
    if [ -e runs ]; then
        cp -rL runs "$L4SI_ARTIFACTS/runs" 2>/dev/null || true
    fi
    logs_root="$(readlink -f runs 2>/dev/null)"
    if [ -n "$logs_root" ] && [ -d "$logs_root/../suites" ]; then
        cp -rL "$logs_root/../suites" "$L4SI_ARTIFACTS/suites" 2>/dev/null || true
    fi
    if [ -n "$logs_root" ] && [ -d "$logs_root/../stacks" ]; then
        # The stack's own record (stack.json, events) without its full process logs.
        mkdir -p "$L4SI_ARTIFACTS/stacks"
        for s in "$logs_root"/../stacks/*/; do
            d="$L4SI_ARTIFACTS/stacks/$(basename "$s")"
            mkdir -p "$d"
            cp "$s"/stack.json "$s"/events.jsonl "$d/" 2>/dev/null || true
        done
    fi
fi
du -sh "$L4SI_ARTIFACTS" 2>/dev/null | sed 's/^/[l4si] artifacts: /' >&2
exit 0
