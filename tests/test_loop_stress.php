<?php
// Stress test - many iterations to get measurable timing
echo "Loop stress test...\n";

$sum = 0;
for ($i = 0; $i < 1000; $i = $i + 1) {
    $sum = $sum + $i;
}

echo "Sum(1000) = " . $sum . "\n";

$sum2 = 0;
for ($j = 0; $j < 1000; $j = $j + 1) {
    $sum2 = $sum2 + $j;
}

echo "Sum2(1000) = " . $sum2 . "\n";

$sum3 = 0;
for ($k = 0; $k < 1000; $k = $k + 1) {
    $sum3 = $sum3 + $k;
}

echo "Sum3(1000) = " . $sum3 . "\n";
