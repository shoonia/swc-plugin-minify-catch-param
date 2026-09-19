#!/bin/sh

set -eu

project_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
target_dir="$project_dir/target/wasm32-wasip1/release"
wasm_file="$target_dir/swc_plugin_minify_catch_param.wasm"
optimized_file="$wasm_file.tmp"

trap 'rm -f "$optimized_file"' EXIT HUP INT TERM

cargo build \
    --manifest-path "$project_dir/Cargo.toml" \
    --target wasm32-wasip1 \
    --release

cargo run \
    --manifest-path "$project_dir/tools/optimize-wasm/Cargo.toml" \
    --release \
    -- "$wasm_file" "$optimized_file"

mv "$optimized_file" "$wasm_file"
printf '\nBuilt and optimized WASM artifact: %s\n\n' "$wasm_file"

trap - EXIT HUP INT TERM
