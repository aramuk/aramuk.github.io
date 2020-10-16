#!/bin/bash

PORT=${1:-"8080"}
wasm-pack build --target web --out-name wasm --out-dir ./static
cd ./static && python3 -m http.server $PORT
