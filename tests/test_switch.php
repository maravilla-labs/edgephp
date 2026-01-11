<?php
echo "=== Testing Switch Statements ===\n";

// Test 1: Simple integer switch
echo "\n=== Test 1: Integer Switch ===\n";
$num = 2;
echo "Number: ", $num, "\n";

switch ($num) {
    case 1:
        echo "One\n";
        break;
    case 2:
        echo "Two\n";
        break;
    case 3:
        echo "Three\n";
        break;
    default:
        echo "Other\n";
        break;
}

// Test 2: String switch
echo "\n=== Test 2: String Switch ===\n";
$fruit = "apple";
echo "Fruit: ", $fruit, "\n";

switch ($fruit) {
    case "apple":
        echo "It's an apple!\n";
        break;
    case "banana":
        echo "It's a banana!\n";
        break;
    case "orange":
        echo "It's an orange!\n";
        break;
    default:
        echo "Unknown fruit\n";
        break;
}

// Test 3: Default case
echo "\n=== Test 3: Default Case ===\n";
$value = 999;
echo "Value: ", $value, "\n";

switch ($value) {
    case 1:
        echo "Value is 1\n";
        break;
    case 2:
        echo "Value is 2\n";
        break;
    default:
        echo "Value is something else\n";
        break;
}

// Test 4: No break (fall-through)
echo "\n=== Test 4: Fall-through ===\n";
$day = 5;
echo "Day: ", $day, "\n";

switch ($day) {
    case 1:
        echo "Monday\n";
        break;
    case 5:
        echo "Friday\n";
        echo "Weekend is coming!\n";
        break;
    case 6:
    case 7:
        echo "Weekend!\n";
        break;
    default:
        echo "Midweek\n";
        break;
}

echo "\n=== All tests done! ===\n";
