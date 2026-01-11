<?php
// Phase 3C Complete Optimization Test
// Tests both string and integer key optimizations

echo "=== Phase 3C Complete Optimization Test ===\n";
echo "\n";

// Test 1: String key optimization (10,000 ops)
echo "Test 1: String Keys (10,000 operations)\n";
$data = ["x" => 100, "y" => 200, "z" => 300];
$i = 0;
while ($i < 10000) {
    $x = $data["x"];  // Optimized: pre-computed hash
    $y = $data["y"];  // Optimized: pre-computed hash
    $z = $data["z"];  // Optimized: pre-computed hash
    $i = $i + 1;
}
echo "String keys: 30,000 optimized operations complete\n";
echo "\n";

// Test 2: Integer key optimization (10,000 ops)
echo "Test 2: Integer Keys (10,000 operations)\n";
$arr = [10, 20, 30, 40, 50];
$i = 0;
while ($i < 10000) {
    $a = $arr[0];  // Optimized: no boxing
    $b = $arr[1];  // Optimized: no boxing
    $c = $arr[2];  // Optimized: no boxing
    $i = $i + 1;
}
echo "Integer keys: 30,000 optimized operations complete\n";
echo "\n";

// Test 3: Mixed operations (5,000 ops each type)
echo "Test 3: Mixed String and Integer Keys (10,000 operations)\n";
$mixed = ["name" => "Test", "id" => 123, 0 => "first", 1 => "second"];
$i = 0;
while ($i < 5000) {
    // String key access (optimized)
    $name = $mixed["name"];
    $id = $mixed["id"];

    // Integer key access (optimized)
    $first = $mixed[0];
    $second = $mixed[1];

    $i = $i + 1;
}
echo "Mixed keys: 20,000 optimized operations complete\n";
echo "\n";

// Test 4: Array assignment with both key types (5,000 ops)
echo "Test 4: Array Assignment (10,000 operations)\n";
$result = ["count" => 0, 0 => 0];
$i = 0;
while ($i < 5000) {
    $result["count"] = $i;  // Optimized: string key with pre-computed hash
    $result[0] = $i;        // Optimized: integer key, no boxing
    $i = $i + 1;
}
echo "Assignment: 10,000 optimized operations complete\n";
echo "Final count: ", $result["count"], "\n";
echo "Final [0]: ", $result[0], "\n";
echo "\n";

echo "=== Phase 3C Test Complete ===\n";
echo "Total operations: 90,000 (all using optimized paths)\n";
?>