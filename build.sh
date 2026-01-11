#!/bin/bash

echo "Building Edge PHP..."

# Build the Rust project
cargo build --release

# Run tests
echo -e "\nRunning tests..."
cargo test

# Test the CLI
echo -e "\nTesting CLI with example file..."
cargo run --bin edge-php -- parse examples/build-test.php

echo -e "\nBuild complete!"