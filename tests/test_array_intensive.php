<?php
// Intensive Array Benchmark - 10,000 operations with literal string keys
// Tests Phase 3C optimized array access

echo "=== Intensive Array Benchmark (10,000 operations) ===\n";
echo "\n";

// Create array with literal string keys
$data = ["x" => 100, "y" => 200, "z" => 300, "total" => 0];

echo "Starting intensive array operations...\n";

// Perform 10,000 iterations of array access and assignment
$i = 0;
while ($i < 10000) {
    // Read with literal string keys (OPTIMIZED in Phase 3C)
    $x = $data["x"];
    $y = $data["y"];
    $z = $data["z"];

    // Calculate
    $sum = $x + $y + $z;

    // Write with literal string keys (OPTIMIZED in Phase 3C)
    $data["total"] = $sum;

    $i = $i + 1;
}

echo "Operations complete!\n";
echo "Final total: ", $data["total"], "\n";
echo "Final x: ", $data["x"], "\n";
echo "Final y: ", $data["y"], "\n";
echo "Final z: ", $data["z"], "\n";
echo "\n";
echo "Benchmark complete! (40,000 total array operations)\n";
?>