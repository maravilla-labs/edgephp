# EdgePHP Benchmark Suite

This benchmark suite compares EdgePHP (WASM) performance against native PHP execution.

## Structure

```
benchmark/
├── examples/          # PHP test scripts
│   ├── minimal.php
│   ├── assignment.php
│   ├── arithmetic.php
│   ├── strings.php
│   └── comprehensive.php
├── compiled/         # Pre-compiled WASM files (generated)
├── compile_all.sh    # Compile all examples to WASM
├── run_benchmark.sh  # Run the benchmark
└── run_benchmark.js  # Main benchmark logic
```

## What We Measure

1. **PHP Cold Start**: Full PHP interpreter startup + code execution
2. **EdgePHP Cold Start**: Node.js startup + WASM loading + execution (pre-compiled WASM)

Note: EdgePHP compilation time is NOT included in the benchmark. WASM files are pre-compiled.

## Running Benchmarks

```bash
# First time: compile all examples
./compile_all.sh

# Run benchmarks
./run_benchmark.sh
```

## Metrics

The benchmark measures:

1. **Execution only**: Pure code execution time (after warmup)
   - PHP: Using eval() within running interpreter
   - EdgePHP: Using pre-loaded WASM module

2. **Cold start**: Full startup + execution
   - PHP: `php script.php` (interpreter startup + execution)
   - EdgePHP: `node runner.js` (Node.js startup + WASM load + execution)

3. **Overhead calculations**:
   - PHP interpreter overhead = Cold start - Execution time
   - WASM load overhead = Cold start - Execution time

## Example Output

```
MINIMAL
──────────────────────────────────────────────────
PHP:
  Execution only:    0.001ms
  Cold start:        13.787ms
  Interpreter time:  13.786ms

EdgePHP:
  Execution only:    0.476ms
  Cold start:        21.272ms
  WASM load time:    20.796ms

Ratios:
  Execution: EdgePHP is 728.9x slower
  Cold start: EdgePHP is 1.5x slower
```

## Results

- EdgePHP execution is slower due to WASM overhead (expected)
- EdgePHP cold start is competitive (only ~1.5x slower than PHP)
- Results are automatically copied to playground for visualization

## Notes

- All measurements exclude outliers (top/bottom 10%)
- PHP measurements use `-dxdebug.mode=off` to disable XDebug
- EdgePHP compilation happens separately and is not measured