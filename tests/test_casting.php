<?php
echo "=== Testing Type Casting ===\n";

// Test (int) cast
echo "\n=== Testing (int) cast ===\n";
echo "(int)'123': ", (int)"123", "\n";
echo "(int)'45.67': ", (int)"45.67", "\n";
echo "(int)45.67: ", (int)45.67, "\n";
echo "(int)true: ", (int)true, "\n";
echo "(int)false: ", (int)false, "\n";

// Test (float) cast
echo "\n=== Testing (float) cast ===\n";
echo "(float)'123.45': ", (float)"123.45", "\n";
echo "(float)100: ", (float)100, "\n";
echo "(float)true: ", (float)true, "\n";
echo "(float)false: ", (float)false, "\n";

// Test (string) cast
echo "\n=== Testing (string) cast ===\n";
echo "(string)123: '", (string)123, "'\n";
echo "(string)45.67: '", (string)45.67, "'\n";
echo "(string)true: '", (string)true, "'\n";
echo "(string)false: '", (string)false, "'\n";

// Test (bool) cast
echo "\n=== Testing (bool) cast ===\n";
echo "(bool)0: ", (bool)0, "\n";
echo "(bool)1: ", (bool)1, "\n";
echo "(bool)42: ", (bool)42, "\n";
echo "(bool)'': ", (bool)"", "\n";
echo "(bool)'hello': ", (bool)"hello", "\n";

// Test nested casts
echo "\n=== Testing nested casts ===\n";
echo "(int)(string)45.67: ", (int)(string)45.67, "\n";
echo "(string)(int)'123.45': '", (string)(int)"123.45", "'\n";

echo "\n=== All casting tests done! ===\n";
