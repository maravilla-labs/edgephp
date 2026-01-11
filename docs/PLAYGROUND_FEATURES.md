# EdgePHP Feature Examples

## ✅ Fully Working Features

### Variables & Basic Operations
```php
<?php
// Variable assignment
$x = 42;
$name = "EdgePHP";
$pi = 3.14;
$active = true;

// Arithmetic operations
echo $x + 10;  // 52
echo $x - 5;   // 37
echo $x * 2;   // 84
echo $x / 6;   // 7
echo $x % 5;   // 2 (modulo)

// String operations
echo "Hello " . $name;        // "Hello EdgePHP"
echo $name . " v" . "1.0";   // "EdgePHP v1.0"
echo "Value: $x\n";          // String interpolation
```

### Comparison & Logical Operations
```php
<?php
// Equality (with type coercion)
echo 10 == 10;   // 1 (true)
echo 10 == "10"; // 1 (true - type coercion)
echo 10 != 5;    // 1 (true)

// Strict comparison (no type coercion)
echo 10 === 10;   // 1 (true)
echo 10 === "10"; // (false - different types)
echo 10 !== "10"; // 1 (true)

// Ordering
echo 10 > 5;   // 1 (true)
echo 10 <= 10; // 1 (true)

// Logical operators
echo (true && false); // (false)
echo (true || false); // 1 (true)
```

### Control Flow - If/Else
```php
<?php
$age = 25;

if ($age >= 18) {
    echo "Adult";
} elseif ($age >= 13) {
    echo "Teenager";
} else {
    echo "Child";
}

// Ternary operator
$status = ($age >= 18) ? "Adult" : "Minor";
echo $status;
```

### Control Flow - Loops
```php
<?php
// While loop
$i = 0;
while ($i < 5) {
    echo $i . "\n";
    $i++;
}

// Do-while loop
$j = 0;
do {
    echo $j . "\n";
    $j++;
} while ($j < 3);

// For loop
for ($k = 0; $k < 5; $k++) {
    echo "Loop: $k\n";
}

// Break and continue
for ($n = 0; $n < 10; $n++) {
    if ($n == 3) continue;
    if ($n == 7) break;
    echo $n . " ";
}
```

### Arrays
```php
<?php
// Indexed arrays
$numbers = [1, 2, 3, 4, 5];
echo $numbers[0]; // 1
echo $numbers[2]; // 3

// Associative arrays
$person = ["name" => "John", "age" => 30, "city" => "NYC"];
echo $person["name"]; // "John"
echo $person["age"];  // 30

// Array modification
$numbers[1] = 10;
$person["email"] = "john@example.com";
```

### Foreach Loops
```php
<?php
// Foreach with values
$arr = [10, 20, 30];
foreach ($arr as $value) {
    echo $value . "\n";
}

// Foreach with keys and values
$data = ["a" => 1, "b" => 2, "c" => 3];
foreach ($data as $key => $value) {
    echo "$key: $value\n";
}
```

### Switch Statements
```php
<?php
$day = 3;
switch ($day) {
    case 1:
        echo "Monday";
        break;
    case 2:
        echo "Tuesday";
        break;
    case 3:
        echo "Wednesday";
        break;
    default:
        echo "Other day";
}
```

### Functions
```php
<?php
// Function definition
function add($a, $b) {
    return $a + $b;
}

// Function calls
$sum = add(10, 5);
echo $sum; // 15

// Function with string return
function greet($name) {
    return "Hello, " . $name . "!";
}

echo greet("World"); // "Hello, World!"

// Function with multiple parameters
function calculate($x, $y, $z) {
    return ($x + $y) * $z;
}

echo calculate(2, 3, 4); // 20
```

### Classes and Objects
```php
<?php
class Person {
    public $name;
    public $age;

    public function __construct($name, $age) {
        $this->name = $name;
        $this->age = $age;
    }

    public function greet() {
        return "Hello, I'm " . $this->name;
    }

    public function haveBirthday() {
        $this->age++;
    }
}

$person = new Person("Alice", 25);
echo $person->greet(); // "Hello, I'm Alice"
echo $person->age;     // 25

$person->haveBirthday();
echo $person->age;     // 26
```

### Type Casting & Unary Operators
```php
<?php
// Type casting
$str = "123";
$num = (int)$str;
echo $num + 1; // 124

$x = 5;
$float = (float)$x;
echo $float; // 5.0

// Unary operators
$a = 10;
echo ++$a; // 11 (pre-increment)
echo $a++; // 11 (post-increment, then a becomes 12)
echo --$a; // 11 (pre-decrement)

echo !true;  // (false)
echo -5;     // -5 (negation)
```

### Built-in Functions - Strings
```php
<?php
echo strlen("Hello");          // 5
echo substr("Hello World", 0, 5); // "Hello"
echo strpos("Hello", "l");     // 2
echo strtoupper("hello");      // "HELLO"
echo strtolower("WORLD");      // "world"
echo trim("  spaces  ");       // "spaces"
echo str_replace("o", "0", "hello"); // "hell0"

$parts = explode(",", "a,b,c");
echo $parts[0]; // "a"

$joined = implode("-", [1, 2, 3]);
echo $joined; // "1-2-3"
```

### Built-in Functions - Arrays
```php
<?php
$arr = [1, 2, 3];
echo count($arr); // 3

array_push($arr, 4);
echo count($arr); // 4

$last = array_pop($arr);
echo $last; // 4

$first = array_shift($arr);
echo $first; // 1

array_unshift($arr, 0);
echo $arr[0]; // 0

echo in_array(2, $arr); // 1 (true)

$keys = array_keys(["a" => 1, "b" => 2]);
$values = array_values(["x" => 10, "y" => 20]);
```

### Built-in Functions - Math
```php
<?php
echo abs(-10);      // 10
echo min(5, 10, 3); // 3
echo max(5, 10, 3); // 10
echo round(3.7);    // 4
echo floor(3.9);    // 3
echo ceil(3.1);     // 4
echo sqrt(16);      // 4
echo pow(2, 3);     // 8
```

### Built-in Functions - Type Checking
```php
<?php
echo is_int(42);       // 1 (true)
echo is_float(3.14);   // 1 (true)
echo is_string("hi");  // 1 (true)
echo is_bool(true);    // 1 (true)
echo is_array([1,2]);  // 1 (true)
echo is_null(null);    // 1 (true)
```

## 🚧 Features Not Yet Implemented

### Exception Handling
```php
<?php
try {
    // Code that may throw exception
    throw new Exception("Error occurred");
} catch (Exception $e) {
    echo "Caught: " . $e->getMessage();
} finally {
    echo "Cleanup";
}
```

### Closures & Anonymous Functions
```php
<?php
// Anonymous function
$greet = function($name) {
    return "Hello, $name";
};
echo $greet("World");

// Closures with use
$multiplier = 5;
$multiply = function($x) use ($multiplier) {
    return $x * $multiplier;
};
echo $multiply(3); // 15

// Arrow functions (PHP 7.4+)
$square = fn($n) => $n * $n;
```

### Namespaces & Use Statements
```php
<?php
namespace App\Models;

class User {
    // ...
}

use App\Models\User;
use App\Utils\{Helper, Logger};
```

### Traits & Interfaces
```php
<?php
interface Greetable {
    public function greet();
}

trait Timestampable {
    public $created_at;
    public $updated_at;
}

class User implements Greetable {
    use Timestampable;

    public function greet() {
        return "Hello";
    }
}
```

### Advanced Array Functions
```php
<?php
// Not yet implemented
$filtered = array_filter([1,2,3,4], fn($x) => $x > 2);
$mapped = array_map(fn($x) => $x * 2, [1,2,3]);
$reduced = array_reduce([1,2,3], fn($a, $b) => $a + $b, 0);
array_slice, array_splice, array_chunk, etc.
```

### Additional Built-in Functions
```php
<?php
// Date/Time
echo date("Y-m-d H:i:s");
$time = time();
$date = strtotime("2024-01-01");

// JSON
$json = json_encode(["key" => "value"]);
$data = json_decode($json, true);

// Regular expressions
preg_match("/pattern/", $string);
preg_replace("/old/", "new", $string);

// File I/O
$content = file_get_contents("file.txt");
file_put_contents("file.txt", $content);
```

### Generators
```php
<?php
function range_generator($start, $end) {
    for ($i = $start; $i <= $end; $i++) {
        yield $i;
    }
}

foreach (range_generator(1, 5) as $num) {
    echo $num;
}
```

### Static Properties & Methods
```php
<?php
class Config {
    public static $version = "1.0";

    public static function getVersion() {
        return self::$version;
    }
}

echo Config::$version;
echo Config::getVersion();
```

### Magic Methods
```php
<?php
class Magic {
    public function __get($name) { }
    public function __set($name, $value) { }
    public function __call($name, $args) { }
    public function __toString() { }
}
```

## 🎯 Current Implementation Status

### ✅ Fully Implemented
- **Core Language**: Variables, expressions, statements
- **Control Flow**: if/else/elseif, while, do-while, for, foreach, switch, break, continue
- **Data Structures**: Arrays (indexed and associative), array access/modification
- **Functions**: User-defined functions with parameters and return values
- **Classes**: OOP with properties, methods, constructors, visibility modifiers
- **Type System**: Type coercion, casting, comparison, logical operators
- **Built-in Functions**: 25+ functions across strings, arrays, math, type checking
- **Operators**: Arithmetic, comparison, logical, unary (++, --, !, -)
- **Memory Management**: Reference counting GC with optimizations

### 🚧 In Progress / Planned
- **Advanced OOP**: Traits, interfaces, static members, magic methods
- **Exception Handling**: try/catch/finally, custom exceptions
- **Closures**: Anonymous functions, arrow functions, variable capture
- **Namespaces**: Namespace declarations and use statements
- **More Built-ins**: Date/time, JSON, regex, file I/O, advanced array functions
- **Generators**: yield and generator functions

## 🏗️ Architecture Overview

### Current Implementation
- **PhpValue Structure**: 16-byte GC-managed value representation
- **Compiler**: Direct PHP → WASM bytecode generation with optimizations
- **Runtime**: Reference counting GC with escape analysis
- **Optimizations**: Loop unrolling, constant folding, inline boxing, type inference
- **Memory Model**: Linear WASM memory with structured heap management

### Performance Characteristics
- **Compilation**: Sub-second for typical programs
- **Execution**: 0.1-0.5ms for simple operations
- **Memory**: Efficient 16-byte value representation
- **Bundle Size**: Varies by program complexity

### Design Philosophy
EdgePHP prioritizes **correctness** and **PHP compatibility** over raw performance. The goal is to enable PHP in new environments (browsers, edge workers, serverless) where traditional PHP cannot run, rather than replacing traditional PHP execution.