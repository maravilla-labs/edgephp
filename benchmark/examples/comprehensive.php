<?php
// Comprehensive test
$x = 15;
$y = 3;
echo "Variables: x=", $x, ", y=", $y, "\n\n";

// Arithmetic
echo "Arithmetic Operations:\n";
echo "x + y = ", ($x + $y), "\n";
echo "x - y = ", ($x - $y), "\n";
echo "x * y = ", ($x * $y), "\n";
echo "x / y = ", ($x / $y), "\n\n";

// Comparisons
echo "Comparisons:\n";
echo "x == y: ", ($x == $y), "\n";
echo "x != y: ", ($x != $y), "\n";
echo "x > y: ", ($x > $y), "\n";
echo "x < y: ", ($x < $y), "\n\n";

// Strings
$greeting = "Hello";
$name = "World";
echo "String Operations:\n";
echo "greeting = ", $greeting, "\n";
echo "name = ", $name, "\n";
echo "Combined: ", $greeting, " ", $name, "!\n\n";

echo "All features working!\n";