<?php
// Verify all comparison operator cases
echo "=== Comparison Operator Verification ===\n\n";

// Test cases that should be TRUE (display as 1)
echo "TRUE cases (should show 1):\n";
echo "10 > 5: ", (10 > 5), "\n";
echo "5 < 10: ", (5 < 10), "\n";
echo "100 > 50: ", (100 > 50), "\n";
echo "1 < 2: ", (1 < 2), "\n";

echo "\nFALSE cases (should show empty):\n";
echo "5 > 10: ", (5 > 10), "\n";
echo "10 < 5: ", (10 < 5), "\n";
echo "50 > 100: ", (50 > 100), "\n";
echo "2 < 1: ", (2 < 1), "\n";

echo "\nEqual value cases:\n";
echo "10 > 10: ", (10 > 10), "\n";
echo "10 < 10: ", (10 < 10), "\n";

// Float tests
echo "\nFloat TRUE cases:\n";
echo "10.5 > 3.14: ", (10.5 > 3.14), "\n";
echo "3.14 < 10.5: ", (3.14 < 10.5), "\n";

echo "\nFloat FALSE cases:\n";
echo "3.14 > 10.5: ", (3.14 > 10.5), "\n";
echo "10.5 < 3.14: ", (10.5 < 3.14), "\n";