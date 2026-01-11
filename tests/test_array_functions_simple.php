<?php
echo "=== Testing Array Functions (Simple) ===\n";

// Test 1: array_pop
echo "\n=== Test 1: array_pop() ===\n";
$arr1 = [1, 2, 3];
echo "Before pop: count = ", count($arr1), "\n";
$last = array_pop($arr1);
echo "Popped: ", $last, "\n";

// Test 2: in_array
echo "\n=== Test 2: in_array() ===\n";
$arr2 = [1, 2, 3];
$result = in_array(2, $arr2);
echo "in_array(2, [1,2,3]): ", $result, "\n";

echo "\n=== Done! ===\n";
