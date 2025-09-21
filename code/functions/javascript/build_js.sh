#!/bin/bash
#===============================================================================
# Builds JavaScript Artifacts.
# Per default, all modes and functions are built, but a specific mode or
# function can be configured.
# Usage:
# ./build_js.sh [-m local|docker|wasm] [-f fn]
#===============================================================================
set -e

MODE="all"
FNS=$(find . -mindepth 1 -maxdepth 1 -type d -printf '%f\n' | grep '^j_')

while getopts ":m:f:h" opt; do
  case $opt in
    m) MODE=$OPTARG ;;
    f) FNS=$OPTARG ;;
    h) echo "Usage: $0 [-m local|docker|wasm] [-f fn]"; exit 0 ;;
    \?) echo "Invalid option: -${OPTARG}" >&2; exit 1 ;;
    :)  echo "Option -${OPTARG} requires an argument" >&2; exit 1 ;;
  esac
done
printf "Building JavaScript artifacts: \nMode: %s\nFunctions: \n%s \n--\n" "$MODE" "$FNS"

# create output directory for wasm binaries
WASM_PATH="../wasm"
mkdir -p "$WASM_PATH"

# build artifacts
for fn in $FNS;
do
  echo "Building function: $fn"

  # install dependencies for running locally / wasm
  if [[ "$MODE" == "all" || "$MODE" == "local" || "$MODE" == "wasm" ]]; then
    echo "=> Installing requirements ..."
    npm install --prefix ./"$fn"
  fi

  # bundle for local execution
  if [[ "$MODE" == "all" || "$MODE" == "local" ]]; then
    echo "=> Bundling for local execution ..."
    npm run build:main --prefix ./"$fn"
  fi

  # bundle for wasm execution and build wasm components
  if [[ "$MODE" == "all" || "$MODE" == "wasm" ]]; then

    echo "=> Building wasm component ..."
    npm run build:wasm --prefix ./"$fn" # need to bundle js for componentize-js when using dependencies
    componentize-js --aot -d http -w ../wit -n wasi:cli/command -o "$WASM_PATH"/"$fn".wasm ./"$fn"/bundle/wasm_bundle.mjs

    echo "=> Compiling to cwasm with Wasmtime ..."
    wasmtime compile "$WASM_PATH"/"$fn".wasm -o "$WASM_PATH"/"$fn".cwasm
  fi

  # build docker images
  if [[ "$MODE" == "all" || "$MODE" == "docker" ]]; then
    echo "=> Building Docker image ..."
    docker build -f ./Dockerfile -t "bench_$fn" --build-arg FUNCTION="$fn" .
  fi
done