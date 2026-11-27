#!/usr/bin/env bash
# shellcheck disable=SC1091 # lib.sh is sourced by path
# Tear down what stack-up.sh started: the swb stack's process group, then the island.
# CARLA (its own container) stays up for the next job.
set -uo pipefail
source "$(dirname "$0")/lib.sh"

stop_group "$L4SI_STATE/stack.pid" "the swb stack"
"$(dirname "$0")/island-down.sh" || true
exit 0
