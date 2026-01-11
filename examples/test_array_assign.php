<?php
echo "Array assignment test\n";

// Test: Array assignment in loop
$arr = [];
for ($i = 0; $i < 5; $i = $i + 1) {
    $arr[$i] = $i * 2;
}

echo "arr[2] = " . $arr[2] . "\n";
echo "Done\n";
