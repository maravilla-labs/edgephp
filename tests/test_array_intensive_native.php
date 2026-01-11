<?php
// Native PHP Version - Intensive Array Benchmark
// For comparison with EdgePHP WASM implementation

$start = microtime(true);

// Create array with literal string keys
$data = ["x" => 100, "y" => 200, "z" => 300, "total" => 0];

// Perform 10,000 iterations of array access and assignment
$i = 0;
while ($i < 10000) {
    // Read with literal string keys
    $x = $data["x"];
    $y = $data["y"];
    $z = $data["z"];

    // Calculate
    $sum = $x + $y + $z;

    // Write with literal string keys
    $data["total"] = $sum;

    $i = $i + 1;
}

$end = microtime(true);
$elapsed = ($end - $start) * 1000; // Convert to milliseconds

echo "Native PHP Results:\n";
echo "==================\n";
echo "Operations: 40,000\n";
echo "Execution Time: " . number_format($elapsed, 3) . "ms\n";
echo "Throughput: " . number_format(40000 / $elapsed * 1000, 2) . " ops/second\n";
echo "\n";
echo "Final values:\n";
echo "  total: " . $data["total"] . "\n";
echo "  x: " . $data["x"] . "\n";
echo "  y: " . $data["y"] . "\n";
echo "  z: " . $data["z"] . "\n";
?>