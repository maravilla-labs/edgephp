<?php
// Comprehensive test of all features
echo "=== EdgePHP Complete Feature Test ===\n";

// Variables
$x = 15;
$y = 3;
echo "Variables: x=" . $x . ", y=" . $y . "\n\n";

// Arithmetic
echo "Arithmetic Operations:\n";
echo "x + y = " . ($x + $y) . "\n";
echo "x - y = " . ($x - $y) . "\n";
echo "x * y = " . ($x * $y) . "\n";
echo "x / y = " . ($x / $y) . "\n\n";

// Comparisons
echo "Comparisons:\n";
echo "x == y: " . ($x == $y) . "\n";
echo "x != y: " . ($x != $y) . "\n";
echo "x > y: " . ($x > $y) . "\n";
echo "x < y: " . ($x < $y) . "\n\n";

// Strings
$greeting = "Hello";
$name = "World";
echo "String Operations:\n";
echo "greeting = " . $greeting . "\n";
echo "name = " . $name . "\n";
echo "Combined: " . $greeting . " " . $name . "!\n\n";

// Complex expressions
$result = ($x > $y) == 1;
echo "Complex: (x > y) == 1 is " . $result . "\n\n";

echo "🎉 All features working! 🎉\n";