#!/bin/bash
#===============================================================================
# Builds Rust Artifacts.
# Per default, all modes and functions are built, but a specific mode or
# function can be configured.
# Usage:
# ./build_rust.sh [-m local|docker|wasm] [-f fn]
#===============================================================================

set -e

MODE="all"
FNS=$(find . -mindepth 1 -maxdepth 1 -type d -printf '%f\n' | grep '^r_')

while getopts ":m:f:h" opt; do
  case $opt in
    m) MODE=$OPTARG ;;
    f) FNS=$OPTARG ;;
    h) echo "Usage: $0 [-m local|docker|wasm] [-f fn]"; exit 0 ;;
    \?) echo "Invalid option: -${OPTARG}" >&2; exit 1 ;;
    :)  echo "Option -${OPTARG} requires an argument" >&2; exit 1 ;;
  esac
done
printf "Building Rust artifacts: \nMode: %s\nFunctions: \n%s \n--\n" "$MODE" "$FNS"

# create output directory for wasm binaries
WASM_PATH="../wasm"
mkdir -p "$WASM_PATH"

for fn in $FNS;
do
    echo "Building function: $fn"
    if [[ "$MODE" == "all" || "$MODE" == "local" ]]; then
      echo "=> Compiling native Rust binary ..."
      cargo build --release -p "$fn"
    fi

    if [[ "$MODE" == "all" || "$MODE" == "wasm" ]]; then
      echo "=> Building wasm component ..."
      cargo build --release --target=wasm32-wasip2 -p "$fn"
      cp ./target/wasm32-wasip2/release/"$fn".wasm "$WASM_PATH"/"$fn".wasm

      echo "=> Compiling to cwasm with Wasmtime ..."
      wasmtime compile "$WASM_PATH"/"$fn".wasm -o "$WASM_PATH"/"$fn".cwasm
    fi

    if [[ "$MODE" == "all" || "$MODE" == "docker" ]]; then
      echo "=> Building Docker image ..."
      docker build --build-arg FUNCTION="$fn" -f ./Dockerfile -t "bench_$fn" .
    fi
done
