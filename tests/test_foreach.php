<?php
// Test foreach loop functionality
echo "=== Testing foreach loops ===\n\n";

// Test 1: Simple foreach with values only
echo "Test 1: Simple foreach (values only)\n";
$numbers = [1, 2, 3, 4, 5];
foreach ($numbers as $num) {
    echo "Value: ", $num, "\n";
}
echo "\n";

// Test 2: Foreach with key => value
echo "Test 2: Foreach with key => value\n";
$colors = ["red", "green", "blue"];
foreach ($colors as $index => $color) {
    echo "Index ", $index, ": ", $color, "\n";
}
echo "\n";

// Test 3: Associative array (once hash arrays are supported)
echo "Test 3: Simple associative array simulation\n";
$person = array();
$person[0] = "John";
$person[1] = "Doe";
$person[2] = 30;
foreach ($person as $key => $value) {
    echo "Key ", $key, ": ", $value, "\n";
}
echo "\n";

// Test 4: Empty array
echo "Test 4: Empty array\n";
$empty = array();
foreach ($empty as $item) {
    echo "This should not print\n";
}
echo "Empty array test passed\n\n";

// Test 5: Break in foreach
echo "Test 5: Break in foreach\n";
$nums = [1, 2, 3, 4, 5];
foreach ($nums as $n) {
    if ($n == 3) {
        break;
    }
    echo "Number: ", $n, "\n";
}
echo "After break\n\n";

// Test 6: Continue in foreach
echo "Test 6: Continue in foreach\n";
$vals = [1, 2, 3, 4, 5];
foreach ($vals as $v) {
    if ($v == 3) {
        continue;
    }
    echo "Value: ", $v, "\n";
}
echo "After continue test\n\n";

echo "✅ All foreach tests completed!\n";