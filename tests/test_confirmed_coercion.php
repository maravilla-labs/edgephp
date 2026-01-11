<?php
// String-to-integer type coercion in == operator
echo "=== String-to-Integer Type Coercion Tests ===\n";
echo "\"10\" == 10: ", ("10" == 10), "\n";
echo "10 == \"10\": ", (10 == "10"), "\n";
echo "\"0\" == 0: ", ("0" == 0), "\n";
echo "0 == \"0\": ", (0 == "0"), "\n";
echo "\"-5\" == -5: ", ("-5" == -5), "\n";
echo "-5 == \"-5\": ", (-5 == "-5"), "\n";
echo "\"42\" == 42: ", ("42" == 42), "\n";

echo "\n=== Strict Comparison (no coercion) ===\n";
echo "\"10\" === 10: ", ("10" === 10), "\n";
echo "10 === \"10\": ", (10 === "10"), "\n";
echo "\"10\" === \"10\": ", ("10" === "10"), "\n";
echo "10 === 10: ", (10 === 10), "\n";