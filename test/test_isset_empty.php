<?php
echo "=== Testing isset() and empty() functions ===\n\n";

// Test variables
$null_var = null;
$false_var = false;
$true_var = true;
$zero_int = 0;
$nonzero_int = 42;
$zero_float = 0.0;
$nonzero_float = 3.14;
$empty_string = "";
$zero_string = "0";
$nonempty_string = "hello";

echo "1. isset() tests:\n";
echo "   isset(null): ", isset($null_var), "\n";
echo "   isset(false): ", isset($false_var), "\n";
echo "   isset(true): ", isset($true_var), "\n";
echo "   isset(0): ", isset($zero_int), "\n";
echo "   isset(42): ", isset($nonzero_int), "\n";
echo "   isset(0.0): ", isset($zero_float), "\n";
echo "   isset(3.14): ", isset($nonzero_float), "\n";
echo "   isset(''): ", isset($empty_string), "\n";
echo "   isset('0'): ", isset($zero_string), "\n";
echo "   isset('hello'): ", isset($nonempty_string), "\n";

echo "\n2. empty() tests:\n";
echo "   empty(null): ", empty($null_var), "\n";
echo "   empty(false): ", empty($false_var), "\n";
echo "   empty(true): ", empty($true_var), "\n";
echo "   empty(0): ", empty($zero_int), "\n";
echo "   empty(42): ", empty($nonzero_int), "\n";
echo "   empty(0.0): ", empty($zero_float), "\n";
echo "   empty(3.14): ", empty($nonzero_float), "\n";
echo "   empty(''): ", empty($empty_string), "\n";
echo "   empty('0'): ", empty($zero_string), "\n";
echo "   empty('hello'): ", empty($nonempty_string), "\n";

echo "\n✅ All isset() and empty() tests completed!\n";