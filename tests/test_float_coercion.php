<?php
// Test float coercion in == operator
echo "=== Float Coercion Tests ===\n";

// String to float comparison
echo "\"10.5\" == 10.5: ";
echo ("10.5" == 10.5) ? "true" : "false";
echo "\n";

echo "\"3.14\" == 3.14: ";
echo ("3.14" == 3.14) ? "true" : "false";
echo "\n";

echo "\"0.5\" == 0.5: ";
echo ("0.5" == 0.5) ? "true" : "false";
echo "\n";

// Float to string comparison
echo "10.5 == \"10.5\": ";
echo (10.5 == "10.5") ? "true" : "false";
echo "\n";

// Invalid float strings should remain strings
echo "\"10.5a\" == 10.5: ";
echo ("10.5a" == 10.5) ? "true" : "false";
echo "\n";

echo "\"abc\" == 1.5: ";
echo ("abc" == 1.5) ? "true" : "false";
echo "\n";

// Edge cases
echo "\"0.0\" == 0.0: ";
echo ("0.0" == 0.0) ? "true" : "false";
echo "\n";

echo "\"-1.5\" == -1.5: ";
echo ("-1.5" == -1.5) ? "true" : "false";
echo "\n";

echo "✅ Float coercion test complete!";