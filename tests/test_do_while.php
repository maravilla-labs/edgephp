<?php
echo "=== Testing Do-While Loops ===\n";

// Test 1: Basic do-while (executes at least once)
echo "\n=== Test 1: Basic Do-While ===\n";
$i = 1;
do {
    echo "i = ", $i, "\n";
    $i = $i + 1;
} while ($i <= 3);
echo "After loop: i = ", $i, "\n";

// Test 2: Do-while with false condition (still executes once)
echo "\n=== Test 2: Execute At Least Once ===\n";
$x = 10;
do {
    echo "This executes once even though condition is false\n";
    echo "x = ", $x, "\n";
} while ($x < 5);

// Test 3: Do-while with break
echo "\n=== Test 3: Do-While with Break ===\n";
$count = 1;
do {
    echo "count = ", $count, "\n";
    if ($count == 3) {
        echo "Breaking at 3\n";
        break;
    }
    $count = $count + 1;
} while ($count <= 10);

// Test 4: Countdown using do-while
echo "\n=== Test 4: Countdown ===\n";
$num = 5;
do {
    echo $num, "... ";
    $num = $num - 1;
} while ($num > 0);
echo "Blast off!\n";

// Test 5: Sum using do-while
echo "\n=== Test 5: Sum Calculation ===\n";
$n = 1;
$sum = 0;
do {
    $sum = $sum + $n;
    $n = $n + 1;
} while ($n <= 5);
echo "Sum of 1 to 5: ", $sum, "\n";

echo "\n=== All tests done! ===\n";
