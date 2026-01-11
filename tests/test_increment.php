<?php
echo "=== Testing Increment/Decrement Operators ===\n";

// Test pre-increment
echo "\n=== Pre-increment (++\$x) ===\n";
$a = 5;
echo "Before: \$a = ", $a, "\n";
echo "++\$a = ", ++$a, "\n";
echo "After: \$a = ", $a, "\n";

// Test post-increment
echo "\n=== Post-increment (\$x++) ===\n";
$b = 5;
echo "Before: \$b = ", $b, "\n";
echo "\$b++ = ", $b++, "\n";
echo "After: \$b = ", $b, "\n";

// Test pre-decrement
echo "\n=== Pre-decrement (--\$x) ===\n";
$c = 5;
echo "Before: \$c = ", $c, "\n";
echo "--\$c = ", --$c, "\n";
echo "After: \$c = ", $c, "\n";

// Test post-decrement
echo "\n=== Post-decrement (\$x--) ===\n";
$d = 5;
echo "Before: \$d = ", $d, "\n";
echo "\$d-- = ", $d--, "\n";
echo "After: \$d = ", $d, "\n";

// Test in expressions
echo "\n=== In expressions ===\n";
$e = 10;
echo "\$e = 10\n";
echo "\$e++ + 5 = ", $e++ + 5, "\n";
echo "After: \$e = ", $e, "\n";
echo "++\$e + 5 = ", ++$e + 5, "\n";
echo "After: \$e = ", $e, "\n";

echo "\n=== All tests done! ===\n";
