<?php
// Test strength reduction optimizations
echo "Strength Reduction Test\n\n";

// Test 1: x * 2 should become x + x
$x = 50;
$result1 = $x * 2;
echo "x * 2 = " . $result1 . "\n";

// Test 2: x * 1 should become x
$result2 = $x * 1;
echo "x * 1 = " . $result2 . "\n";

// Test 3: x * 0 should become 0
$result3 = $x * 0;
echo "x * 0 = " . $result3 . "\n";

// Test 4: x + 0 should become x
$result4 = $x + 0;
echo "x + 0 = " . $result4 . "\n";

// Test 5: x - 0 should become x
$result5 = $x - 0;
echo "x - 0 = " . $result5 . "\n";

// Test 6: x / 1 should become x
$result6 = $x / 1;
echo "x / 1 = " . $result6 . "\n";

// Test 7: Performance test - multiplication by 2 in loop
$sum = 0;
for ($i = 0; $i < 1000; $i = $i + 1) {
    $sum = $sum + ($i * 2);
}
echo "\nLoop with *2: sum = " . $sum . "\n";

echo "\nAll tests passed!\n";
