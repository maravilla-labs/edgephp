<?php
echo "=== Testing Array Functions ===\n";

// Test 1: array_pop
echo "\n=== Test 1: array_pop() ===\n";
$arr1 = [1, 2, 3, 4, 5];
echo "Array: [1, 2, 3, 4, 5]\n";
$last = array_pop($arr1);
echo "Popped: ", $last, "\n";
echo "After pop, first element: ", $arr1[0], "\n";

// Test 2: array_shift
echo "\n=== Test 2: array_shift() ===\n";
$arr2 = [10, 20, 30, 40];
echo "Array: [10, 20, 30, 40]\n";
$first = array_shift($arr2);
echo "Shifted: ", $first, "\n";
echo "After shift, element at 0: ", $arr2[0], "\n";

// Test 3: in_array
echo "\n=== Test 3: in_array() ===\n";
$arr3 = [1, 2, 3, 4, 5];
echo "Array: [1, 2, 3, 4, 5]\n";
echo "in_array(3, arr): ", in_array(3, $arr3), " (1=true)\n";
echo "in_array(10, arr): ", in_array(10, $arr3), " (empty=false)\n";

// Test 4: in_array with strings
echo "\n=== Test 4: in_array() with strings ===\n";
$fruits = ["apple", "banana", "orange"];
echo "Array: [apple, banana, orange]\n";
echo "in_array('banana', fruits): ", in_array("banana", $fruits), " (1=true)\n";
echo "in_array('grape', fruits): ", in_array("grape", $fruits), " (empty=false)\n";

// Test 5: array_push (already exists, testing)
echo "\n=== Test 5: array_push() ===\n";
$arr5 = [1, 2, 3];
echo "Array: [1, 2, 3]\n";
array_push($arr5, 4);
echo "After push(4), count: ", count($arr5), "\n";

echo "\n=== All tests done! ===\n";
