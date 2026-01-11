<?php
// Test case where type inference alone wouldn't help
// but escape analysis should

echo "Testing mixed type scenario...\n";

// Variables that could be any type (type inference uncertain)
// but escape analysis knows they don't escape
$a = 10;
$b = 20;

// Complex expressions where type might not be obvious
$temp1 = $a + $b;
$temp2 = $temp1 * 2;
$temp3 = $temp2 - 5;

// Chain of computations - none escape until final echo
for ($i = 0; $i < 100; $i = $i + 1) {
    $local1 = $i;
    $local2 = $local1 + 1;
    $local3 = $local2 + 1;
    $temp3 = $temp3 + $local3;
}

// Only NOW does temp3 escape
echo "Result: " . $temp3 . "\n";

// Test with variables that DO escape early
$escape1 = 100;
echo "Escape1: " . $escape1 . "\n";  // Escapes immediately

$escape2 = 200;
$arr = [$escape2];  // Escapes to array

echo "Done!\n";
