#!/usr/bin/env bash
# cspell:ignore pgid setsid
# shellcheck disable=SC1091 # lib.sh is sourced by path
# Bring up the integration stack: CARLA's address checked, the swb project staged and its
# assets fetched, the safety island (L4SI_WITH_ISLAND=1), then swb's stack in a process
# group of its own. Returns once the stack is started; `swb run --wait` waits for ready.
set -euo pipefail
source "$(dirname "$0")/lib.sh"

mkdir -p "$L4SI_WORK" "$L4SI_STATE" "$SWB_CONFIG_HOME"
l4si_env

# swb user settings for this run: where CARLA is. CARLA is never started or stopped here.
cat >"$SWB_CONFIG_HOME/host.yaml" <<EOF
carla: {host: $CARLA_HOST, port: $CARLA_PORT, start: external}
EOF

log "waiting for CARLA at $CARLA_HOST:$CARLA_PORT (up to ${CARLA_READY_TIMEOUT}s)"
wait_tcp "$CARLA_HOST" "$CARLA_PORT" "$CARLA_READY_TIMEOUT" ||
    die 3 "CARLA does not answer on $CARLA_HOST:$CARLA_PORT (is the carla service up?)"

# The project: a fresh copy each run, so a stale lock or link never leaks between jobs.
rm -rf "$L4SI_PROJECT"
mkdir -p "$L4SI_PROJECT"
cp -a "$L4SI_PROJECT_SRC/." "$L4SI_PROJECT/"
cd "$L4SI_PROJECT"
log "fetching assets"
ros2 run swb assets fetch || die 3 "assets fetch failed"
ros2 run swb project check || die 3 "project check failed"

if [ "$L4SI_WITH_ISLAND" = 1 ]; then
    "$(dirname "$0")/island-up.sh" || die 3 "the safety island did not start"
fi

log "starting the swb stack (map $L4SI_MAP, web UI $L4SI_WEB_ADDR)"
setsid play_launch launch --web-addr "$L4SI_WEB_ADDR" \
    swb_launch swb.launch.xml "map:=$L4SI_MAP" \
    >"$L4SI_STATE/stack.log" 2>&1 </dev/null &
echo $! >"$L4SI_STATE/stack.pid"
sleep 5
kill -0 "$(cat "$L4SI_STATE/stack.pid")" 2>/dev/null ||
    {
        tail -n 50 "$L4SI_STATE/stack.log" >&2
        die 2 "the stack exited at start"
    }
log "stack started (pgid $(cat "$L4SI_STATE/stack.pid")); log $L4SI_STATE/stack.log"
