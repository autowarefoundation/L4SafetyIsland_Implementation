# cspell:ignore pgid pidfile
# Shared by ci/scripts/*.sh (source it; bash). Settings come from the environment:
#
#   L4SI_WORK          work directory of this run (default: $L4SI_ROOT/build/integration)
#   L4SI_ARTIFACTS     where collect-artifacts.sh puts what a run leaves
#                      (default: $L4SI_WORK/artifacts)
#   L4SI_PROJECT_SRC   the swb project to run (default: ci/project)
#   CARLA_HOST/PORT    the CARLA server (default: carla:2000, the compose service)
#   L4SI_READY_TIMEOUT seconds the stack has to become ready (default 1200)
#   L4SI_WITH_ISLAND   1: start the safety island (host build) before the stack
#   L4SI_ENV           the environment script (default: /etc/l4si/env.sh, else ci/runner/env.sh)
#
# Exit codes, the same as `ros2 run swb run`: 0 passed, 1 a scenario failed, 2 the stack
# could not judge a run, 3 refused (bad input, no CARLA, missing asset).
# shellcheck shell=bash disable=SC1090,SC2034 # variables for the scripts that source this

L4SI_ROOT="${L4SI_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
L4SI_WORK="${L4SI_WORK:-$L4SI_ROOT/build/integration}"
L4SI_ARTIFACTS="${L4SI_ARTIFACTS:-$L4SI_WORK/artifacts}"
L4SI_PROJECT_SRC="${L4SI_PROJECT_SRC:-$L4SI_ROOT/ci/project}"
L4SI_PROJECT="$L4SI_WORK/project"
L4SI_STATE="$L4SI_WORK/state"
L4SI_READY_TIMEOUT="${L4SI_READY_TIMEOUT:-1200}"
L4SI_WITH_ISLAND="${L4SI_WITH_ISLAND:-0}"
L4SI_MAP="${L4SI_MAP:-Town01}"
L4SI_WEB_ADDR="${L4SI_WEB_ADDR:-127.0.0.1:8080}"
CARLA_HOST="${CARLA_HOST:-carla}"
CARLA_PORT="${CARLA_PORT:-2000}"
CARLA_READY_TIMEOUT="${CARLA_READY_TIMEOUT:-300}"

# swb reads its user settings from here; one directory per work tree keeps runs apart.
export SWB_CONFIG_HOME="$L4SI_STATE/swb-config"

log() { printf '[l4si %s] %s\n' "$(date +%H:%M:%S)" "$*" >&2; }
die() {
    local code="$1"
    shift
    log "error: $*"
    exit "$code"
}

l4si_env() {
    local env="${L4SI_ENV:-}"
    if [ -z "$env" ]; then
        if [ -f /etc/l4si/env.sh ]; then env=/etc/l4si/env.sh; else env="$L4SI_ROOT/ci/runner/env.sh"; fi
    fi
    source "$env"
}

# Wait until host:port accepts a TCP connection; 0 when it does, 1 at the timeout.
wait_tcp() {
    local host="$1" port="$2" timeout="$3" t=0
    while ! (exec 3<>"/dev/tcp/$host/$port") 2>/dev/null; do
        [ "$t" -ge "$timeout" ] && return 1
        sleep 5
        t=$((t + 5))
    done
    return 0
}

# Stop a process group by its leader's pid file: SIGINT (play_launch shuts its tree
# down), then SIGTERM, then SIGKILL. Only that group: never anything by name.
stop_group() {
    local pidfile="$1" what="$2" pid
    [ -f "$pidfile" ] || return 0
    pid="$(cat "$pidfile")"
    if kill -0 "$pid" 2>/dev/null; then
        log "stopping $what (pgid $pid)"
        kill -INT -- "-$pid" 2>/dev/null || true
        for _ in $(seq 1 30); do
            kill -0 "$pid" 2>/dev/null || break
            sleep 1
        done
        if kill -0 "$pid" 2>/dev/null; then
            kill -TERM -- "-$pid" 2>/dev/null || true
            for _ in $(seq 1 10); do
                kill -0 "$pid" 2>/dev/null || break
                sleep 1
            done
        fi
        kill -KILL -- "-$pid" 2>/dev/null || true
    fi
    rm -f "$pidfile"
}
