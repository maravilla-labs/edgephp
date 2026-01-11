<?php
echo "=== Testing strpos() ===\n";
$haystack = "Hello World";
echo "Haystack: '", $haystack, "'\n";
echo "strpos(haystack, 'World'): ", strpos($haystack, "World"), "\n";
echo "strpos(haystack, 'o'): ", strpos($haystack, "o"), "\n";
echo "Done!\n";
