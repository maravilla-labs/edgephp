#!/bin/bash

# Script to update playground with latest benchmark results

echo "Updating playground with benchmark results..."

# Check if benchmark results exist
if [ ! -f "benchmark_results.json" ]; then
    echo "Error: benchmark_results.json not found. Run ./run_benchmark.sh first."
    exit 1
fi

# Copy to playground public directory so it can be fetched
cp benchmark_results.json ../playground/public/benchmark_results.json

echo "Benchmark results copied to playground/public/"
echo "The playground can now fetch /benchmark_results.json"