<?php
class Test {
    public function __construct() {
        echo "In constructor\n";
    }
}

echo "Before new\n";
$t = new Test();
echo "After new\n";
