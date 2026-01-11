<?php

// Comprehensive benchmark script for PHP, Node.js, and EdgePHP

$examples = [
    'minimal' => [
        'php' => '<?php echo "test";',
        'js' => 'process.stdout.write("test");'
    ],
    'assignment' => [
        'php' => '<?php $x = 42; echo $x;',
        'js' => 'const x = 42; process.stdout.write(String(x));'
    ],
    'arithmetic' => [
        'php' => '<?php
$x = 15;
$y = 3;
echo "Numbers: x = ", $x, ", y = ", $y, "\n";
echo "Addition: ", ($x + $y), "\n";
echo "Subtraction: ", ($x - $y), "\n";
echo "Multiplication: ", ($x * $y), "\n";
echo "Division: ", ($x / $y), "\n";',
        'js' => '
const x = 15;
const y = 3;
process.stdout.write("Numbers: x = " + x + ", y = " + y + "\\n");
process.stdout.write("Addition: " + (x + y) + "\\n");
process.stdout.write("Subtraction: " + (x - y) + "\\n");
process.stdout.write("Multiplication: " + (x * y) + "\\n");
process.stdout.write("Division: " + (x / y) + "\\n");'
    ],
    'strings' => [
        'php' => '<?php
$first = "Hello";
$second = "World";
$name = "EdgePHP";
echo $first, " ", $second, "!\n";
echo "Welcome to ", $name, "\n";
echo "The answer is: ", 42, "\n";',
        'js' => '
const first = "Hello";
const second = "World";
const name = "EdgePHP";
process.stdout.write(first + " " + second + "!\\n");
process.stdout.write("Welcome to " + name + "\\n");
process.stdout.write("The answer is: " + 42 + "\\n");'
    ]
];

// Function to measure cold start time
function measureColdStart($command, $iterations = 50) {
    $times = [];
    
    for ($i = 0; $i < $iterations; $i++) {
        $start = microtime(true);
        exec($command . ' 2>&1', $output, $return);
        $end = microtime(true);
        $times[] = ($end - $start) * 1000000; // Convert to microseconds
    }
    
    // Remove outliers
    sort($times);
    $count = count($times);
    $trimmed = array_slice($times, (int)($count * 0.1), (int)($count * 0.8));
    
    return [
        'avg' => array_sum($trimmed) / count($trimmed),
        'min' => min($trimmed),
        'max' => max($trimmed)
    ];
}

// Function to measure warm execution
function measurePhpWarmExecution($code, $iterations = 10000) {
    // Warmup
    for ($i = 0; $i < 100; $i++) {
        ob_start();
        eval(str_replace('<?php', '', $code));
        ob_clean();
    }
    ob_end_clean();
    
    // Measure
    ob_start();
    $start = microtime(true);
    for ($i = 0; $i < $iterations; $i++) {
        ob_clean();
        eval(str_replace('<?php', '', $code));
    }
    $end = microtime(true);
    ob_end_clean();
    
    return (($end - $start) * 1000000) / $iterations;
}

$results = [
    'timestamp' => date('Y-m-d H:i:s'),
    'php' => ['coldStart' => [], 'warmExecution' => []],
    'nodejs' => ['coldStart' => [], 'warmExecution' => []],
    'edgephp' => ['typical' => 175] // EdgePHP typical execution time in microseconds
];

echo "=== Comprehensive Benchmark: PHP vs Node.js vs EdgePHP ===\n\n";

foreach ($examples as $name => $code) {
    echo "Benchmarking: $name\n";
    
    // Create temporary files
    $phpFile = "temp_$name.php";
    $jsFile = "temp_$name.js";
    
    file_put_contents($phpFile, $code['php']);
    file_put_contents($jsFile, $code['js']);
    
    // PHP Cold Start
    echo "  PHP cold start... ";
    $results['php']['coldStart'][$name] = measureColdStart("php $phpFile");
    printf("%.2fms\n", $results['php']['coldStart'][$name]['avg'] / 1000);
    
    // Node.js Cold Start
    echo "  Node.js cold start... ";
    $results['nodejs']['coldStart'][$name] = measureColdStart("node $jsFile");
    printf("%.2fms\n", $results['nodejs']['coldStart'][$name]['avg'] / 1000);
    
    // PHP Warm Execution
    echo "  PHP warm execution... ";
    $results['php']['warmExecution'][$name] = measurePhpWarmExecution($code['php']);
    printf("%.1fμs\n", $results['php']['warmExecution'][$name]);
    
    // Node.js Warm Execution (simplified - just measure the JS file execution many times)
    echo "  Node.js warm execution... ";
    $jsWarmCode = "
const iterations = 10000;
const start = process.hrtime.bigint();
for (let i = 0; i < iterations; i++) {
    " . str_replace('process.stdout.write', '// output:', $code['js']) . "
}
const end = process.hrtime.bigint();
const avgTime = Number(end - start) / 1000 / iterations;
console.log(avgTime);
";
    file_put_contents("warm_$jsFile", $jsWarmCode);
    $nodeWarmTime = trim(shell_exec("node warm_$jsFile 2>&1"));
    $results['nodejs']['warmExecution'][$name] = floatval($nodeWarmTime);
    printf("%.1fμs\n", $results['nodejs']['warmExecution'][$name]);
    
    // Cleanup
    unlink($phpFile);
    unlink($jsFile);
    unlink("warm_$jsFile");
    
    echo "\n";
}

// Calculate averages
function calculateAverage($data, $key = 'avg') {
    $sum = 0;
    $count = 0;
    foreach ($data as $item) {
        if (is_array($item) && isset($item[$key])) {
            $sum += $item[$key];
        } elseif (is_numeric($item)) {
            $sum += $item;
        }
        $count++;
    }
    return $count > 0 ? $sum / $count : 0;
}

$results['summary'] = [
    'php' => [
        'coldStart' => calculateAverage($results['php']['coldStart']),
        'warmExecution' => calculateAverage($results['php']['warmExecution'])
    ],
    'nodejs' => [
        'coldStart' => calculateAverage($results['nodejs']['coldStart']),
        'warmExecution' => calculateAverage($results['nodejs']['warmExecution'])
    ]
];

echo "=== Summary ===\n\n";
echo "Average Cold Start Times (including interpreter startup):\n";
printf("  PHP:     %.2fms\n", $results['summary']['php']['coldStart'] / 1000);
printf("  Node.js: %.2fms\n", $results['summary']['nodejs']['coldStart'] / 1000);
echo "\nAverage Warm Execution Times (no startup overhead):\n";
printf("  PHP:     %.1fμs\n", $results['summary']['php']['warmExecution']);
printf("  Node.js: %.1fμs\n", $results['summary']['nodejs']['warmExecution']);
printf("  EdgePHP: ~%dμs (typical browser execution)\n", $results['edgephp']['typical']);

echo "\n=== Performance Ratios ===\n";
$phpWarm = $results['summary']['php']['warmExecution'];
$nodeWarm = $results['summary']['nodejs']['warmExecution'];
$edgeWarm = $results['edgephp']['typical'];

printf("EdgePHP vs PHP:     %.1fx slower\n", $edgeWarm / $phpWarm);
printf("EdgePHP vs Node.js: %.1fx slower\n", $edgeWarm / $nodeWarm);
printf("Node.js vs PHP:     %.1fx %s\n", 
    $nodeWarm > $phpWarm ? $nodeWarm / $phpWarm : $phpWarm / $nodeWarm,
    $nodeWarm > $phpWarm ? 'slower' : 'faster'
);

// Save results
file_put_contents('benchmark_results_complete.json', json_encode($results, JSON_PRETTY_PRINT));
echo "\nResults saved to benchmark_results_complete.json\n";