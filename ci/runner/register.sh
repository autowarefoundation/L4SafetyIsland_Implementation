#!/usr/bin/env bash
# cspell:ignore argjson jitconfig startswith
# Register and run the L4 Safety Island integration runner (an admin runs this on the
# runner host, never in CI). See ci/runner/README.md.
#
#   register.sh jit       print a just-in-time runner config (single use, ephemeral)
#   register.sh token     print a registration token (1 hour; for config.sh --ephemeral)
#   register.sh loop      run the runner forever: a fresh JIT config and a fresh
#                         container (docker compose run --rm) per job
#   register.sh list      list this scope's self-hosted runners
#   register.sh prune     remove this scope's offline runners named $RUNNER_PREFIX*
#
# Scope: L4SI_RUNNER_REPO=<owner>/<repo> (repository runner), or L4SI_RUNNER_ORG=<org>
# (organization runner; L4SI_RUNNER_GROUP_ID selects the runner group, default 1).
#
# Credentials: a GitHub token that may administer the scope's runners, read from the
# file L4SI_GH_TOKEN_FILE (default ~/.config/l4si-runner/token, mode 600) or from
# GH_TOKEN. Fine-grained PAT: repository "Administration: read and write" (repo scope),
# or organization "Self-hosted runners: read and write" (org scope). The token never
# leaves this host: only the single-use JIT config reaches the container.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
api="${GITHUB_API_URL:-https://api.github.com}"
labels="${RUNNER_LABELS:-self-hosted,linux,x64,l4si-integration}"
prefix="${RUNNER_PREFIX:-l4si-}"
compose=(docker compose -f "$here/compose.yaml")

die() {
    echo "register.sh: $*" >&2
    exit 2
}

scope_path() {
    if [ -n "${L4SI_RUNNER_REPO:-}" ]; then
        echo "repos/$L4SI_RUNNER_REPO"
    elif [ -n "${L4SI_RUNNER_ORG:-}" ]; then
        echo "orgs/$L4SI_RUNNER_ORG"
    else die "set L4SI_RUNNER_REPO=<owner>/<repo> or L4SI_RUNNER_ORG=<org>"; fi
}

read_token() {
    local f="${L4SI_GH_TOKEN_FILE:-$HOME/.config/l4si-runner/token}"
    if [ -f "$f" ]; then
        [ "$(stat -c %a "$f")" = 600 ] || die "$f must be mode 600"
        tr -d '\n' <"$f"
    elif [ -n "${GH_TOKEN:-}" ]; then
        printf '%s' "$GH_TOKEN"
    else
        die "no token: write it to $f (mode 600) or set GH_TOKEN"
    fi
}

# gh_api METHOD PATH [JSON]: the token goes to curl on stdin (-K -), never on argv.
gh_api() {
    local method="$1" path="$2" body="${3:-}"
    local args=(-fsS -X "$method" -H "Accept: application/vnd.github+json"
        -H "X-GitHub-Api-Version: 2022-11-28" "$api/$path")
    [ -n "$body" ] && args+=(-H "Content-Type: application/json" -d "$body")
    printf 'header = "Authorization: Bearer %s"\n' "$(read_token)" | curl -K - "${args[@]}"
}

jit_config() {
    local name="${RUNNER_NAME:-$prefix$(hostname -s)-$(date +%s)}" body
    body="$(jq -nc --arg name "$name" --arg labels "$labels" \
        --argjson group "${L4SI_RUNNER_GROUP_ID:-1}" \
        '{name: $name, runner_group_id: $group, labels: ($labels | split(",")), work_folder: "_work"}')"
    gh_api POST "$(scope_path)/actions/runners/generate-jitconfig" "$body" |
        jq -er .encoded_jit_config
}

case "${1:-}" in
jit) jit_config ;;
token) gh_api POST "$(scope_path)/actions/runners/registration-token" | jq -er .token ;;
list)
    gh_api GET "$(scope_path)/actions/runners?per_page=100" |
        jq -r '.runners[] | "\(.id)\t\(.name)\t\(.status)\tbusy=\(.busy)\t\([.labels[].name] | join(","))"'
    ;;
prune)
    gh_api GET "$(scope_path)/actions/runners?per_page=100" |
        jq -r --arg p "$prefix" '.runners[] | select(.status == "offline" and (.name | startswith($p))) | .id' |
        while read -r id; do
            echo "removing offline runner $id"
            gh_api DELETE "$(scope_path)/actions/runners/$id" >/dev/null
        done
    ;;
loop)
    command -v jq >/dev/null || die "jq is required"
    backoff=10
    while true; do
        # CARLA has no restart policy: (re)start it and wait for its RPC port before
        # taking a job, so a dead CARLA stops here instead of failing jobs.
        if ! "${compose[@]}" up -d --wait --wait-timeout 600 carla; then
            echo "register.sh: CARLA is not healthy; retry in ${backoff}s" >&2
            sleep "$backoff"
            backoff=$((backoff < 300 ? backoff * 2 : 300))
            continue
        fi
        if ! jit="$(jit_config)"; then
            echo "register.sh: no JIT config; retry in ${backoff}s" >&2
            sleep "$backoff"
            backoff=$((backoff < 300 ? backoff * 2 : 300))
            continue
        fi
        backoff=10
        # One job, one container: --rm drops the workspace, caches stay in volumes.
        ACTIONS_RUNNER_JITCONFIG="$jit" "${compose[@]}" run --rm --no-deps \
            -e ACTIONS_RUNNER_JITCONFIG runner || true
        unset jit
    done
    ;;
*)
    sed -n '2,20p' "$0" | sed 's/^# \{0,1\}//'
    exit 2
    ;;
esac
