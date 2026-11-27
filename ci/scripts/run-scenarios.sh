#!/usr/bin/env bash
# shellcheck disable=SC1091 # lib.sh is sourced by path
# Run a suite (or a single .xosc) of the staged project against the running stack.
#   run-scenarios.sh [suites/smoke.yaml]
# Exit code: swb's (0 passed, 1 failed, 2 error, 3 refused).
set -uo pipefail
source "$(dirname "$0")/lib.sh"
l4si_env

suite="${1:-suites/smoke.yaml}"
[ -d "$L4SI_PROJECT" ] || die 3 "no staged project at $L4SI_PROJECT (run stack-up.sh first)"
cd "$L4SI_PROJECT" || die 3 "cannot enter $L4SI_PROJECT"
[ -e "$suite" ] || die 3 "no $suite in the project"

log "running $suite (the stack has ${L4SI_READY_TIMEOUT}s to become ready)"
ros2 run swb run "$suite" --wait "$L4SI_READY_TIMEOUT" 2>&1 | tee -a "$L4SI_STATE/runs.log"
code=${PIPESTATUS[0]}
log "$suite: exit $code"
exit "$code"
