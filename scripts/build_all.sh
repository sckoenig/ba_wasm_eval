#!/bin/bash
#=======================================================================================
# Builds all benchmark function artifacts for local execution, wasm, wasm aot and docker
# and saves their sizes to a file.
# To build only a specific language, use the language's build script instead.
# Usage: ./build_all.sh
#=======================================================================================
set -e

cd ./code/wasm_host || exit
cargo build --release

cd ../functions/rust || exit
./build_rust.sh all

cd ../python || exit
./build_py.sh all

cd ../javascript || exit
./build_js.sh all

cd ../go || exit
./build_go.sh all

cd ../wasm || exit

RESULT_PATH="../../../results/artifact_sizes"
mkdir -p "$RESULT_PATH"
du -b *.wasm | numfmt --to=si --suffix=B --format="%.2f" > "$RESULT_PATH/wasm.txt"
du -b *.cwasm | numfmt --to=si --suffix=B --format="%.2f" > "$RESULT_PATH/cwasm.txt"

docker images --filter=reference='bench_*_*' --format='{{.Repository}} {{.ID}}' |
while read -r repo id; do
  bytes=$(docker image inspect "$id" --format='{{.Size}}');
  mb=$(awk "BEGIN{printf \"%.3fMB\", $bytes/1000000}");
  echo "$mb $repo";
done > "$RESULT_PATH/docker.txt"