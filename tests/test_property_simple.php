<?php
class Counter {
    public $count = 5;

    public function __construct() {
        echo "Counter created\n";
    }
}

$c = new Counter();
echo "count = ", $c->count, "\n";
