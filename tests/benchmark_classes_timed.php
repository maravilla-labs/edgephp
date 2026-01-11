<?php
class Point {
    public $x;
    public $y;

    public function __construct($x, $y) {
        $this->x = $x;
        $this->y = $y;
    }

    public function distance() {
        return $this->x * $this->x + $this->y * $this->y;
    }

    public function move($dx, $dy) {
        $this->x = $this->x + $dx;
        $this->y = $this->y + $dy;
    }
}

echo "=== Class Performance Benchmark ===\n";

$start = microtime(true);

// Create 100 Point objects
$i = 0;
while ($i < 100) {
    $p = new Point($i, $i);
    $dist = $p->distance();
    $p->move(1, 1);
    $i = $i + 1;
}

$end = microtime(true);
$time_ms = ($end - $start) * 1000;

echo "Created 100 objects with constructor calls\n";
echo "Called 100 methods (distance)\n";
echo "Called 100 methods (move)\n";
echo "Benchmark complete!\n";
echo "Execution time: " . number_format($time_ms, 2) . "ms\n";
