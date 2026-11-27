#!/usr/bin/env bash
# cspell:ignore setsid
# shellcheck disable=SC1091 # lib.sh is sourced by path
# Start the safety island next to the stack (L4SI_WITH_ISLAND=1).
#
# The island here is the host build of this repository (platforms/host, nano-ros POSIX).
# It joins the ego's ROS domain (seat 0's first pilot domain) with swb's DDS profile, so
# it sees the ego Autoware's topics as a separate ECU would.
#
#   L4SI_ISLAND_TARGET  host (default) | qemu | external | hil
#                       external is an island another repository builds: it brings its
#                       own L4SI_ISLAND_CMD (and L4SI_ISLAND_BUILD, if any), mounted
#                       into the container by a compose override of its own.
#                       hil is the real board on a `l4si-hil` runner: not wired yet.
#   L4SI_ISLAND_BUILD   the build command (default: the host platform's cargo build)
#   L4SI_ISLAND_CMD     the command that runs the island (default: the built binary)
set -euo pipefail
source "$(dirname "$0")/lib.sh"
l4si_env

target="${L4SI_ISLAND_TARGET:-host}"
host_dir="$L4SI_ROOT/platforms/host"

case "$target" in
host)
    if [ -z "${L4SI_ISLAND_BUILD:-}" ] && [ ! -f "$host_dir/Cargo.toml" ]; then
        die 3 "no $host_dir/Cargo.toml: the host platform is not in this checkout yet"
    fi
    build="${L4SI_ISLAND_BUILD:-cargo build --release --manifest-path $host_dir/Cargo.toml}"
    ;;
qemu | external)
    [ -n "${L4SI_ISLAND_CMD:-}" ] || die 3 "L4SI_ISLAND_TARGET=$target needs L4SI_ISLAND_CMD"
    build="${L4SI_ISLAND_BUILD:-true}"
    ;;
hil)
    die 3 "L4SI_ISLAND_TARGET=hil runs on a runner labelled l4si-hil (not wired yet)"
    ;;
*) die 3 "unknown L4SI_ISLAND_TARGET=$target" ;;
esac

log "building the island ($target): $build"
(cd "$L4SI_ROOT" && bash -c "$build") >"$L4SI_STATE/island-build.log" 2>&1 ||
    {
        tail -n 40 "$L4SI_STATE/island-build.log" >&2
        die 3 "the island build failed"
    }

cmd="${L4SI_ISLAND_CMD:-}"
if [ -z "$cmd" ]; then
    # The host platform's binary: the one bin target cargo built.
    bin="$(cargo metadata --manifest-path "$host_dir/Cargo.toml" --format-version 1 --no-deps |
        jq -r '[.packages[].targets[] | select(.kind[]=="bin") | .name][0] // empty')"
    target_dir="$(cargo metadata --manifest-path "$host_dir/Cargo.toml" --format-version 1 --no-deps | jq -r .target_directory)"
    [ -n "$bin" ] && [ -x "$target_dir/release/$bin" ] || die 3 "no island binary under $target_dir/release"
    cmd="$target_dir/release/$bin"
fi

# The ego's domain and swb's DDS profile (the same exports `swb seat env 0` gives a shell).
eval "$(ros2 run swb seat env 0)"
log "starting the island: $cmd (ROS_DOMAIN_ID=$ROS_DOMAIN_ID)"
L4SI_STATE="$L4SI_STATE" setsid bash -c "exec $cmd" >"$L4SI_STATE/island.log" 2>&1 </dev/null &
echo $! >"$L4SI_STATE/island.pid"
sleep 2
kill -0 "$(cat "$L4SI_STATE/island.pid")" 2>/dev/null ||
    {
        tail -n 40 "$L4SI_STATE/island.log" >&2
        die 3 "the island exited at start"
    }
