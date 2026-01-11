#!/bin/bash

# Compile all benchmark examples to WASM
echo "=== Compiling benchmark examples ==="

# Build EdgePHP in release mode first
echo "Building EdgePHP compiler..."
cd .. && cargo build --release --bin edge-php
cd benchmark

# Compile each example
for example in examples/*.php; do
    basename=$(basename "$example" .php)
    echo "Compiling $basename.php..."
    ../target/release/edge-php compile "$example" -o "compiled/${basename}.wasm" --optimize
done

echo "=== Compilation complete ==="
echo "WASM files saved to benchmark/compiled/"