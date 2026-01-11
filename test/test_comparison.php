<?php
// Complete comparison operators test
echo "=== Comparison Operators Test ===\n";

$a = 150;
$b = 55;
$c = 10;

echo "Values: a = ";
echo $a;
echo ", b = ";
echo $b;
echo ", c = ";
echo $c;
echo "\n\n";

// Equality tests
echo "Equality Tests:\n";
echo "a == b: ";
echo $a == $b;
echo " (false = empty)\n";
echo "a == c: ";
echo $a == $c;
echo " (true = 1)\n";

echo "a != b: ";
echo $a != $b;
echo " (true = 1)\n";
echo "a != c: ";
echo $a != $c;
echo " (false = empty)\n\n";

// Comparison tests
echo "Comparison Tests:\n";
echo "a > b: ";
echo $a > $b;
echo " (true = 1)\n";
echo "b > a: ";
echo $b > $a;
echo " (false = empty)\n";
echo "a < b: ";
echo $a < $b;
echo " (false = empty)\n";
echo "b < a: ";
echo $b < $a;
echo " (true = 1)\n";