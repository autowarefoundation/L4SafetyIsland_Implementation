#!/usr/bin/env bash
# shellcheck disable=SC1091 # lib.sh is sourced by path
# One integration run, end to end: stack up, suite, artifacts, stack down.
#   integration.sh [suite]          default: suites/smoke.yaml, or suites/island.yaml
#                                   when L4SI_WITH_ISLAND=1
# The exit code is the verdict: 0 passed, 1 a scenario failed, 2 the stack could not
# judge a run, 3 refused. The same script runs in CI and on a workstation.
set -uo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
source "$here/lib.sh"

if [ "$L4SI_WITH_ISLAND" = 1 ]; then default=suites/island.yaml; else default=suites/smoke.yaml; fi
suite="${1:-$default}"

# shellcheck disable=SC2329 # called by the EXIT trap
cleanup() {
    "$here/collect-artifacts.sh"
    "$here/stack-down.sh"
}
trap cleanup EXIT
trap 'exit 130' INT TERM

"$here/stack-up.sh" || exit $?
"$here/run-scenarios.sh" "$suite"
code=$?
# L4SI_ISLAND_CHECK: the island's own pass checks, run after the suite while the island
# still runs. It can only make a passed run fail (exit 1); its log is an artifact.
if [ "$L4SI_WITH_ISLAND" = 1 ] && [ -n "${L4SI_ISLAND_CHECK:-}" ]; then
    log "island checks: $L4SI_ISLAND_CHECK"
    if ! L4SI_STATE="$L4SI_STATE" bash -c "$L4SI_ISLAND_CHECK" 2>&1 | tee "$L4SI_STATE/island-check.log"; then
        log "island checks failed"
        [ "$code" = 0 ] && code=1
    fi
fi
case $code in
0) log "verdict: passed" ;;
1) log "verdict: failed" ;;
2) log "verdict: error (the stack could not judge a run)" ;;
*) log "verdict: refused ($code)" ;;
esac
exit "$code"
