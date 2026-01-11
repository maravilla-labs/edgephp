<?php
// Test arithmetic type coercion
echo "=== Arithmetic Type Coercion Test ===\n";

// String + number operations
echo "String + int:\n";
echo "\"10\" + 5 = " . ("10" + 5) . "\n";
echo "5 + \"10\" = " . (5 + "10") . "\n";

echo "\nString + float:\n";
echo "\"10.5\" + 2.5 = " . ("10.5" + 2.5) . "\n";
echo "2.5 + \"10.5\" = " . (2.5 + "10.5") . "\n";

echo "\nMixed arithmetic:\n";
echo "\"10\" + 5.5 = " . ("10" + 5.5) . "\n";
echo "5.5 + \"10\" = " . (5.5 + "10") . "\n";

// String concatenation vs addition
echo "\nConcat vs addition:\n";
echo "\"10\" . \"5\" = " . ("10" . "5") . "\n";
echo "\"10\" + \"5\" = " . ("10" + "5") . "\n";

// Subtraction
echo "\nSubtraction:\n";
echo "\"20\" - 5 = " . ("20" - 5) . "\n";
echo "\"10.5\" - \"2.5\" = " . ("10.5" - "2.5") . "\n";

// Multiplication
echo "\nMultiplication:\n";  
echo "\"3\" * 4 = " . ("3" * 4) . "\n";
echo "\"2.5\" * \"4\" = " . ("2.5" * "4") . "\n";

// Division
echo "\nDivision:\n";
echo "\"10\" / 2 = " . ("10" / 2) . "\n";
echo "\"15.5\" / \"2.5\" = " . ("15.5" / "2.5") . "\n";

echo "\n✅ Arithmetic coercion test complete!";