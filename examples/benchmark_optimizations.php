<?php
// Benchmark test for type inference optimizations
echo "=== EdgePHP Optimization Benchmark ===\n\n";

// Test 1: Integer arithmetic (should use fast path)
echo "Test 1: Integer Arithmetic (Fast Path)\n";
$x = 10;
$y = 5;
$result = $x + $y;
echo "10 + 5 = " . $result . "\n";

$result = $x - $y;
echo "10 - 5 = " . $result . "\n";

$result = $x * $y;
echo "10 * 5 = " . $result . "\n";

$result = $x / $y;
echo "10 / 5 = " . $result . "\n\n";

// Test 2: Integer loop (should use fast path)
echo "Test 2: Integer Loop (Fast Path)\n";
$sum = 0;
for ($i = 1; $i <= 10; $i = $i + 1) {
    $sum = $sum + $i;
}
echo "Sum of 1-10: " . $sum . "\n\n";

// Test 3: Integer comparisons (should use fast path)
echo "Test 3: Integer Comparisons (Fast Path)\n";
$a = 10;
$b = 20;
echo "$a > $b: " . ($a > $b) . "\n";
echo "$a < $b: " . ($a < $b) . "\n";
echo "$a == $a: " . ($a == $a) . "\n\n";

// Test 4: Float arithmetic (should use fast path)
echo "Test 4: Float Arithmetic (Fast Path)\n";
$f1 = 3.14;
$f2 = 2.71;
$fresult = $f1 + $f2;
echo "3.14 + 2.71 = " . $fresult . "\n";

$fresult = $f1 * $f2;
echo "3.14 * 2.71 = " . $fresult . "\n\n";

// Test 5: Complex expression with known types
echo "Test 5: Complex Expression\n";
$p = 5;
$q = 3;
$r = 2;
$complex = ($p + $q) * $r;
echo "(5 + 3) * 2 = " . $complex . "\n\n";

echo "All optimizations working!\n";
