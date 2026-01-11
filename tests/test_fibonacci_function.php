<?php
// Test recursive function - Fibonacci
function fibonacci($n) {
    if ($n <= 1) {
        return $n;
    }
    return fibonacci($n - 1) + fibonacci($n - 2);
}

echo "Fibonacci sequence:\n";
$i = 0;
while ($i <= 10) {
    echo "fib(";
    echo $i;
    echo ") = ";
    echo fibonacci($i);
    echo "\n";
    $i = $i + 1;
}
?>
