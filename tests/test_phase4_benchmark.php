<?php
// PHASE 4 Performance Benchmark: Functions
// Tests function call overhead, parameter passing, and recursion

// Test 1: Simple function calls (no parameters) - 10,000 iterations
function simple() {
    return 42;
}

$i = 0;
while ($i < 10000) {
    $result = simple();
    $i = $i + 1;
}

// Test 2: Function with parameters - 10,000 iterations
function add($a, $b) {
    return $a + $b;
}

$i = 0;
while ($i < 10000) {
    $result = add($i, 100);
    $i = $i + 1;
}

// Test 3: Function with multiple operations - 5,000 iterations
function calculate($x, $y) {
    $sum = $x + $y;
    $product = $x * $y;
    $difference = $x - $y;
    return $sum + $product + $difference;
}

$i = 0;
while ($i < 5000) {
    $result = calculate($i, 10);
    $i = $i + 1;
}

// Test 4: Nested function calls - 5,000 iterations
function inner($x) {
    return $x * 2;
}

function outer($x) {
    return inner($x) + inner($x);
}

$i = 0;
while ($i < 5000) {
    $result = outer($i);
    $i = $i + 1;
}

// Test 5: Recursive function (Fibonacci) - Calculate fib(15) 100 times
function fib($n) {
    if ($n <= 1) {
        return $n;
    }
    return fib($n - 1) + fib($n - 2);
}

$i = 0;
while ($i < 100) {
    $result = fib(15);
    $i = $i + 1;
}

echo "Phase 4 Benchmark Complete\n";
echo "Total operations: 30,100\n";
echo "  - 10,000 simple calls\n";
echo "  - 10,000 calls with parameters\n";
echo "  - 5,000 multi-operation calls\n";
echo "  - 5,000 nested calls\n";
echo "  - 100 recursive calls (fib(15) = ";
echo fib(15);
echo ")\n";
?>
