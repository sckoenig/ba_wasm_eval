#!/bin/bash
#=======================================================================================
# Runs a specific function in a specific configuration.
# The script will assume all artifacts have been built.
#
# Usage:
# ./run_function.sh FUNCTION LANGUAGE(r|p|j|g) INPUT MODE(docker|wasm|cwasm|local) ITERATIONS ID RESULT_PATH
# Creates a result file like this: RESULT_PATH/<LANGUAGE>_<FUNCTION>_<MODE>_<ID>.txt
#=======================================================================================

FUNCTION=$1
INPUT=$2
MODE=$3
ITER=$4
ID=$5
RESULT_PATH=$6

WASM_HOST="./code/wasm_host/target/release/wasm_host"
WASM_PATH="./code/functions/wasm"
FUNCTIONS_PATH="./code/functions/"

usage="Usage: $0 FUNCTION LANGUAGE(r|p|j|g) INPUT MODE(docker|wasm|cwasm|local) ITERATIONS ID
Results in a file named: <LANGUAGE>_<FUNCTION>_<MODE>_<ID>.txt"

# choose execution command
case $MODE in
  local)
     case "${FUNCTION%_*}" in
       r) cmd="$FUNCTIONS_PATH/rust/target/release/$FUNCTION" ;;
       p) cmd="python3 $FUNCTIONS_PATH/python/$FUNCTION/main.py" ;;
       j) cmd="node $FUNCTIONS_PATH/javascript/$FUNCTION/bundle/main_bundle.mjs" ;;
       g) cmd="$FUNCTIONS_PATH/go/out/$FUNCTION" ;;
       *) echo "$usage"; exit 0 ;;
    esac ;;
  docker) cmd="docker run --rm --mount type=bind,source=./input,target=/input bench_$FUNCTION";;
  wasm)  cmd="$WASM_HOST $WASM_PATH/$FUNCTION.wasm";;
  cwasm) cmd="$WASM_HOST --aot $WASM_PATH/$FUNCTION.cwasm";;
  *) echo "$usage"; exit 0 ;;
esac

# setup logfile
log_file="${RESULT_PATH}/${FUNCTION}_${MODE}_${ID}.txt"
echo "Start(Unix Epoch) ComputationStart(Unix Epoch) ComputationEnd(Unix Epoch)">"$log_file"

# run benchmark
for ((i=1; i<=ITER; i++))
  do
      echo "Running Benchmark for $FUNCTION in $MODE and input=$INPUT -- Iteration: $i / $ITER."
      {
        # the wasm host makes the first timestamp in wasm and cwasm modus
        [[ $MODE == "docker" || $MODE == "local" ]] && echo -n "${EPOCHREALTIME/[^0-9]/} "
        $cmd "$INPUT" 1>/dev/null
        sleep 2
      } >> "$log_file" 2>&1
done
