<?php
// Array Operations Benchmark
echo "=== Array Operations Benchmark ===\n\n";

// Test 1: Array creation and sequential access
echo "Test 1: Sequential Array Access\n";
$arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
$sum = 0;
for ($i = 0; $i < 10; $i = $i + 1) {
    $sum = $sum + $arr[$i];
}
echo "Sum of array elements: " . $sum . "\n\n";

// Test 2: Array building in loop
echo "Test 2: Array Building in Loop\n";
$arr = [];
for ($i = 0; $i < 20; $i = $i + 1) {
    $arr[$i] = $i * 2;
}
echo "Built array with 20 elements\n";
echo "arr[10] = " . $arr[10] . "\n\n";

// Test 3: Associative array with string keys
echo "Test 3: Associative Array Operations\n";
$person = [
    "name" => "John",
    "age" => 30,
    "city" => "New York"
];
echo "Name: " . $person["name"] . "\n";
echo "Age: " . $person["age"] . "\n";
echo "City: " . $person["city"] . "\n\n";

// Test 4: Associative array in loop
echo "Test 4: Associative Array in Loop\n";
$data = [];
for ($i = 0; $i < 10; $i = $i + 1) {
    $key = "key";
    $data[$key] = $i * 3;
}
echo "data[\"key\"] = " . $data["key"] . "\n\n";

// Test 5: Mixed numeric and string keys
echo "Test 5: Mixed Key Types\n";
$mixed = [];
$mixed[0] = "first";
$mixed["name"] = "Alice";
$mixed[1] = "second";
$mixed["value"] = 42;
echo "mixed[0] = " . $mixed[0] . "\n";
echo "mixed[\"name\"] = " . $mixed["name"] . "\n";
echo "mixed[1] = " . $mixed[1] . "\n";
echo "mixed[\"value\"] = " . $mixed["value"] . "\n\n";

echo "Array benchmark complete!\n";
