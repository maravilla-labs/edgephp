<?php
// Simple test for isset() and empty()
$x = null;
$y = 0;
$z = "hello";

echo "isset(null): ", isset($x), "\n";
echo "isset(0): ", isset($y), "\n";
echo "isset('hello'): ", isset($z), "\n";

echo "empty(null): ", empty($x), "\n";
echo "empty(0): ", empty($y), "\n";
echo "empty('hello'): ", empty($z), "\n";