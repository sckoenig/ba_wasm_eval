#!/bin/bash
#=======================================================================================
# Runs the whole benchmark.
# The default configuration is running
# each function in each mode, language and input for 25 iterations.
# The script will assume all artifacts have been built.
# Function inputs are read from ../input/inputs.csv.
#
# The configuration can be changed:
# Usage: ./run_bench.sh [-l all|r|p|j|g] [-s all|small|large|common] [-m all|local|wasm|cwasm|docker] [-i iterations] [-f /path/to/inputfile]
#=======================================================================================

### Default config
CSV="./input/inputs.csv"
language="all"; size="all"; iterations=25 mode="all"

### Set config with optional arguments
while getopts ":l:s:i:m:f:h" opt; do
  case $opt in
    l) language=$OPTARG ;;
    s) size=$OPTARG ;;
    i) iterations=$OPTARG ;;
    m) mode=$OPTARG ;;
    f) CSV=$OPTARG ;;
    h) echo "Usage: $0 [-l all|r|p|j|g] [-s all|small|large|common] [-m all|local|wasm|cwasm|docker] [-i iterations] [-f /path/to/inputfile]"; exit 0 ;;
    \?) echo "Invalid option: -${OPTARG}" >&2; exit 1 ;;
    :)  echo "Option -${OPTARG} requires an argument" >&2; exit 1 ;;
  esac
done

### Get inputs for benchmark functions from .csv
declare -A benchmarks_per_lang      # benchmarks_per_lang[language] = “fn1 fn2 fn3 ...”
declare -A inputs                   # inputs["$language;$fn;$size"] = input_value

while IFS=";" read -r fn small large common; do
  [[ $fn == fn ]] && continue # skip header
  language_c="${fn%_*}"
  benchmarks_per_lang[$language_c]+=" $fn"

  inputs["$fn;small"]=$small
  inputs["$fn;common"]=$common
  inputs["$fn;large"]=$large
done < "$CSV"

languages=$([[ $language == "all" ]] && echo "${!benchmarks_per_lang[@]}" || echo "$language")
sizes=$([[ $size == all ]] && echo small large common || echo "$size")
modes=$([[ $mode == all ]] && echo local docker wasm cwasm || echo "$mode")

printf "Running script with: \n Language: %s \n Inputsize: %s \n Iterations: %s \n Execution Mode: %s \n" "$languages" "$sizes" "$iterations" "$modes"

RESULT_PATH="./results/performance/${EPOCHREALTIME/[^0-9]/}"
mkdir -p "$RESULT_PATH"

### Run benchmarks
for lang in $languages; do
  for fn in ${benchmarks_per_lang[$lang]}; do
    for sz in $sizes; do
      input="${inputs["$fn;$sz"]}"
      [[ -z "$input" ]] && continue

      for mode in $modes; do
        if [[ "$fn" == "g_thumbnailer" && ("$mode" == "wasm" || "$mode" == "cwasm") ]]; then
          # replace ./input/.. with /input/..
          # why? Go-wasm-wasi seems to handle paths differently and must use absolute paths
          # find a better / more generic way
          input="${input/.\/input/\/input}" # var / pattern / replace
        fi
        ./scripts/run_function.sh "$fn" "$input" "$mode" "$iterations" "$sz" "$RESULT_PATH"
      done
    done
  done
done
