#!/bin/bash

set -Eeuo pipefail
PID=""

cleanup () {
    if [[ -nz "$PID" ]]; then
        kill ${PID}
    fi
    trap - SIGINT SIGTERM ERR EXIT
    exit
}

trap cleanup SIGINT SIGTERM ERR EXIT

PORT=${1:-"8080"}

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)

error () {
    printf "Error: %s\n" "$*" >&2
}

COMPILER_ERRORS=$(wasm-pack build --target web --out-name wasm --out-dir ./static 2>&1 | grep 'error: could not compile `aramuk-web`.')
if [[ -z "$COMPILER_ERRORS" ]]; then
    PID=$(cd ./static && python3 -m http.server ${PORT} &>/dev/null && echo $!)
    sleep 1
    LINK="http://localhost:${PORT}"
    echo "Project running on ${LINK}"
    open "${LINK}"
else
    error "failed to compile ${basename ${script_dir}}" 
fi