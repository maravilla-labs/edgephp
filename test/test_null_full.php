<?php
echo "=== Complete NULL Type Testing ===\n\n";

// 1. Null creation and display
$x = null;
echo "1. Null to string: '", $x, "' (should be empty)\n";

// 2. Null in arithmetic (treated as 0)
echo "\n2. Null in arithmetic:\n";
echo "   null + 10 = ", ($x + 10), "\n";
echo "   20 - null = ", (20 - $x), "\n";
echo "   null * 5 = ", ($x * 5), "\n";
echo "   null / 1 = ", ($x / 1), "\n";

// 3. Null in string concatenation
echo "\n3. String concatenation:\n";
echo "   'Hello' . null . 'World' = '", ("Hello" . $x . "World"), "'\n";

// 4. Null in comparisons (treated as 0/false)
echo "\n4. Null comparisons:\n";
echo "   null > 0: ", ($x > 0), "\n";
echo "   null < 1: ", ($x < 1), "\n";
echo "   0 > null: ", (0 > $x), "\n";

// 5. is_null() function
echo "\n5. is_null() function:\n";
echo "   is_null(null): ", is_null($x), "\n";
echo "   is_null(0): ", is_null(0), "\n";
echo "   is_null(''): ", is_null(""), "\n";
echo "   is_null(false): ", is_null(false), "\n";

// 6. Null assignment and reassignment
echo "\n6. Variable operations:\n";
$y = null;
echo "   After \$y = null, is_null(\$y): ", is_null($y), "\n";
$y = 42;
echo "   After \$y = 42, is_null(\$y): ", is_null($y), "\n";

echo "\n✅ All null tests completed!\n";