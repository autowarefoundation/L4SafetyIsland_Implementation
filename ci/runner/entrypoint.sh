#!/usr/bin/env bash
# cspell:ignore disableupdate jitconfig
# The runner container's command (runs as the aw user, after Autoware's entrypoint).
#
# One container serves one job, then exits; the host loop (register.sh loop) starts a
# fresh container for the next one. Two ways in, JIT preferred:
#
#   ACTIONS_RUNNER_JITCONFIG  a just-in-time config (register.sh jit): single use, the
#                             runner is registered, labelled and ephemeral already.
#   RUNNER_URL + RUNNER_TOKEN a registration token (register.sh token): this script
#                             configures an --ephemeral runner with RUNNER_LABELS.
#
# Neither value is written anywhere but the runner's own .runner/.credentials files in
# this container, which --rm removes with it.
set -euo pipefail

cd /opt/actions-runner

if [ -n "${ACTIONS_RUNNER_JITCONFIG:-}" ]; then
    jit="$ACTIONS_RUNNER_JITCONFIG"
    unset ACTIONS_RUNNER_JITCONFIG
    exec ./run.sh --jitconfig "$jit"
fi

if [ -n "${RUNNER_URL:-}" ] && [ -n "${RUNNER_TOKEN:-}" ]; then
    token="$RUNNER_TOKEN"
    unset RUNNER_TOKEN
    ./config.sh --unattended --ephemeral --replace --disableupdate \
        --url "$RUNNER_URL" --token "$token" \
        --name "${RUNNER_NAME:-l4si-$(hostname)}" \
        --labels "${RUNNER_LABELS:-l4si-integration}" \
        --work /home/aw/_work
    unset token
    exec ./run.sh
fi

echo "l4si-runner: set ACTIONS_RUNNER_JITCONFIG, or RUNNER_URL and RUNNER_TOKEN (see ci/runner/README.md)" >&2
exit 2
