<?php
// Test comparison operators with integers and floats
$a = 10;
$b = 5;
$c = 10;

// Integer comparisons
echo "Integer comparisons:\n";
echo "10 > 5: ", ($a > $b), "\n";
echo "5 < 10: ", ($b < $a), "\n";
echo "10 == 10: ", ($a == $c), "\n";
echo "10 != 5: ", ($a != $b), "\n";

// Float comparisons
$x = 10.5;
$y = 3.14;

echo "\nFloat comparisons:\n";
echo "10.5 > 3.14: ", ($x > $y), "\n";
echo "3.14 < 10.5: ", ($y < $x), "\n";

// Mixed int/float comparisons
echo "\nMixed comparisons:\n";
echo "10 > 3.14: ", ($a > $y), "\n";
echo "3.14 < 10: ", ($y < $a), "\n";