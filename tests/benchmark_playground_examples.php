<?php

// Benchmark script to measure execution time of playground examples in native PHP

$examples = [
    'arithmetic' => '<?php
// Comprehensive arithmetic test
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
echo "\n";',

    'comparisons' => '<?php
// Complete comparison operators test
echo "=== Comparison Operators Test ===\n";

$a = 10;
$b = 5;
$c = 10;

echo "Values: a=";
echo $a;
echo ", b=";
echo $b;
echo ", c=";
echo $c;
echo "\n\n";

echo "Equal (==):\n";
echo "  a == b: ";
echo $a == $b;
echo " (false)\n";
echo "  a == c: ";
echo $a == $c;
echo " (true)\n\n";

echo "Not Equal (!=):\n";
echo "  a != b: ";
echo $a != $b;
echo " (true)\n";
echo "  a != c: ";
echo $a != $c;
echo " (false)\n\n";

echo "Greater Than (>):\n";
echo "  a > b: ";
echo $a > $b;
echo " (true)\n";
echo "  b > a: ";
echo $b > $a;
echo " (false)\n\n";

echo "Less Than (<):\n";
echo "  a < b: ";
echo $a < $b;
echo " (false)\n";
echo "  b < a: ";
echo $b < $a;
echo " (true)\n\n";

echo "Greater Than or Equal (>=):\n";
echo "  a >= b: ";
echo $a >= $b;
echo " (true)\n";
echo "  a >= c: ";
echo $a >= $c;
echo " (true)\n\n";

echo "Less Than or Equal (<=):\n";
echo "  a <= b: ";
echo $a <= $b;
echo " (false)\n";
echo "  a <= c: ";
echo $a <= $c;
echo " (true)\n";',

    'strings' => '<?php
// String operations showcase
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
echo "Calculation: " . (10 + 5) . "\n";',

    'types' => '<?php
// PHP type juggling demonstration
echo "=== PHP Type System ===\n";

// Integer
$int = 42;
echo "Integer: ";
echo $int;
echo "\n";

// Float
$float = 3.14159;
echo "Float: ";
echo $float;
echo "\n";

// String
$string = "Hello PHP!";
echo "String: ";
echo $string;
echo "\n";

// Boolean
$bool_true = true;
$bool_false = false;
echo "Boolean true: ";
echo $bool_true;
echo "\n";
echo "Boolean false: ";
echo $bool_false;
echo "\n";

// Null
$null_var = null;
echo "Null: ";
echo $null_var;
echo "\n\n";

echo "Type coercion examples:\n";
echo "String + Number: ";
echo "10" + 5;
echo "\n";
echo "Number + Boolean: ";
echo 10 + true;
echo "\n";',

    'variables' => '<?php
// Variable manipulation examples
echo "=== Variable Operations ===\n";

$x = 10;
echo "Initial value: ";
echo $x;
echo "\n";

$x = $x + 5;
echo "After adding 5: ";
echo $x;
echo "\n";

$x = $x * 2;
echo "After multiplying by 2: ";
echo $x;
echo "\n";

$x = $x - 10;
echo "After subtracting 10: ";
echo $x;
echo "\n";

$x = $x / 4;
echo "After dividing by 4: ";
echo $x;
echo "\n\n";

// Multiple variables
$a = 100;
$b = 50;
$c = $a + $b;
echo "a = ";
echo $a;
echo ", b = ";
echo $b;
echo ", c = a + b = ";
echo $c;
echo "\n";',

    'floats' => '<?php
// Floating point arithmetic
echo "=== Floating Point Math ===\n";

$pi = 3.14159;
$radius = 5;
$area = $pi * $radius * $radius;

echo "Circle calculations:\n";
echo "PI ≈ ";
echo $pi;
echo "\n";
echo "Radius = ";
echo $radius;
echo "\n";
echo "Area = PI * r² = ";
echo $area;
echo "\n\n";

// Division resulting in float
echo "Integer division:\n";
echo "10 / 3 = ";
echo 10 / 3;
echo "\n";
echo "7 / 2 = ";
echo 7 / 2;
echo "\n\n";

// Float operations
$x = 1.5;
$y = 2.5;
echo "Float arithmetic:\n";
echo $x;
echo " + ";
echo $y;
echo " = ";
echo $x + $y;
echo "\n";
echo $x;
echo " * ";
echo $y;
echo " = ";
echo $x * $y;
echo "\n";',

    'booleans' => '<?php
// Boolean logic examples
echo "=== Boolean Logic ===\n";

$is_php = true;
$is_compiled = true;
$is_slow = false;

echo "EdgePHP properties:\n";
echo "Is PHP? ";
echo $is_php;
echo " (1 = true)\n";
echo "Is compiled? ";
echo $is_compiled;
echo " (1 = true)\n";
echo "Is slow? ";
echo $is_slow;
echo " (empty = false)\n\n";

// Logical operations
echo "Logical AND (&&):\n";
echo "is_php && is_compiled = ";
echo $is_php && $is_compiled;
echo "\n";
echo "is_php && is_slow = ";
echo $is_php && $is_slow;
echo "\n\n";

echo "Logical OR (||):\n";
echo "is_slow || is_compiled = ";
echo $is_slow || $is_compiled;
echo "\n";
echo "is_slow || false = ";
echo $is_slow || false;
echo "\n";',

    'complex' => '<?php
// Complex expression evaluation
echo "=== Complex Expressions ===\n";

$a = 10;
$b = 5;
$c = 3;
$d = 2;

// Nested arithmetic
$result1 = ($a + $b) * ($c - $d);
echo "($a + $b) * ($c - $d) = ";
echo $result1;
echo "\n";

// Multiple operations
$result2 = $a * $b + $c * $d;
echo "$a * $b + $c * $d = ";
echo $result2;
echo "\n";

// Division and multiplication
$result3 = $a / $d * $c;
echo "$a / $d * $c = ";
echo $result3;
echo "\n";

// Complex boolean
$bool_result = ($a > $b) && ($c < $d) || ($a == 10);
echo "($a > $b) && ($c < $d) || ($a == 10) = ";
echo $bool_result;
echo "\n";',

    'coercion' => '<?php
// PHP type coercion showcase
echo "=== Type Coercion Examples ===\n";

// String to number
echo "String to number:\n";
echo "\'42\' + 8 = ";
echo "42" + 8;
echo "\n";
echo "\'3.14\' * 2 = ";
echo "3.14" * 2;
echo "\n\n";

// Boolean to number
echo "Boolean to number:\n";
echo "true + 10 = ";
echo true + 10;
echo "\n";
echo "false + 10 = ";
echo false + 10;
echo "\n\n";

// Mixed operations
echo "Mixed type operations:\n";
echo "\'5\' * true = ";
echo "5" * true;
echo "\n";
echo "\'10\' + false = ";
echo "10" + false;
echo "\n";

// Concatenation forces string
echo "\nString concatenation:\n";
echo "Number " . 42 . " as string\n";
echo "Boolean " . true . " as string\n";',

    'typeCoercion' => '<?php
// Advanced type juggling
echo "=== PHP Type Juggling ===\n";

// Numeric strings
$str_int = "123";
$str_float = "45.67";
$actual_int = 100;

echo "String + Integer:\n";
echo "\"123\" + 100 = ";
echo $str_int + $actual_int;
echo "\n\n";

echo "String float operations:\n";
echo "\"45.67\" * 2 = ";
echo $str_float * 2;
echo "\n\n";

// Boolean conversions
echo "Boolean in arithmetic:\n";
echo "true + true = ";
echo true + true;
echo "\n";
echo "true * 50 = ";
echo true * 50;
echo "\n\n";

// Null handling
$null_val = null;
echo "Null in operations:\n";
echo "null + 10 = ";
echo $null_val + 10;
echo "\n";
echo "null == 0: ";
echo $null_val == 0;
echo "\n";',

    'operators' => '<?php
// Comprehensive operator showcase
echo "=== All Operators ===\n";

$x = 20;
$y = 8;

echo "Arithmetic operators:\n";
echo "$x + $y = " . ($x + $y) . "\n";
echo "$x - $y = " . ($x - $y) . "\n";
echo "$x * $y = " . ($x * $y) . "\n";
echo "$x / $y = " . ($x / $y) . "\n\n";

echo "Comparison operators:\n";
echo "$x > $y = ";
echo $x > $y;
echo "\n";
echo "$x < $y = ";
echo $x < $y;
echo "\n";
echo "$x == $y = ";
echo $x == $y;
echo "\n";
echo "$x != $y = ";
echo $x != $y;
echo "\n\n";

echo "String concatenation:\n";
echo "Hello" . " " . "World" . "!\n";
echo "Number " . $x . " and " . $y . "\n";',

    'hello' => '<?php
// Quick demonstration
$message = "EdgePHP is working!";
$number = 42;
echo $message;
echo "\n";
echo "The answer is: ";
echo $number;',

    'minimal' => '<?php
echo "test";',

    'assignment' => '<?php
$x = 42;
echo $x;',

    'phpTags' => '<?php echo \'if you want to serve PHP code in XHTML or XML documents,
                use these tags\'; ?>

  You can use the short echo tag to <?= \'print this string\' ?>.
    It\'s equivalent to <?php echo \'print this string\' ?>.

  <? echo \'this code is within short tags, but will only work \'.
            \'if short_open_tag is enabled\'; ?>',

    'arrays' => '<?php
// PHP Array Operations with count() function
echo "=== PHP Array Operations ===\n\n";

// Test 1: Empty array
echo "Test 1: Empty array\n";
$empty = array();
echo "Empty array: count() = ";
echo count($empty);
echo "\n\n";

// Test 2: Array with 3 elements
echo "Test 2: Array with elements\n";
$numbers = array(1, 2, 3);
echo "array(1, 2, 3): count() = ";
echo count($numbers);
echo "\n\n";

// Test 3: Large array
echo "Test 3: Large array\n";
$large = array(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
echo "array(1-10): count() = ";
echo count($large);
echo "\n\n";

// Test 4: Mixed types
echo "Test 4: Mixed types\n";
$mixed = array(42, "hello", 3, "world");
echo "Mixed array: count() = ";
echo count($mixed);
echo "\n\n";

// Test 5: String and number arrays
echo "Test 5: String array\n";
$words = array("PHP", "is", "awesome");
echo "Word array: count() = ";
echo count($words);
echo "\n\n";

// Test 6: Associative arrays (string keys)
echo "Test 6: Associative arrays\n";
$person = ["name" => "John", "age" => 30];
echo "person array: count() = ";
echo count($person);
echo "\n\n";

// Test 7: Short array syntax []
echo "Test 7: Short syntax []\n";
$short = [1, 2, 3, "four", "five"];
echo "[1, 2, 3, \"four\", \"five\"]: count() = ";
echo count($short);
echo "\n\n";

// Test 8: Mixed key types
echo "Test 8: Mixed key types\n";
$mixedKeys = [0 => "first", "key" => "value", 2 => "third"];
echo "Mixed keys array: count() = ";
echo count($mixedKeys);
echo "\n\n";

echo "✅ Array operations completed!\n";
echo "📊 PHP count() function working perfectly!\n";
echo "🔥 Associative arrays & short syntax working!";'
];

// Function to measure execution time
function measureExecutionTime($code) {
    // Create a temporary file
    $tempFile = tempnam(sys_get_temp_dir(), 'php_bench_');
    file_put_contents($tempFile, $code);
    
    // Measure execution time using microtime
    $iterations = 100; // Run multiple times for more accurate measurement
    $times = [];
    
    for ($i = 0; $i < $iterations; $i++) {
        $start = microtime(true);
        exec("php -f $tempFile 2>&1", $output, $return_var);
        $end = microtime(true);
        
        if ($return_var === 0) {
            $times[] = ($end - $start) * 1000000; // Convert to microseconds
        }
    }
    
    unlink($tempFile);
    
    if (empty($times)) {
        return null;
    }
    
    // Remove outliers (top and bottom 10%)
    sort($times);
    $count = count($times);
    $trimmed = array_slice($times, (int)($count * 0.1), (int)($count * 0.8));
    
    // Return average
    return array_sum($trimmed) / count($trimmed);
}

// Run benchmarks
echo "=== PHP Native Execution Time Baseline ===\n\n";
echo "Running each example 100 times and averaging (excluding outliers)...\n\n";

$results = [];

foreach ($examples as $name => $code) {
    echo "Benchmarking: $name... ";
    flush();
    
    $time = measureExecutionTime($code);
    
    if ($time !== null) {
        $results[$name] = $time;
        printf("%.1fμs\n", $time);
    } else {
        echo "ERROR\n";
    }
}

// Summary statistics
echo "\n=== Summary ===\n";
$avgTime = array_sum($results) / count($results);
$minTime = min($results);
$maxTime = max($results);

printf("Average execution time: %.1fμs\n", $avgTime);
printf("Fastest example: %s (%.1fμs)\n", array_search($minTime, $results), $minTime);
printf("Slowest example: %s (%.1fμs)\n", array_search($maxTime, $results), $maxTime);

// Display sorted results
echo "\n=== Sorted by execution time ===\n";
asort($results);
foreach ($results as $name => $time) {
    printf("%-20s: %6.1fμs\n", $name, $time);
}

// Create JSON output for easy integration
$jsonOutput = [
    'timestamp' => date('Y-m-d H:i:s'),
    'iterations_per_example' => $iterations,
    'results' => $results,
    'summary' => [
        'average' => $avgTime,
        'min' => $minTime,
        'max' => $maxTime
    ]
];

file_put_contents('php_baseline_benchmark.json', json_encode($jsonOutput, JSON_PRETTY_PRINT));
echo "\nResults saved to php_baseline_benchmark.json\n";