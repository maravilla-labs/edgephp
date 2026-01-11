<?php
// Fibonacci Benchmark with Timing for Standard PHP Interpreter
echo "=== Fibonacci Benchmark (Standard PHP) ===\n\n";

// Warm up run
$n = 10;
$a = 0;
$b = 1;
for ($i = 0; $i < $n; $i++) {
    $temp = $a;
    $a = $b;
    $b = $temp + $b;
}

// Test 1: Fibonacci(10)
$start = microtime(true);
$n = 10;
$a = 0;
$b = 1;
for ($i = 0; $i < $n; $i++) {
    $temp = $a;
    $a = $b;
    $b = $temp + $b;
}
$end = microtime(true);
$time1 = ($end - $start) * 1000;
echo "Fibonacci(10) = $a\n";
echo "Time: " . number_format($time1, 6) . "ms\n\n";

// Test 2: Fibonacci(20)
$start = microtime(true);
$n = 20;
$a = 0;
$b = 1;
for ($i = 0; $i < $n; $i++) {
    $temp = $a;
    $a = $b;
    $b = $temp + $b;
}
$end = microtime(true);
$time2 = ($end - $start) * 1000;
echo "Fibonacci(20) = $a\n";
echo "Time: " . number_format($time2, 6) . "ms\n\n";

// Test 3: Fibonacci(30)
$start = microtime(true);
$n = 30;
$a = 0;
$b = 1;
for ($i = 0; $i < $n; $i++) {
    $temp = $a;
    $a = $b;
    $b = $temp + $b;
}
$end = microtime(true);
$time3 = ($end - $start) * 1000;
echo "Fibonacci(30) = $a\n";
echo "Time: " . number_format($time3, 6) . "ms\n\n";

// Test 4: Fibonacci(100) - THE GOLD STANDARD
$start = microtime(true);
$n = 100;
$a = 0;
$b = 1;
for ($i = 0; $i < $n; $i++) {
    $temp = $a;
    $a = $b;
    $b = $temp + $b;
}
$end = microtime(true);
$time4 = ($end - $start) * 1000;
echo "Fibonacci(100) = $a\n";
echo "Time: " . number_format($time4, 6) . "ms\n\n";

// Summary
echo "=== Performance Summary ===\n";
echo "Total execution time: " . number_format($time1 + $time2 + $time3 + $time4, 6) . "ms\n";
echo "Average per test: " . number_format(($time1 + $time2 + $time3 + $time4) / 4, 6) . "ms\n";
