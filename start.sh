#!/bin/bash

PORT=${1:-"8080"}

COMPILER_ERRORS=$(wasm-pack build --target web --out-name wasm --out-dir ./static 2>&1 | grep 'error: could not compile `aramuk-web`.')
ps aux | grep 'http.server' | awk '{print $2}' | xargs kill &> /dev/null
if [[ -z "$COMPILER_ERRORS" ]]; then
    cd ./static && python3 -m http.server ${PORT} &
    open "http://localhost:${PORT}"
else 
    echo "${COMPILER_ERRORS}"
fi
