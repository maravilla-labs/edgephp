<?php
class Point {
    public $x;
    public $y;

    public function __construct($x, $y) {
        $this->x = $x;
        $this->y = $y;
    }
}

$p = new Point(10, 20);
echo "Point created\n";
echo "x = ", $p->x, "\n";
echo "y = ", $p->y, "\n";
