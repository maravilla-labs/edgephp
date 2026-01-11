<?php
// Comprehensive Built-in Functions Test

echo "=== Type Checking Functions ===\n";
echo "is_int(42): ", is_int(42), "\n";
echo "is_float(3.14): ", is_float(3.14), "\n";
echo "is_string('hello'): ", is_string('hello'), "\n";
echo "is_bool(true): ", is_bool(true), "\n";
echo "is_array([1,2,3]): ", is_array([1,2,3]), "\n";
echo "is_null(null): ", is_null(null), "\n";

echo "\n=== Array Functions ===\n";
$arr = [10, 20, 30];
echo "Array count: ", count($arr), "\n";

array_push($arr, 40);
echo "After push: ", count($arr), "\n";

$keys = array_keys($arr);
echo "Array keys count: ", count($keys), "\n";

$values = array_values($arr);
echo "Array values count: ", count($values), "\n";

echo "\n=== String Functions ===\n";
$str = "hello";
echo "strlen('hello'): ", strlen($str), "\n";
echo "strlen('world!'): ", strlen("world!"), "\n";

echo "\n=== Math Functions ===\n";
echo "abs(-42): ", abs(-42), "\n";
echo "abs(42): ", abs(42), "\n";
echo "min(5, 3, 8): ", min(5, 3, 8), "\n";
echo "max(5, 3, 8): ", max(5, 3, 8), "\n";
echo "floor(3.7): ", floor(3.7), "\n";
echo "ceil(3.2): ", ceil(3.2), "\n";
echo "round(3.5): ", round(3.5), "\n";
echo "sqrt(16.0): ", sqrt(16.0), "\n";

echo "\n=== Utility Functions ===\n";
$var = 100;
echo "isset(\$var): ", isset($var), "\n";
echo "empty(0): ", empty(0), "\n";
echo "empty(100): ", empty(100), "\n";

echo "\n=== All Built-in Functions Test Complete! ===\n";
