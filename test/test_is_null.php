<?php
// Test is_null function
$x = null;
$y = 42;
$z = "hello";

echo "is_null(null): ", is_null($x), "\n";
echo "is_null(42): ", is_null($y), "\n";
echo "is_null('hello'): ", is_null($z), "\n";

// Test with literal
echo "is_null(null literal): ", is_null(null), "\n";