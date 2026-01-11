<?php

// Read the baseline data
$data = json_decode(file_get_contents('php_execution_baseline.json'), true);

echo "=== PHP Native Performance Baseline ===\n\n";
echo "Timestamp: {$data['timestamp']}\n";
echo "Iterations per example: " . number_format($data['iterations_per_example']) . "\n\n";

echo "Example Performance (execution only, no startup):\n";
echo str_repeat("-", 50) . "\n";

foreach ($data['results'] as $name => $time) {
    printf("%-20s: %6.1fμs\n", $name, $time);
}

echo str_repeat("-", 50) . "\n";
printf("Average: %.1fμs\n", $data['summary']['average']);
printf("Fastest: %.1fμs\n", $data['summary']['min']);
printf("Slowest: %.1fμs\n", $data['summary']['max']);

echo "\n=== EdgePHP Comparison ===\n";
echo "EdgePHP typical execution: ~175μs\n";
$ratio = 175 / $data['summary']['average'];
printf("EdgePHP is approximately %.1fx slower than native PHP\n", $ratio);

echo "\n=== Important Notes ===\n";
echo "- PHP baseline excludes interpreter startup time\n";
echo "- EdgePHP time includes WASM execution overhead\n";
echo "- EdgePHP compiles to WASM for browser portability\n";
echo "- Native PHP uses optimized C implementation\n";