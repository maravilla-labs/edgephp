<?php
// Fibonacci Benchmark - The Gold Standard Test
echo "=== Fibonacci Benchmark ===\n\n";

// Test 1: Iterative Fibonacci (small - fib(10))
echo "Test 1: Fibonacci(10) - Iterative\n";
$n = 10;
$a = 0;
$b = 1;
for ($i = 0; $i < $n; $i = $i + 1) {
    $temp = $a;
    $a = $b;
    $b = $temp + $b;
}
echo "Fibonacci(10) = " . $a . "\n\n";

// Test 2: Iterative Fibonacci (medium - fib(20))
echo "Test 2: Fibonacci(20) - Iterative\n";
$n = 20;
$a = 0;
$b = 1;
for ($i = 0; $i < $n; $i = $i + 1) {
    $temp = $a;
    $a = $b;
    $b = $temp + $b;
}
echo "Fibonacci(20) = " . $a . "\n\n";

// Test 3: Iterative Fibonacci (large - fib(30))
echo "Test 3: Fibonacci(30) - Iterative\n";
$n = 30;
$a = 0;
$b = 1;
for ($i = 0; $i < $n; $i = $i + 1) {
    $temp = $a;
    $a = $b;
    $b = $temp + $b;
}
echo "Fibonacci(30) = " . $a . "\n\n";

// Test 4: Stress test - fib(100)
echo "Test 4: Fibonacci(100) - Stress Test\n";
$n = 100;
$a = 0;
$b = 1;
for ($i = 0; $i < $n; $i = $i + 1) {
    $temp = $a;
    $a = $b;
    $b = $temp + $b;
}
echo "Fibonacci(100) = " . $a . "\n\n";

echo "Fibonacci benchmark complete!\n";
