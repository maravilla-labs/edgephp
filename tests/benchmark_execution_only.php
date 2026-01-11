<?php

// This script measures execution time excluding PHP startup overhead

$examples = [
    'arithmetic' => function() {
        echo "=== EdgePHP Arithmetic Demo ===\n";
        $x = 15;
        $y = 3;
        echo "Numbers: x = ";
        echo $x;
        echo ", y = ";
        echo $y;
        echo "\n";
        echo "Addition: ";
        echo $x;
        echo " + ";
        echo $y;
        echo " = ";
        echo $x + $y;
        echo "\n";
        echo "Subtraction: ";
        echo $x;
        echo " - ";
        echo $y;
        echo " = ";
        echo $x - $y;
        echo "\n";
        echo "Multiplication: ";
        echo $x;
        echo " * ";
        echo $y;
        echo " = ";
        echo $x * $y;
        echo "\n";
        echo "Division: ";
        echo $x;
        echo " / ";
        echo $y;
        echo " = ";
        echo $x / $y;
        echo "\n";
    },
    
    'minimal' => function() {
        echo "test";
    },
    
    'assignment' => function() {
        $x = 42;
        echo $x;
    },
    
    'hello' => function() {
        $message = "EdgePHP is working!";
        $number = 42;
        echo $message;
        echo "\n";
        echo "The answer is: ";
        echo $number;
    },
    
    'strings' => function() {
        echo "=== String Operations ===\n";
        $first = "Hello";
        $second = "World";
        $name = "EdgePHP";
        echo "Concatenation:\n";
        echo $first . " " . $second . "!\n";
        echo "Welcome to " . $name . "\n\n";
        echo "Variable interpolation would be:\n";
        $message = $first . " from " . $name;
        echo $message;
        echo "\n\n";
        echo "Numbers to strings:\n";
        $num = 42;
        echo "The answer is: " . $num . "\n";
        echo "Calculation: " . (10 + 5) . "\n";
    }
];

// Warm up PHP engine
for ($i = 0; $i < 1000; $i++) {
    $dummy = $i * 2;
}

echo "=== PHP Native Execution Time (excluding startup) ===\n\n";

$results = [];

foreach ($examples as $name => $func) {
    // Run multiple iterations
    $iterations = 10000;
    
    // Capture output
    ob_start();
    
    // Measure time
    $start = microtime(true);
    for ($i = 0; $i < $iterations; $i++) {
        ob_clean(); // Clear buffer but keep it active
        $func();
    }
    $end = microtime(true);
    
    ob_end_clean();
    
    $totalTime = ($end - $start) * 1000000; // Convert to microseconds
    $avgTime = $totalTime / $iterations;
    
    $results[$name] = $avgTime;
    printf("%-20s: %6.1fμs (avg over %d iterations)\n", $name, $avgTime, $iterations);
}

echo "\n=== Summary ===\n";
$avgTime = array_sum($results) / count($results);
$minTime = min($results);
$maxTime = max($results);

printf("Average execution time: %.1fμs\n", $avgTime);
printf("Fastest example: %s (%.1fμs)\n", array_search($minTime, $results), $minTime);
printf("Slowest example: %s (%.1fμs)\n", array_search($maxTime, $results), $maxTime);

echo "\n=== Comparison with EdgePHP ===\n";
echo "EdgePHP typically shows: ~175μs\n";
printf("PHP native average: %.1fμs\n", $avgTime);
printf("EdgePHP is %.1fx slower than native PHP (execution only)\n", 175 / $avgTime);

// Save results
file_put_contents('php_execution_baseline.json', json_encode([
    'timestamp' => date('Y-m-d H:i:s'),
    'iterations_per_example' => $iterations,
    'results' => $results,
    'summary' => [
        'average' => $avgTime,
        'min' => $minTime,
        'max' => $maxTime
    ]
], JSON_PRETTY_PRINT));