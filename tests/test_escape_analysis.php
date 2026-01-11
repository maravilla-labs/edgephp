<?php
// Test escape analysis - which values can stay unboxed?

// Test 1: Variables that DON'T escape (can stay unboxed)
function test_no_escape() {
    $a = 10;      // Literal - doesn't escape
    $b = 20;      // Literal - doesn't escape
    $c = $a + $b; // Computation - doesn't escape
    // None of these are echoed, returned, or stored in arrays
}

// Test 2: Variables that ESCAPE (must be boxed)
function test_escapes() {
    $x = 10;
    echo $x;      // ESCAPES - output to user

    $y = 20;
    return $y;    // ESCAPES - returns to caller

    $z = 30;
    $arr = [$z];  // ESCAPES - stored in array
}

// Test 3: Loop variables (complex case)
function test_loop() {
    $sum = 0;      // Used in loop, never escapes
    for ($i = 0; $i < 100; $i = $i + 1) {
        $temp = $i * 2;  // Temporary - doesn't escape
        $sum = $sum + $temp;  // Stays local
    }
    // $sum never escapes - can stay unboxed!
}

// Test 4: Propagation
function test_propagation() {
    $a = 10;
    $b = $a;      // Propagates from $a
    $c = $b;      // Propagates from $b
    echo $c;      // This makes $a, $b, $c all escape!
}

echo "Escape analysis test complete\n";
