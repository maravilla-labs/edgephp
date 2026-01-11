<?php
// Test null handling
$x = null;
echo "null value: ", $x, "\n";

// Test null in comparisons
echo "null == null: ", ($x == $x), "\n";
echo "null != 0: ", ($x != 0), "\n";