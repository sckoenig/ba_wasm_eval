#!/bin/bash
#===============================================================================
# Builds Go Artifacts.
# Per default, all modes and functions are built, but a specific mode or
# function can be configured.
# Usage:
# ./build_go.sh [-m local|docker|wasm] [-f fn]
#===============================================================================

set -e

MODE="all"
FNS=$(find . -mindepth 1 -maxdepth 1 -type d -printf '%f\n' | grep '^g_')

while getopts ":m:f:h" opt; do
  case $opt in
    m) MODE=$OPTARG ;;
    f) FNS=$OPTARG ;;
    h) echo "Usage: $0 [-m local|docker|wasm] [-f fn]"; exit 0 ;;
    \?) echo "Invalid option: -${OPTARG}" >&2; exit 1 ;;
    :)  echo "Option -${OPTARG} requires an argument" >&2; exit 1 ;;
  esac
done
printf "Building Go artifacts: \nMode: %s\nFunctions: \n%s \n--\n" "$MODE" "$FNS"

# create output directory for wasm binaries
WASM_PATH="../wasm"
mkdir -p "$WASM_PATH"
# create output directory for go native binaries
mkdir -p ./out

for fn in $FNS;
do
  echo "Building function: $fn"
  cd ./"$fn"
  echo "=> Installing dependencies ..."
  go mod download

  if [[ "$MODE" == "all" || "$MODE" == "local" ]]; then
    echo "=> Compiling native Go binary ..."
    CGO_ENABLED=0 go build -o ../out
  fi

  if [[ "$MODE" == "all" || "$MODE" == "wasm" ]]; then
    echo "=> Building wasm component ..."
    GOOS=wasip2 GOARCH=wasm tinygo build -o "../$WASM_PATH"/"$fn".wasm main.go

    echo "=> Compiling to cwasm with Wasmtime ..."
    wasmtime compile "../$WASM_PATH"/"$fn".wasm -o "../$WASM_PATH"/"$fn".cwasm
  fi

  cd ..
  if [[ "$MODE" == "all" || "$MODE" == "docker" ]]; then
    echo "=> Building Docker image ..."
    docker build --build-arg FUNCTION="$fn" -f ./Dockerfile -t "bench_$fn" .
  fi
done
