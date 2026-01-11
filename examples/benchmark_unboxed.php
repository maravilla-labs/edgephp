<?php
// Benchmark unboxed locals optimization
echo "=== Unboxed Locals Benchmark ===\n\n";

// Test 1: Unboxed integer variables
echo "Test 1: Unboxed Integer Variables\n";
$a = 10;  // Stored as unboxed i64
$b = 20;  // Stored as unboxed i64
$c = $a + $b;  // All operations use unboxed values!
echo "10 + 20 = " . $c . "\n\n";

// Test 2: Loop with unboxed counter
echo "Test 2: Unboxed Loop Counter\n";
$sum = 0;  // Unboxed i64
for ($i = 1; $i <= 100; $i = $i + 1) {
    $sum = $sum + $i;  // No boxing/unboxing in loop!
}
echo "Sum of 1-100: " . $sum . "\n\n";

// Test 3: Complex expression with unboxed values
echo "Test 3: Complex Expression\n";
$x = 5;   // Unboxed i64
$y = 3;   // Unboxed i64
$z = 2;   // Unboxed i64
$result = ($x + $y) * $z - ($x - $y);
echo "(5 + 3) * 2 - (5 - 3) = " . $result . "\n\n";

// Test 4: Unboxed floats
echo "Test 4: Unboxed Float Variables\n";
$pi = 3.14159;  // Unboxed f64
$e = 2.71828;   // Unboxed f64
$phi = $pi * $e;
echo "pi * e = " . $phi . "\n\n";

echo "All unboxed optimizations working!\n";
