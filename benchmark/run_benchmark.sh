#!/bin/bash

echo "=== EdgePHP Benchmark Suite ==="
echo

# Run the benchmark
node run_benchmark.js

# Copy results to playground if benchmark succeeded
if [ -f "benchmark_results.json" ]; then
    echo ""
    echo "Copying results to playground..."
    cp benchmark_results.json ../playground/public/benchmark_results.json
    echo "✓ Results available in playground"
fi