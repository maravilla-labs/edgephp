<?php
// Simple escape analysis test (no functions)

echo "=== Escape Analysis Test ===\n";

// Test 1: Variables that DON'T escape
$a = 10;      // Literal - doesn't escape
$b = 20;      // Literal - doesn't escape
$c = $a + $b; // Computation - doesn't escape initially

// Test 2: Loop with non-escaping temps
$sum = 0;
for ($i = 0; $i < 10; $i = $i + 1) {
    $temp = $i * 2;  // Temporary - doesn't escape
    $sum = $sum + $temp;
}

// Test 3: Now $sum escapes (output)
echo "Sum = " . $sum . "\n";

// Test 4: Variable propagation
$x = 5;
$y = $x;      // Propagates from $x
$z = $y;      // Propagates from $y
echo "Z = " . $z . "\n";  // Makes all escape

echo "Test complete!\n";
