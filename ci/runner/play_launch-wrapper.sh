#!/usr/bin/env bash
# cspell:ignore agnocast
# /opt/l4si/bin/play_launch: play_launch with `--parser $L4SI_PLAY_LAUNCH_PARSER` added to
# `play_launch launch` (and `dump launch`) when the caller gave no --parser. See the
# Dockerfile's note: the Rust parser cannot evaluate Autoware 2.0.0's
# agnocast_env.launch.py. Unset L4SI_PLAY_LAUNCH_PARSER to get play_launch as it is.
set -euo pipefail
real=/usr/local/bin/play_launch
parser="${L4SI_PLAY_LAUNCH_PARSER:-}"

if [ -z "$parser" ] || [[ " $* " == *" --parser "* ]] || [[ " $* " == *" --parser="* ]]; then
    exec "$real" "$@"
fi
case "${1:-}" in
launch) exec "$real" launch --parser "$parser" "${@:2}" ;;
dump) [ "${2:-}" = launch ] && exec "$real" dump launch --parser "$parser" "${@:3}" ;;
esac
exec "$real" "$@"
