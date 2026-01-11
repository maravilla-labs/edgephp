<?php
// Array operations benchmark (without loops)
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

// Test 5: Building arrays dynamically
echo "Test 5: Building Arrays\n";
$build = array();
$build[0] = 100;
$build[1] = 200;
$build[2] = 300;
$build[3] = 400;
$build[4] = 500;
echo "Built array: count = " . count($build) . "\n";
echo "Values: " . $build[0] . ", " . $build[1] . ", " . $build[2] . ", " . $build[3] . ", " . $build[4] . "\n\n";

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

// Test 7: Mixed key arrays
echo "Test 7: Mixed Key Arrays\n";
$mixed_keys = array();
$mixed_keys[0] = "numeric zero";
$mixed_keys["one"] = "string one";
$mixed_keys[2] = "numeric two";
$mixed_keys["three"] = "string three";
echo "Mixed keys array: count = " . count($mixed_keys) . "\n";
echo "mixed_keys[0] = " . $mixed_keys[0] . "\n";
echo "mixed_keys['one'] = " . $mixed_keys["one"] . "\n";
echo "mixed_keys[2] = " . $mixed_keys[2] . "\n";
echo "mixed_keys['three'] = " . $mixed_keys["three"] . "\n\n";

// Test 8: Key normalization  
echo "Test 8: Key Normalization\n";
$keys = array();
$keys["789"] = "string 789";
$keys[789] = "int 789 (overwrites)";
$keys["0456"] = "leading zero";
$keys["xyz"] = "pure string";
echo "keys['789'] = " . $keys["789"] . "\n";
echo "keys[789] = " . $keys[789] . "\n";
echo "keys['0456'] = " . $keys["0456"] . "\n";
echo "keys['xyz'] = " . $keys["xyz"] . "\n";
echo "Count: " . count($keys) . " (should be 3)\n\n";

echo "✅ All array benchmarks completed successfully!\n";
echo "📊 Arrays are working efficiently in EdgePHP!";