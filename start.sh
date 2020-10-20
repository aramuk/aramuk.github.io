#!/bin/bash

PORT=${1:-"8080"}
COMPILER_ERRORS=$(wasm-pack build --target web --out-name wasm --out-dir ./static 2>&1 | grep 'error: could not compile `aramuk-web`.')
if [[ -z "$COMPILER_ERRORS" ]]; then
    cd ./static && python3 -m http.server $PORT &> /dev/null &
    pid=$!
    sleep 1
    open "http://localhost:${PORT}"
    kill "${pid}"
fi
