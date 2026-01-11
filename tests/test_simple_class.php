<?php
// Phase 5: Simple class test

class Point {
    public $x;
    public $y;

    public function __construct($x, $y) {
        echo "Constructor called\n";
    }
}

$p = new Point(10, 20);
echo "Object created\n";
?>
