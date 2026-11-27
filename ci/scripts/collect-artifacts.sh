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
    # swb's log directory of this project: <logs>/<project name>/{runs,suites,stacks}.
    logs_dir="${L4SI_LOGS:-/var/lib/l4si/logs}/$(basename "$L4SI_PROJECT")"
    # A suite's table and merged JUnit (its run links are the runs/ copied above).
    for t in "$logs_dir"/suites/*/summary.csv "$logs_dir"/suites/*/junit.xml; do
        [ -f "$t" ] || continue
        d="$L4SI_ARTIFACTS/suites/$(basename "$(dirname "$t")")"
        mkdir -p "$d" && cp "$t" "$d/"
    done
    # The stack's record and every process's log: what says why a stack never got ready.
    if [ -d "$logs_dir/stacks" ]; then
        cp -r "$logs_dir/stacks" "$L4SI_ARTIFACTS/stacks" 2>/dev/null || true
    fi
fi
du -sh "$L4SI_ARTIFACTS" 2>/dev/null | sed 's/^/[l4si] artifacts: /' >&2
exit 0
