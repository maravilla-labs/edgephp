<?php
$start = microtime(true);

// Test 1: String key optimization (10,000 ops)
$data = ["x" => 100, "y" => 200, "z" => 300];
$i = 0;
while ($i < 10000) {
    $x = $data["x"];
    $y = $data["y"];
    $z = $data["z"];
    $i = $i + 1;
}

// Test 2: Integer key optimization (10,000 ops)
$arr = [10, 20, 30, 40, 50];
$i = 0;
while ($i < 10000) {
    $a = $arr[0];
    $b = $arr[1];
    $c = $arr[2];
    $i = $i + 1;
}

// Test 3: Mixed operations (5,000 ops each type)
$mixed = ["name" => "Test", "id" => 123, 0 => "first", 1 => "second"];
$i = 0;
while ($i < 5000) {
    $name = $mixed["name"];
    $id = $mixed["id"];
    $first = $mixed[0];
    $second = $mixed[1];
    $i = $i + 1;
}

// Test 4: Array assignment with both key types (5,000 ops)
$result = ["count" => 0, 0 => 0];
$i = 0;
while ($i < 5000) {
    $result["count"] = $i;
    $result[0] = $i;
    $i = $i + 1;
}

$end = microtime(true);
$elapsed = ($end - $start) * 1000;

echo "Native PHP - Phase 3C Complete Test\n";
echo "====================================\n";
echo "Total operations: 90,000\n";
echo "Execution Time: " . number_format($elapsed, 3) . "ms\n";
echo "Throughput: " . number_format(90000 / $elapsed * 1000, 2) . " ops/second\n";
echo "\nFinal values:\n";
echo "  result[\"count\"]: " . $result["count"] . "\n";
echo "  result[0]: " . $result[0] . "\n";
?>