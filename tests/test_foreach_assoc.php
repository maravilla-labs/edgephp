<?php
// Test foreach with associative arrays
echo "=== Foreach with Associative Arrays ===\n\n";

// Test 1: Simple associative array
echo "Test 1: Simple associative array\n";
$person = array();
$person["name"] = "John Doe";
$person["age"] = 30;
$person["city"] = "New York";
$person["email"] = "john@example.com";

echo "Person details:\n";
foreach ($person as $key => $value) {
    echo "  " . $key . ": " . $value . "\n";
}
echo "\n";

// Test 2: Mixed keys (numeric and string)
echo "Test 2: Mixed numeric and string keys\n";
$mixed = array();
$mixed[0] = "first";
$mixed["key1"] = "value1";
$mixed[1] = "second";
$mixed["key2"] = "value2";

foreach ($mixed as $k => $v) {
    echo "  [" . $k . "] => " . $v . "\n";
}
echo "\n";

// Test 3: Without braces (single statement)
echo "Test 3: Single statement foreach\n";
$items = ["apple", "banana", "orange"];
foreach ($items as $item)
    echo "- " . $item . "\n";
echo "\n";

// Test 4: Nested foreach with mixed arrays
echo "Test 4: Nested foreach\n";
$data = array();
$data["users"] = array();
$data["users"][0] = "Alice";
$data["users"][1] = "Bob";
$data["products"] = array();
$data["products"][0] = "Laptop";
$data["products"][1] = "Phone";

foreach ($data as $category => $items) {
    echo $category . ":\n";
    foreach ($items as $idx => $item) {
        echo "  [" . $idx . "] " . $item . "\n";
    }
}

echo "\n✅ Associative array foreach completed!\n";