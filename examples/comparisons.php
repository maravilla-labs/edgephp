<?php
// Complete comparison operators test
echo "=== Comparison Operators Test ===\n";

$a = 10;
$b = 5;
$c = 10;

echo "Values: a = " . $a . ", b = " . $b . ", c = " . $c . "\n\n";

// Equality tests
echo "Equality Tests:\n";
echo "a == b: " . ($a == $b) . " (false = empty)\n";
echo "a == c: " . ($a == $c) . " (true = 1)\n";

echo "a != b: " . ($a != $b) . " (true = 1)\n";
echo "a != c: " . ($a != $c) . " (false = empty)\n\n";

// Comparison tests
echo "Comparison Tests:\n";
echo "a > b: " . ($a > $b) . " (true = 1)\n";
echo "b > a: " . ($b > $a) . " (false = empty)\n";
echo "a < b: " . ($a < $b) . " (false = empty)\n";
echo "b < a: " . ($b < $a) . " (true = 1)\n";

echo "a >= c: " . ($a >= $c) . " (true = 1)\n";
echo "b <= a: " . ($b <= $a) . " (true = 1)\n";