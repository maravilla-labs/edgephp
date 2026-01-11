<?php
// Intensive arithmetic benchmark - minimal I/O
// Focus on pure computation to measure inline boxing impact

$sum = 0;
$product = 1;

// 10,000 iterations of arithmetic operations
for ($i = 0; $i < 10000; $i = $i + 1) {
    // Mix of operations that will use inline boxing
    $a = $i + 1;
    $b = $i * 2;
    $c = $a + $b;
    $d = $c - $i;
    $e = $d * 3;
    $f = $e + 100;

    $sum = $sum + $f;

    // Modulo to keep numbers reasonable
    $product = ($product + $i) % 1000000;
}

echo "Sum: " . $sum . "\n";
echo "Product: " . $product . "\n";
echo "Done!\n";
