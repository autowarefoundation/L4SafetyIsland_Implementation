#!/usr/bin/env bash
# Check every node contract of the repository with play_launch.
#
# A contract is <stem>.contract.yaml beside its launch file <stem>.launch.xml
# in a package under src/. The message packages must be on the ament index
# (build them with colcon and source the overlay first), because play_launch
# resolves every message field a contract names through it.
#
# Usage: scripts/check_contracts.sh   (PLAY_LAUNCH=<path> to use another binary)
set -euo pipefail
cd "$(dirname "$0")/.."
pl="${PLAY_LAUNCH:-play_launch}"

echo "play_launch: $("$pl" --version)"
fail=0
n=0
while IFS= read -r contract; do
    launch="${contract%.contract.yaml}.launch.xml"
    n=$((n + 1))
    if [ ! -f "$launch" ]; then
        echo "FAIL $contract: no launch file $launch beside it"
        fail=1
        continue
    fi
    echo "== $launch"
    if ! out="$("$pl" check --locked --strict "$launch" 2>&1)"; then
        printf '%s\n' "$out"
        echo "FAIL $contract"
        fail=1
        continue
    fi
    # A field the checker cannot resolve is only a warning; treat it as a failure.
    if printf '%s\n' "$out" | grep -q -- '-field-unknown'; then
        printf '%s\n' "$out"
        echo "FAIL $contract: a message field is not on the ament index"
        fail=1
        continue
    fi
    printf '%s\n' "$out" | tail -2
done < <(find src -name '*.contract.yaml' | sort)

echo "$n contract(s) checked"
exit "$fail"
