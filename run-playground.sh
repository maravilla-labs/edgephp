#!/bin/bash

echo "Starting Edge PHP Playground with WASM..."

# First build the WASM module if needed
if [ ! -f "playground/src/wasm/edge_php_wasm_bg.wasm" ]; then
    echo "Building WASM module..."
    ./build-wasm.sh
fi

# Install playground dependencies
cd playground
if [ ! -d "node_modules" ]; then
    echo "Installing playground dependencies..."
    npm install
fi

# Start the dev server
echo "Starting playground at http://localhost:5173"
npm run dev