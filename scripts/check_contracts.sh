#!/usr/bin/env bash
# Check every node contract of the repository with play_launch.
#
# A contract is <stem>.contract.yaml beside its launch file <stem>.launch.xml
# in a package under src/. A launch file that another contracted launch file
# includes (`<include file="$(dirname)/...">`) is checked as part of that one,
# where its contract is merged with the others, and not on its own.
# The message packages must be on the ament index (build them with colcon and
# source the overlay first), because play_launch resolves every message field
# a contract names through it.
#
# A negative case is a line of <stem>.expect beside the launch file:
#   <rule> [NAME:=VALUE ...]
# The check with those launch arguments must fail with exactly that rule.
#
# Usage: scripts/check_contracts.sh   (PLAY_LAUNCH=<path> to use another binary)
set -euo pipefail
cd "$(dirname "$0")/.."
pl="${PLAY_LAUNCH:-play_launch}"

echo "play_launch: $("$pl" --version)"

mapfile -t contracts < <(find src -name '*.contract.yaml' | sort)
declare -A included=()
for contract in "${contracts[@]}"; do
    launch="${contract%.contract.yaml}.launch.xml"
    [ -f "$launch" ] || continue
    dir="$(dirname "$launch")"
    # shellcheck disable=SC2016 # $(dirname) is the launch substitution, not the shell's
    while IFS= read -r rel; do
        included["$(realpath -m "$dir/$rel")"]=1
    done < <(grep -o 'file="$(dirname)/[^"]*"' "$launch" | sed 's|^file="$(dirname)/||; s|"$||')
done

fail=0
n=0
for contract in "${contracts[@]}"; do
    launch="${contract%.contract.yaml}.launch.xml"
    if [ ! -f "$launch" ]; then
        echo "FAIL $contract: no launch file $launch beside it"
        fail=1
        continue
    fi
    if [ -n "${included[$(realpath "$launch")]:-}" ]; then
        continue
    fi
    n=$((n + 1))
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

    expect="${contract%.contract.yaml}.expect"
    [ -f "$expect" ] || continue
    while read -r rule args; do
        case "$rule" in '' | '#'*) continue ;; esac
        # shellcheck disable=SC2086 # the launch arguments are words
        if out="$("$pl" check --locked --strict "$launch" $args --expect "$rule" 2>&1)"; then
            echo "ok   expected $rule with $args"
        else
            printf '%s\n' "$out"
            echo "FAIL $expect: expected $rule with $args"
            fail=1
        fi
    done <"$expect"
done

echo "$n contract tree(s) checked"
exit "$fail"
