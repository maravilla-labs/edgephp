#!/bin/bash

echo "Building Edge PHP WASM module..."

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WASM module
cd packages/wasm-bindings
wasm-pack build --target web --out-dir ../../playground/src/wasm

# Copy the generated files to playground
echo "WASM module built successfully!"
echo "Files generated in playground/src/wasm/"

cd ../..
ls -la playground/src/wasm/