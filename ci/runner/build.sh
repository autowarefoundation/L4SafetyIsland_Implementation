#!/usr/bin/env bash
# Build the CI images: l4si-ci-stack (no runner) and l4si-ci-runner (stack + runner).
#   ci/runner/build.sh [tag]          default tag: dev
# Extra docker build flags via L4SI_BUILD_ARGS, e.g.
#   L4SI_BUILD_ARGS="--build-arg SWB_REF=<commit> --build-arg AUTOWARE_VERSION=2.0.0"
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
tag="${1:-dev}"
read -r -a extra <<<"${L4SI_BUILD_ARGS:-}"

docker build -f "$here/Dockerfile" --target stack -t "l4si-ci-stack:$tag" "${extra[@]}" "$here"
docker build -f "$here/Dockerfile" --target runner -t "l4si-ci-runner:$tag" "${extra[@]}" "$here"
docker image ls --format '{{.Repository}}:{{.Tag}}\t{{.Size}}' | grep -E "^l4si-ci-(stack|runner):$tag"
