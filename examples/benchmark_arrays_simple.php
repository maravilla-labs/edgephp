<?php
// Simplified Array Operations Benchmark
echo "=== Array Operations Benchmark ===\n\n";

// Test 1: Array creation and sequential access
echo "Test 1: Sequential Array Access\n";
$arr = [1, 2, 3, 4, 5];
$sum = 0;
for ($i = 0; $i < 5; $i = $i + 1) {
    $sum = $sum + $arr[$i];
}
echo "Sum: " . $sum . "\n\n";

// Test 2: Array building in loop
echo "Test 2: Array Building\n";
$arr2 = [];
for ($i = 0; $i < 10; $i = $i + 1) {
    $arr2[$i] = $i * 2;
}
echo "arr2[5] = " . $arr2[5] . "\n\n";

echo "Array benchmark complete!\n";
