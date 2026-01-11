<?php
// This code should benefit MASSIVELY from escape analysis
// All loop variables are non-escaping until the final echo

echo "Testing escape analysis performance...\n";

// Test 1: Sum loop - all variables non-escaping until echo
$sum = 0;
for ($i = 0; $i < 1000; $i = $i + 1) {
    // $temp doesn't escape - should stay unboxed i64!
    $temp = $i * 2;
    // $sum doesn't escape in loop - should stay unboxed i64!
    $sum = $sum + $temp;
}
// Only here does $sum escape (output)
echo "Sum: " . $sum . "\n";

// Test 2: Fibonacci - perfect case for escape analysis
$a = 0;
$b = 1;
for ($j = 0; $j < 100; $j = $j + 1) {
    // All these are non-escaping in the loop!
    $temp = $a;
    $a = $b;
    $b = $temp + $b;
}
// Only here does $a escape
echo "Fibonacci(100): " . $a . "\n";

// Test 3: Multiple operations - non-escaping chain
$x = 5;
$y = 10;
$z = $x + $y;        // $z doesn't escape initially
$result = $z * 3;    // $result doesn't escape initially
// Only here do they escape
echo "Result: " . $result . "\n";

echo "All tests complete!\n";
