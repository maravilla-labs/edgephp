<?php
// String operations example
$greeting = "Hello";
$name = "EdgePHP";
$version = "v1.0";

echo "=== String Operations Demo ===\n";

echo "Variables:\n";
echo "greeting = ";
echo $greeting;
echo "\n";
echo "name = ";
echo $name;
echo "\n";
echo "version = ";
echo $version;
echo "\n\n";

echo "String Concatenation:\n";
$simple = $greeting . " " . $name;
echo "Simple: ";
echo $simple;
echo "\n";

$complex = $greeting . " " . $name . " " . $version;
echo "Complex: ";
echo $complex;
echo "!\n";

echo "\nMixed with numbers:\n";
$count = 42;
$result = "The answer is " . $count;
echo $result;
echo "\n";