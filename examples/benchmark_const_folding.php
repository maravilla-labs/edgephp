<?php
// Benchmark constant folding optimization
echo "=== Constant Folding Benchmark ===\n\n";

// Test 1: Constant integer arithmetic (evaluated at compile time!)
echo "Test 1: Constant Integer Arithmetic\n";
echo "5 + 3 = " . (5 + 3) . "\n";
echo "10 - 4 = " . (10 - 4) . "\n";
echo "7 * 6 = " . (7 * 6) . "\n";
echo "20 / 5 = " . (20 / 5) . "\n";
echo "17 % 5 = " . (17 % 5) . "\n\n";

// Test 2: Constant float arithmetic (evaluated at compile time!)
echo "Test 2: Constant Float Arithmetic\n";
echo "3.14 + 2.71 = " . (3.14 + 2.71) . "\n";
echo "10.5 - 3.2 = " . (10.5 - 3.2) . "\n";
echo "2.5 * 4.0 = " . (2.5 * 4.0) . "\n";
echo "15.0 / 3.0 = " . (15.0 / 3.0) . "\n\n";

// Test 3: Constant string concatenation (evaluated at compile time!)
echo "Test 3: Constant String Concatenation\n";
echo "Result: " . ("Hello" . " " . "World") . "\n\n";

// Test 4: Complex constant expression
echo "Test 4: Complex Constant Expression\n";
echo "Complex result: " . ((5 + 3) * 2) . "\n\n";

echo "Constant folding complete!\n";
