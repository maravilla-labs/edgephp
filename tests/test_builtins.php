<?php
// Test Built-in Functions

echo "=== Type Checking Functions ===\n";

$int_var = 42;
$float_var = 3.14;
$string_var = "hello";
$bool_var = true;
$array_var = [1, 2, 3];
$null_var = null;

echo "is_int(42): ", is_int($int_var), "\n";
echo "is_float(3.14): ", is_float($float_var), "\n";
echo "is_string('hello'): ", is_string($string_var), "\n";
echo "is_bool(true): ", is_bool($bool_var), "\n";
echo "is_array([1,2,3]): ", is_array($array_var), "\n";
echo "is_null(null): ", is_null($null_var), "\n";

echo "\n=== Array Functions ===\n";

$arr = [1, 2, 3];
echo "Original array count: ", count($arr), "\n";

array_push($arr, 4);
echo "After push: ", count($arr), "\n";

$keys = array_keys($arr);
echo "Array keys count: ", count($keys), "\n";

$values = array_values($arr);
echo "Array values count: ", count($values), "\n";

$arr2 = [5, 6];
$merged = array_merge($arr, $arr2);
echo "Merged array count: ", count($merged), "\n";

echo "\n=== Utility Functions ===\n";

$test_var = 42;
echo "isset(\$test_var): ", isset($test_var), "\n";
echo "isset(\$undefined): ", isset($undefined), "\n";

echo "empty(0): ", empty(0), "\n";
echo "empty(42): ", empty(42), "\n";
echo "empty(''): ", empty(''), "\n";
echo "empty('hello'): ", empty('hello'), "\n";

echo "\n=== All Built-in Functions Working! ===\n";
