<?php
// Phase 3C Array Benchmark - Tests literal string key optimization
// This benchmark exercises the optimized array access paths

echo "=== Array Benchmark ===\n";
echo "\n";

echo "Test 1: Array Creation with Literal String Keys\n";
$user = ["name" => "John", "age" => 30, "email" => "john@example.com"];
echo "User created: ", $user["name"], "\n";
echo "\n";

echo "Test 2: Multiple Array Access with Literal Keys (1000 iterations)\n";
$data = ["x" => 10, "y" => 20, "z" => 30];
$i = 0;
while ($i < 1000) {
    $x = $data["x"];
    $y = $data["y"];
    $z = $data["z"];
    $sum = $x + $y + $z;
    $i = $i + 1;
}
echo "Sum after 1000 iterations: ", $sum, "\n";
echo "\n";

echo "Test 3: Array Assignment with Literal Keys (500 iterations)\n";
$config = ["width" => 800, "height" => 600, "fps" => 60];
$i = 0;
while ($i < 500) {
    $config["width"] = 1920;
    $config["height"] = 1080;
    $config["fps"] = 144;
    $i = $i + 1;
}
echo "Config updated: ", $config["width"], "x", $config["height"], " @ ", $config["fps"], " fps\n";
echo "\n";

echo "Test 4: Mixed Array Operations (200 iterations)\n";
$record = ["id" => 1, "name" => "Item", "price" => 100];
$i = 0;
while ($i < 200) {
    $id = $record["id"];
    $name = $record["name"];
    $price = $record["price"];
    $record["id"] = $id + 1;
    $record["price"] = $price + 10;
    $i = $i + 1;
}
echo "Record after updates: ID=", $record["id"], ", Price=", $record["price"], "\n";
echo "\n";

echo "Array benchmark complete!\n";
?>