<?php
// Comprehensive String Functions Test

echo "=== strlen() ===\n";
echo "strlen('hello'): ", strlen("hello"), "\n";
echo "strlen('world!'): ", strlen("world!"), "\n";
echo "strlen(''): ", strlen(""), "\n";

echo "\n=== substr() ===\n";
$str = "Hello World";
echo "Original: '", $str, "'\n";
echo "substr(str, 0, 5): '", substr($str, 0, 5), "'\n";
echo "substr(str, 6): '", substr($str, 6), "'\n";
echo "substr(str, -5): '", substr($str, -5), "'\n";
echo "substr(str, 0, 100): '", substr($str, 0, 100), "'\n";

echo "\n=== strpos() ===\n";
$haystack = "Hello World";
echo "Haystack: '", $haystack, "'\n";
echo "strpos(haystack, 'World'): ", strpos($haystack, "World"), "\n";
echo "strpos(haystack, 'o'): ", strpos($haystack, "o"), "\n";
echo "strpos(haystack, 'xyz'): ", strpos($haystack, "xyz"), "\n";

echo "\n=== strtolower() ===\n";
echo "strtolower('HELLO'): '", strtolower("HELLO"), "'\n";
echo "strtolower('HeLLo WoRLD'): '", strtolower("HeLLo WoRLD"), "'\n";
echo "strtolower('abc123'): '", strtolower("abc123"), "'\n";

echo "\n=== strtoupper() ===\n";
echo "strtoupper('hello'): '", strtoupper("hello"), "'\n";
echo "strtoupper('HeLLo WoRLD'): '", strtoupper("HeLLo WoRLD"), "'\n";
echo "strtoupper('abc123'): '", strtoupper("abc123"), "'\n";

echo "\n=== trim() ===\n";
echo "trim('  hello  '): '", trim("  hello  "), "'\n";
echo "trim('\\thello\\n'): '", trim("\thello\n"), "'\n";
echo "trim('hello'): '", trim("hello"), "'\n";

echo "\n=== All String Functions Test Complete! ===\n";
