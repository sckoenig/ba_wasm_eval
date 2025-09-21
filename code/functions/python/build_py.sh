#!/bin/bash
#===============================================================================
# Builds Python Artifacts.
# Per default, all modes and functions are built, but a specific mode or
# function can be configured.
# Usage:
# ./build_py.sh [-m local|docker|wasm] [-f fn]
#===============================================================================
set -e

MODE="all"
FNS=$(find . -mindepth 1 -maxdepth 1 -type d -printf '%f\n' | grep '^p_')

while getopts ":m:f:h" opt; do
  case $opt in
    m) MODE=$OPTARG ;;
    f) FNS=$OPTARG ;;
    h) echo "Usage: $0 [-m local|docker|wasm] [-f fn]"; exit 0 ;;
    \?) echo "Invalid option: -${OPTARG}" >&2; exit 1 ;;
    :)  echo "Option -${OPTARG} requires an argument" >&2; exit 1 ;;
  esac
done
printf "Building Python artifacts: \nMode: %s\nFunctions: \n%s \n--\n" "$MODE" "$FNS"

# create output directory for wasm binaries
WASM_PATH="../wasm"
mkdir -p "$WASM_PATH"

pip install ./bench/bench.tar.gz

for fn in $FNS;
do
  echo "Building function: $fn"

  if [[ "$MODE" == "all" || "$MODE" == "wasm" || "$MODE" == "local" ]]; then
    echo "=> Installing requirements ..."
    pip install -r ./"$fn"/requirements.txt
  fi

  if [[ "$MODE" == "all" || "$MODE" == "wasm" ]]; then
    echo "=> Building wasm component ..."
    componentize-py -d ../wit -w wasi:cli/command@0.2.0 componentize -p ./"$fn" wasm -o "$WASM_PATH"/"$fn".wasm

    echo "=> Compiling to cwasm with Wasmtime ..."
    wasmtime compile "$WASM_PATH"/"$fn".wasm -o "$WASM_PATH"/"$fn".cwasm
  fi

  if [[ "$MODE" == "all" || "$MODE" == "docker" ]]; then
    echo "=> Building Docker image ..."
    docker build -f ./Dockerfile -t "bench_$fn" --build-arg FUNCTION="$fn" .
  fi

done