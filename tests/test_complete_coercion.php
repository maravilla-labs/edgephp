<?php
// Complete type coercion test suite
echo "=== Complete Type Coercion Test ===\n";

// String to int coercion
echo "String to int coercion:\n";
echo "\"10\" == 10: " . (("10" == 10) ? "true" : "false") . "\n";
echo "\"0\" == 0: " . (("0" == 0) ? "true" : "false") . "\n";
echo "\"-5\" == -5: " . (("-5" == -5) ? "true" : "false") . "\n";

// String to float coercion  
echo "\nString to float coercion:\n";
echo "\"10.5\" == 10.5: " . (("10.5" == 10.5) ? "true" : "false") . "\n";
echo "\"3.14\" == 3.14: " . (("3.14" == 3.14) ? "true" : "false") . "\n";
echo "\"-2.5\" == -2.5: " . (("-2.5" == -2.5) ? "true" : "false") . "\n";
echo "\"0.0\" == 0.0: " . (("0.0" == 0.0) ? "true" : "false") . "\n";

// Int to float coercion
echo "\nInt to float coercion:\n";
echo "10 == 10.0: " . ((10 == 10.0) ? "true" : "false") . "\n";
echo "0 == 0.0: " . ((0 == 0.0) ? "true" : "false") . "\n";
echo "-5 == -5.0: " . ((-5 == -5.0) ? "true" : "false") . "\n";

// Float to int coercion
echo "\nFloat to int coercion:\n";
echo "10.0 == 10: " . ((10.0 == 10) ? "true" : "false") . "\n";
echo "0.0 == 0: " . ((0.0 == 0) ? "true" : "false") . "\n";

// Mixed type coercion (string/int/float combinations)
echo "\nMixed type coercion:\n";
echo "\"10\" == 10.0: " . (("10" == 10.0) ? "true" : "false") . "\n";
echo "10.0 == \"10\": " . ((10.0 == "10") ? "true" : "false") . "\n";
echo "\"10.0\" == 10: " . (("10.0" == 10) ? "true" : "false") . "\n";

// Invalid string conversions (should remain false)
echo "\nInvalid string conversions:\n";
echo "\"10.5a\" == 10.5: " . (("10.5a" == 10.5) ? "true" : "false") . "\n";
echo "\"abc\" == 10: " . (("abc" == 10) ? "true" : "false") . "\n";
echo "\"\" == 0: " . (("" == 0) ? "true" : "false") . "\n";
echo "\"hello\" == 0.0: " . (("hello" == 0.0) ? "true" : "false") . "\n";

// Edge cases
echo "\nEdge cases:\n";
echo "\"10.\" == 10.0: " . (("10." == 10.0) ? "true" : "false") . "\n";
echo "\".5\" == 0.5: " . ((".5" == 0.5) ? "true" : "false") . "\n";
echo "\"-.5\" == -0.5: " . (("-.5" == -0.5) ? "true" : "false") . "\n";

echo "\n✅ Complete type coercion test finished!";