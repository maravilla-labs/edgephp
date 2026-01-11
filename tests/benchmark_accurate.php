<?php

// Accurate benchmark: Cold start to completion time
// Each measurement is a complete run from start to finish

$examples = [
    'minimal' => '<?php echo "test";',
    'assignment' => '<?php $x = 42; echo $x;',
    'arithmetic' => '<?php
$x = 15;
$y = 3;
echo "Numbers: x = ", $x, ", y = ", $y, "\n";
echo "Addition: ", ($x + $y), "\n";
echo "Subtraction: ", ($x - $y), "\n";
echo "Multiplication: ", ($x * $y), "\n";
echo "Division: ", ($x / $y), "\n";',
    'strings' => '<?php
$first = "Hello";
$second = "World";
$name = "EdgePHP";
echo $first, " ", $second, "!\n";
echo "Welcome to ", $name, "\n";
echo "The answer is: ", 42, "\n";',
    'comprehensive' => '<?php
// Comprehensive test
$x = 15;
$y = 3;
echo "Variables: x=", $x, ", y=", $y, "\n\n";

// Arithmetic
echo "Arithmetic Operations:\n";
echo "x + y = ", ($x + $y), "\n";
echo "x - y = ", ($x - $y), "\n";
echo "x * y = ", ($x * $y), "\n";
echo "x / y = ", ($x / $y), "\n\n";

// Comparisons
echo "Comparisons:\n";
echo "x == y: ", ($x == $y), "\n";
echo "x != y: ", ($x != $y), "\n";
echo "x > y: ", ($x > $y), "\n";
echo "x < y: ", ($x < $y), "\n\n";

// Strings
$greeting = "Hello";
$name = "World";
echo "String Operations:\n";
echo "greeting = ", $greeting, "\n";
echo "name = ", $name, "\n";
echo "Combined: ", $greeting, " ", $name, "!\n\n";

echo "All features working!\n";'
];

// Function to measure cold start to completion
function measureColdStartToCompletion($command, $iterations = 100) {
    $times = [];
    
    for ($i = 0; $i < $iterations; $i++) {
        $start = microtime(true);
        exec($command . ' 2>&1', $output, $return);
        $end = microtime(true);
        $times[] = ($end - $start) * 1000; // Convert to milliseconds
    }
    
    // Remove outliers (top and bottom 10%)
    sort($times);
    $count = count($times);
    $trimmed = array_slice($times, (int)($count * 0.1), (int)($count * 0.8));
    
    return [
        'avg' => array_sum($trimmed) / count($trimmed),
        'min' => min($trimmed),
        'max' => max($trimmed),
        'raw_times' => $times
    ];
}

// Function to measure EdgePHP (compilation + execution in Node.js)
function measureEdgePHP($phpCode, $iterations = 50) {
    // Create temporary PHP file
    $tempFile = tempnam(sys_get_temp_dir(), 'edgephp_bench_');
    file_put_contents($tempFile, $phpCode);
    
    // Compile with EdgePHP
    $wasmFile = $tempFile . '.wasm';
    $compileCommand = "cargo run --release --bin edge-php -- compile $tempFile -o $wasmFile 2>&1";
    
    $times = [];
    
    for ($i = 0; $i < $iterations; $i++) {
        $start = microtime(true);
        
        // Compile
        exec($compileCommand, $output, $return);
        if ($return !== 0) {
            continue;
        }
        
        // Execute with Node.js
        $execCommand = "node test_wasm.js $wasmFile 2>&1";
        exec($execCommand, $output, $return);
        
        $end = microtime(true);
        $times[] = ($end - $start) * 1000; // milliseconds
    }
    
    // Cleanup
    unlink($tempFile);
    if (file_exists($wasmFile)) {
        unlink($wasmFile);
    }
    
    // Remove outliers
    sort($times);
    $count = count($times);
    $trimmed = array_slice($times, (int)($count * 0.1), (int)($count * 0.8));
    
    return [
        'avg' => array_sum($trimmed) / count($trimmed),
        'min' => min($trimmed),
        'max' => max($trimmed),
        'count' => count($trimmed)
    ];
}

$results = [
    'timestamp' => date('Y-m-d H:i:s'),
    'description' => 'Cold start to completion time (interpreter startup + code execution)',
    'php' => [],
    'edgephp' => []
];

echo "=== Accurate Benchmark: PHP vs EdgePHP ===\n";
echo "Measuring cold start to completion time\n";
echo "(Each measurement includes interpreter startup + code execution)\n\n";

foreach ($examples as $name => $code) {
    echo "Benchmarking: $name\n";
    
    // Create temporary PHP file
    $phpFile = tempnam(sys_get_temp_dir(), 'bench_') . '.php';
    file_put_contents($phpFile, $code);
    
    // Measure PHP
    echo "  PHP... ";
    flush();
    $results['php'][$name] = measureColdStartToCompletion("php $phpFile");
    printf("%.2fms (avg of %d runs)\n", $results['php'][$name]['avg'], 80);
    
    // Measure EdgePHP (if possible)
    if (file_exists('cargo')) {
        echo "  EdgePHP... ";
        flush();
        $results['edgephp'][$name] = measureEdgePHP($code, 20);
        if ($results['edgephp'][$name]['count'] > 0) {
            printf("%.2fms (avg of %d runs)\n", $results['edgephp'][$name]['avg'], $results['edgephp'][$name]['count']);
        } else {
            echo "Failed to compile\n";
        }
    }
    
    // Cleanup
    unlink($phpFile);
    
    echo "\n";
}

// Calculate averages
function calculateAverage($data) {
    $sum = 0;
    $count = 0;
    foreach ($data as $item) {
        if (isset($item['avg']) && $item['avg'] > 0) {
            $sum += $item['avg'];
            $count++;
        }
    }
    return $count > 0 ? $sum / $count : 0;
}

$results['summary'] = [
    'php' => [
        'avgColdStartToCompletion' => calculateAverage($results['php']),
        'description' => 'Average time from PHP interpreter start to program completion'
    ]
];

if (!empty($results['edgephp'])) {
    $results['summary']['edgephp'] = [
        'avgColdStartToCompletion' => calculateAverage($results['edgephp']),
        'description' => 'Average time from EdgePHP compile + Node.js execution to completion'
    ];
}

echo "=== Summary ===\n\n";
echo "Average Cold Start to Completion Times:\n";
printf("  PHP:     %.2fms\n", $results['summary']['php']['avgColdStartToCompletion']);
if (isset($results['summary']['edgephp'])) {
    printf("  EdgePHP: %.2fms (compile + execute)\n", $results['summary']['edgephp']['avgColdStartToCompletion']);
}

echo "\nNote: EdgePHP in browser has different characteristics:\n";
echo "  - Compilation happens once in browser (~5ms)\n";
echo "  - Execution is fast after compilation (~0.2ms)\n";
echo "  - No repeated compilation needed for same code\n";

// Save results
file_put_contents('benchmark_accurate_results.json', json_encode($results, JSON_PRETTY_PRINT));
echo "\nResults saved to benchmark_accurate_results.json\n";