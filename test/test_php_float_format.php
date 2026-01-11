<?php
// Test PHP float formatting behavior
echo "Basic decimals:\n";
echo 3.14 . "\n";
echo 2.5 . "\n";
echo 0.5 . "\n";
echo 0.125 . "\n";
echo 1.23456789 . "\n";

echo "\nTrailing zeros:\n";
echo 1.0 . "\n";
echo 2.50 . "\n";
echo 3.140 . "\n";

echo "\nVery small decimals:\n";
echo 0.1 . "\n";
echo 0.01 . "\n";
echo 0.001 . "\n";
echo 0.0001 . "\n";
echo 0.00001 . "\n";

echo "\nLarge numbers:\n";
echo 1234.5678 . "\n";
echo 12345678.9 . "\n";
echo 123456789.123456 . "\n";

echo "\nNegative:\n";
echo -3.14 . "\n";
echo -0.5 . "\n";
echo -123.456 . "\n";