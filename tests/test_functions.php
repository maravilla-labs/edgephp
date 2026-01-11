<?php
// PHASE 4: Test user-defined functions

// Simple function with no parameters
function greet() {
    return "Hello, World!";
}

echo greet();
echo "\n";

// Function with parameters
function add($a, $b) {
    return $a + $b;
}

$result = add(10, 20);
echo "10 + 20 = ";
echo $result;
echo "\n";

// Function with multiple operations
function double($x) {
    $doubled = $x * 2;
    return $doubled;
}

echo "double(5) = ";
echo double(5);
echo "\n";

// Function that calls another function
function addAndDouble($a, $b) {
    $sum = add($a, $b);
    return double($sum);
}

echo "addAndDouble(3, 7) = ";
echo addAndDouble(3, 7);
echo "\n";
?>
