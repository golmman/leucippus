#!/bin/bash

MACHINE_NAME=$(cat /proc/cpuinfo | grep Model | cut -d\  -f2-)
MACHINE_NAME="${MACHINE_NAME// /_}"
EXPORT_FILE_NAME="$(date +%Y_%m_%d_%H_%M_%S)_$(git rev-parse --short HEAD).json"

mkdir -p "bench_results/$MACHINE_NAME"

hyperfine \
    'cargo run --release -- -m silent -i 200' \
    --export-json "bench_results/$MACHINE_NAME/$EXPORT_FILE_NAME"
