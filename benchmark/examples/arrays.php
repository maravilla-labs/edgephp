<?php
// Array operations benchmark
echo "=== EdgePHP Array Operations Benchmark ===\n\n";

// Test 1: Array creation
echo "Test 1: Array Creation\n";
$numbers = array(1, 2, 3, 4, 5);
echo "Numeric array: count = " . count($numbers) . "\n";

$person = array("name" => "Alice", "age" => 25, "city" => "NYC");
echo "Associative array: count = " . count($person) . "\n";

$mixed = array(1, "hello", 3.14);
echo "Mixed array: count = " . count($mixed) . "\n\n";

// Test 2: Array access
echo "Test 2: Array Access\n";
echo "numbers[0] = " . $numbers[0] . "\n";
echo "numbers[4] = " . $numbers[4] . "\n";
echo "person['name'] = " . $person["name"] . "\n";
echo "person['age'] = " . $person["age"] . "\n\n";

// Test 3: Array assignment
echo "Test 3: Array Assignment\n";
$data = array();
$data[0] = "first";
$data[1] = "second";
$data["key"] = "value";
echo "After assignment: count = " . count($data) . "\n";
echo "data[0] = " . $data[0] . "\n";
echo "data['key'] = " . $data["key"] . "\n\n";

// Test 4: Array modification
echo "Test 4: Array Modification\n";
$numbers[0] = 10;
$numbers[2] = 30;
echo "Modified numbers: " . $numbers[0] . ", " . $numbers[1] . ", " . $numbers[2] . "\n";

$person["age"] = 30;
$person["country"] = "USA";
echo "Updated person: " . $person["name"] . " is " . $person["age"] . " from " . $person["country"] . "\n\n";

// Test 5: Large array operations
echo "Test 5: Large Array Operations\n";
$large = array();
for ($i = 0; $i < 100; $i++) {
    $large[$i] = $i * 2;
}
echo "Created array with 100 elements\n";
echo "large[0] = " . $large[0] . "\n";
echo "large[50] = " . $large[50] . "\n";
echo "large[99] = " . $large[99] . "\n";
echo "Total count: " . count($large) . "\n\n";

// Test 6: Associative array operations
echo "Test 6: Associative Array Operations\n";
$config = array(
    "host" => "localhost",
    "port" => 3306,
    "database" => "mydb",
    "username" => "user",
    "password" => "pass"
);
echo "Database config:\n";
echo "  Host: " . $config["host"] . ":" . $config["port"] . "\n";
echo "  Database: " . $config["database"] . "\n";
echo "  Credentials: " . $config["username"] . "/" . $config["password"] . "\n\n";

// Test 7: Nested arrays
echo "Test 7: Nested Arrays\n";
$users = array(
    array("id" => 1, "name" => "John"),
    array("id" => 2, "name" => "Jane"),
    array("id" => 3, "name" => "Bob")
);
echo "Users array with " . count($users) . " users\n";
echo "First user: " . $users[0]["name"] . " (ID: " . $users[0]["id"] . ")\n";
echo "Second user: " . $users[1]["name"] . " (ID: " . $users[1]["id"] . ")\n\n";

// Test 8: Performance test
echo "Test 8: Performance Test\n";
$perf = array();
$start = 0; // Would use microtime(true) in real PHP
for ($i = 0; $i < 1000; $i++) {
    $perf[$i] = $i * $i;
}
echo "Created and filled array with 1000 elements\n";
echo "Sample values: perf[0]=" . $perf[0] . ", perf[500]=" . $perf[500] . ", perf[999]=" . $perf[999] . "\n\n";

// Test 9: Empty array to hash table conversion
echo "Test 9: Empty Array Conversion\n";
$convert = array();
echo "Initial empty array count: " . count($convert) . "\n";
$convert["string_key"] = "This triggers conversion to hash table";
echo "After adding string key, count: " . count($convert) . "\n";
$convert[0] = "Can still add numeric keys";
$convert[1] = "Another numeric";
$convert["another"] = "And more strings";
echo "Final count: " . count($convert) . "\n";
echo "Values: " . $convert["string_key"] . ", " . $convert[0] . "\n\n";

// Test 10: Float display in arrays
echo "Test 10: Float Display\n";
$measurements = array(
    "pi" => 3.14159,
    "e" => 2.71828,
    "whole" => 5.0,
    "negative" => -1.5,
    "large" => 123456.789
);
echo "Mathematical constants:\n";
echo "  Pi = " . $measurements["pi"] . "\n";
echo "  e = " . $measurements["e"] . "\n";
echo "  Whole float = " . $measurements["whole"] . "\n";
echo "  Negative = " . $measurements["negative"] . "\n";
echo "  Large = " . $measurements["large"] . "\n\n";

// Test 11: Array assignment with conversion
echo "Test 11: Array Assignment Updates\n";
$test = array();
echo "Empty array created\n";
$test["key"] = "value"; // This converts to hash table
echo "After string key assignment: count = " . count($test) . "\n";
$result = $test; // Assignment should preserve the converted array
$result["key2"] = "value2";
echo "Copied array still works: count = " . count($result) . "\n";
echo "Original: " . $test["key"] . ", Copy: " . $result["key"] . ", " . $result["key2"] . "\n\n";

// Test 12: Key normalization (string->int conversion)
echo "Test 12: Key Normalization\n";
$norm = array();
$norm["123"] = "string key 123";
$norm[123] = "integer key 123";
echo "norm['123'] = " . $norm["123"] . " (should overwrite)\n";
echo "norm[123] = " . $norm[123] . " (same value)\n";
$norm["0123"] = "leading zero";
$norm["abc"] = "pure string";
echo "norm['0123'] = " . $norm["0123"] . " (stays string)\n";
echo "norm['abc'] = " . $norm["abc"] . "\n";
echo "Total count: " . count($norm) . " (should be 3)\n\n";

// Test 13: Array merge function
echo "Test 13: Array Merge\n";
$arr1 = array(1, 2, 3);
$arr2 = array(4, 5, 6);
$arr3 = array(7, 8);
$merged = array_merge($arr1, $arr2, $arr3);
echo "Merged array count: " . count($merged) . "\n";
echo "First three: " . $merged[0] . ", " . $merged[1] . ", " . $merged[2] . "\n";
echo "Last three: " . $merged[5] . ", " . $merged[6] . ", " . $merged[7] . "\n";

// Test with associative arrays
$colors1 = array("a" => "red", "b" => "green");
$colors2 = array("c" => "blue", "a" => "yellow");
$merged_colors = array_merge($colors1, $colors2);
echo "Merged colors count: " . count($merged_colors) . "\n";
echo "Values: " . $merged_colors[0] . ", " . $merged_colors[1] . ", " . $merged_colors[2] . ", " . $merged_colors[3] . "\n\n";

// Test 14: Array slice function
echo "Test 14: Array Slice\n";
$numbers = array(10, 20, 30, 40, 50, 60, 70, 80, 90, 100);
$slice1 = array_slice($numbers, 2, 4);
echo "Slice(2, 4) count: " . count($slice1) . "\n";
echo "Values: " . $slice1[0] . ", " . $slice1[1] . ", " . $slice1[2] . ", " . $slice1[3] . "\n";

// Test negative offset
$slice2 = array_slice($numbers, -3);
echo "Slice(-3) count: " . count($slice2) . "\n";
echo "Values: " . $slice2[0] . ", " . $slice2[1] . ", " . $slice2[2] . "\n";

// Test negative length
$slice3 = array_slice($numbers, 1, -7);
echo "Slice(1, -7) count: " . count($slice3) . "\n";
echo "Values: " . $slice3[0] . ", " . $slice3[1] . "\n\n";

// Test 15: Foreach loops
echo "Test 15: Foreach Loops\n";

// Simple foreach with values only
$fruits = array("apple", "banana", "orange", "grape");
echo "Iterating over fruits:\n";
foreach ($fruits as $fruit) {
    echo "  - " . $fruit . "\n";
}

// Foreach with key => value
echo "\nIterating with indices:\n";
foreach ($fruits as $index => $fruit) {
    echo "  [" . $index . "] => " . $fruit . "\n";
}

// Foreach with numeric array
$squares = array();
for ($i = 1; $i <= 5; $i++) {
    $squares[$i - 1] = $i * $i;
}
echo "\nSquares of numbers:\n";
foreach ($squares as $num => $square) {
    $num_plus_one = $num + 1;
    echo "  " . $num_plus_one . " squared = " . $square . "\n";
}

// Foreach with break
echo "\nForeach with break (stop at 20):\n";
$numbers = array(10, 15, 20, 25, 30);
foreach ($numbers as $n) {
    if ($n > 20) {
        break;
    }
    echo "  Number: " . $n . "\n";
}

// Foreach with continue
echo "\nForeach with continue (skip even numbers):\n";
$range = array(1, 2, 3, 4, 5, 6, 7, 8);
foreach ($range as $r) {
    if ($r % 2 == 0) {
        continue;
    }
    echo "  Odd: " . $r . "\n";
}

// Nested foreach loops
echo "\nNested foreach loops:\n";
$matrix = array(
    array(1, 2, 3),
    array(4, 5, 6),
    array(7, 8, 9)
);
foreach ($matrix as $row_idx => $row) {
    echo "Row " . $row_idx . ": ";
    foreach ($row as $col_idx => $val) {
        echo $val;
        if ($col_idx < 2) echo ", ";
    }
    echo "\n";
}
echo "\n";

echo "✅ All array benchmarks completed successfully!\n";
echo "📊 Arrays are working efficiently in EdgePHP!";