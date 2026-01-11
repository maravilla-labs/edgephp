<?php
// Comprehensive null test
echo "=== NULL Type Testing ===\n";

// Basic null
$x = null;
echo "null value: '", $x, "'\n";
echo "null concatenated: 'prefix" . $x . "suffix'\n";

// Null in arithmetic (should be treated as 0)
echo "\nNull in arithmetic:\n";
echo "null + 5 = ", ($x + 5), "\n";
echo "10 - null = ", (10 - $x), "\n";
echo "null * 3 = ", ($x * 3), "\n";
echo "15 / null = ", (15 / $x), "\n"; // This will cause division by zero

// Null in comparisons
echo "\nNull comparisons:\n";
echo "null > 0: ", ($x > 0), "\n";
echo "null < 1: ", ($x < 1), "\n";
echo "null > -1: ", ($x > -1), "\n";