#!/usr/bin/env bash
# shellcheck disable=SC1091 # lib.sh is sourced by path
# Stop the island island-up.sh started, if any.
set -uo pipefail
source "$(dirname "$0")/lib.sh"
stop_group "$L4SI_STATE/island.pid" "the safety island"
exit 0
