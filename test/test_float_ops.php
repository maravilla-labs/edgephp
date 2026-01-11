<?php
// Test more float operations
$x = 10.5;
$y = 3.0;

echo "Float arithmetic:\n";
echo "10.5 + 3.0 = ", ($x + $y), "\n";
echo "10.5 - 3.0 = ", ($x - $y), "\n";
echo "10.5 * 3.0 = ", ($x * $y), "\n";
echo "10.5 / 3.0 = ", ($x / $y), "\n";

// Test float comparisons
echo "\nComparisons:\n";
echo "10.5 > 3.0: ", ($x > $y), "\n";
echo "10.5 < 3.0: ", ($x < $y), "\n";

// Test float to string concat
$msg = "Pi is approximately " . 3.14159;
echo "\n", $msg, "\n";
