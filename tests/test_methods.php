<?php
class Calculator {
    public $value;

    public function __construct($initial) {
        $this->value = $initial;
    }

    public function add($n) {
        $this->value = $this->value + $n;
    }

    public function getValue() {
        return $this->value;
    }
}

$calc = new Calculator(10);
echo "Initial: ", $calc->getValue(), "\n";
$calc->add(5);
echo "After add(5): ", $calc->getValue(), "\n";
$calc->add(3);
echo "After add(3): ", $calc->getValue(), "\n";
