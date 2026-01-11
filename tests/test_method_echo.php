<?php
// Test if echo works in regular methods
class Test {
    public function sayHello() {
        echo "Hello from method\n";
    }
}

$t = new Test();
echo "Created object\n";
?>
