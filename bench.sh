#!/bin/bash

CPU_NAME=$(lscpu | grep "Model name:" | sed -r 's/Model name:\s{1,}//g')
CPU_NAME="${CPU_NAME// /_}"
EXPORT_FILE_NAME="$(date +%Y%m%d%H%M%S)_$(git rev-parse --short HEAD).json"

mkdir -p "bench_results/$CPU_NAME"

hyperfine \
    'cargo run --release -- -m silent -i 1000' \
    --export-json "bench_results/$CPU_NAME/$EXPORT_FILE_NAME" \
    --warmup 1
