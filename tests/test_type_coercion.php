<?php
// Test string to number coercion in == operator
echo "String to number coercion tests:\n";
echo "\"10\" == 10: ", ("10" == 10), "\n";
echo "\"10.5\" == 10.5: ", ("10.5" == 10.5), "\n";
echo "10 == \"10\": ", (10 == "10"), "\n";
echo "10.5 == \"10.5\": ", (10.5 == "10.5"), "\n";
echo "\"123abc\" == 123: ", ("123abc" == 123), "\n";
echo "\"0\" == 0: ", ("0" == 0), "\n";
echo "\"-5\" == -5: ", ("-5" == -5), "\n";
echo "\"  10  \" == 10: ", ("  10  " == 10), "\n";

echo "\nString to bool coercion tests:\n";
echo "\"1\" == true: ", ("1" == true), "\n";
echo "\"0\" == false: ", ("0" == false), "\n";
echo "\"\" == false: ", ("" == false), "\n";
echo "\"hello\" == true: ", ("hello" == true), "\n";

echo "\nNull coercion tests:\n";
echo "null == 0: ", (null == 0), "\n";
echo "null == false: ", (null == false), "\n";
echo "null == \"\": ", (null == ""), "\n";