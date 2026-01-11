<?php
echo "=== Testing Compound Assignment Operators ===\n";

// Test +=
echo "\n=== += operator ===\n";
$a = 10;
echo "Before: \$a = ", $a, "\n";
$a += 5;
echo "After \$a += 5: ", $a, "\n";

// Test -=
echo "\n=== -= operator ===\n";
$b = 20;
echo "Before: \$b = ", $b, "\n";
$b -= 7;
echo "After \$b -= 7: ", $b, "\n";

// Test *=
echo "\n=== *= operator ===\n";
$c = 6;
echo "Before: \$c = ", $c, "\n";
$c *= 3;
echo "After \$c *= 3: ", $c, "\n";

// Test /=
echo "\n=== /= operator ===\n";
$d = 50;
echo "Before: \$d = ", $d, "\n";
$d /= 5;
echo "After \$d /= 5: ", $d, "\n";

// Test chaining
echo "\n=== Chaining operations ===\n";
$e = 100;
echo "Start: \$e = ", $e, "\n";
$e += 50;  // 150
echo "After \$e += 50: ", $e, "\n";
$e -= 30;  // 120
echo "After \$e -= 30: ", $e, "\n";
$e *= 2;   // 240
echo "After \$e *= 2: ", $e, "\n";
$e /= 3;   // 80
echo "After \$e /= 3: ", $e, "\n";

echo "\n=== All tests done! ===\n";
