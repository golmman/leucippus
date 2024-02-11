#!/bin/bash

BENCH_COMMAND='cargo run --release -- -m silent -i 1000'

cargo build --release

if [[ $1 != '-r' ]]; then
    echo "quick run, no export, use '-r' option for release run"
    hyperfine "$BENCH_COMMAND"
    exit 0
fi

CPU_NAME=$(lscpu | grep "Model name:" | sed -r 's/Model name:\s{1,}//g')
CPU_NAME="${CPU_NAME// /_}"
EXPORT_DIR_NAME="bench_results/$CPU_NAME"
EXPORT_FILE_NAME="$(date +%Y%m%d%H%M%S)_$(git rev-parse --short HEAD).json"
EXPORT_PATH_NAME="$EXPORT_DIR_NAME/$EXPORT_FILE_NAME"

echo "release run, export results to '$EXPORT_PATH_NAME'"

mkdir -p "$EXPORT_DIR_NAME"

hyperfine \
    "$BENCH_COMMAND" \
    --export-json "$EXPORT_PATH_NAME" \
    --warmup 1
