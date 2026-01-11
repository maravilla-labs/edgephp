# EdgePHP Missing Features & Roadmap

**Last Updated:** 2025-10-28 (After Phase 12)
**Status:** Comprehensive list of missing functionality

---

## ✅ What's Already Implemented (Pretty Complete!)

### Core Language
- ✅ Variables, arithmetic, string concatenation
- ✅ All control flow: if/else/elseif, while, do-while, for, foreach, switch
- ✅ break/continue
- ✅ Functions: definition, calls, parameters, return values
- ✅ Arrays: creation, access, assignment
- ✅ Type casting: (int), (string), (bool), (float)
- ✅ Operators: ++, --, +=, -=, *=, /=, !, -, +, *, /, ., ==, !=, <, >, <=, >=
- ✅ Ternary operator: `$x ? $y : $z`
- ✅ Classes (basic): definition, instantiation, properties, methods
- ✅ String interpolation: `"Hello $name"`

### Built-in Functions (30+ functions)

**Arrays (9 functions)**:
- ✅ count, array_push, array_pop, array_shift, array_unshift
- ✅ in_array, array_keys, array_values, array_merge

**Strings (8 functions)**:
- ✅ strlen, substr, strpos, strtolower, strtoupper, trim
- ⚠️ str_replace (stub), explode (stub), implode (stub)

**Math (8 functions)**:
- ✅ abs, min, max, round, floor, ceil, sqrt
- ⚠️ pow (approximation only)

**Type Checking (7 functions)**:
- ✅ is_int, is_float, is_string, is_bool, is_array, is_object, is_null

**Utility (2 functions)**:
- ✅ isset, empty

---

## ❌ What's Missing

### 1. Error Handling 🔴 **HIGH PRIORITY**
Status: ❌ Not implemented at all

```php
try {
    // code that might throw
    throw new Exception("Something went wrong");
} catch (Exception $e) {
    echo "Error: " . $e->getMessage();
} finally {
    // cleanup
}
```

**What's needed:**
- `try` statement parsing and compilation
- `catch` blocks with exception variable binding
- `finally` blocks (always execute)
- `throw` statement
- Exception class and hierarchy
- Stack unwinding mechanism in WASM
- Error propagation infrastructure

**Why important:** Essential for production code, error recovery, and debugging.

**Estimated effort:** 6-8 hours (complex due to WASM stack unwinding)

---

### 2. Closures / Anonymous Functions 🔴 **HIGH PRIORITY**
Status: ❌ Not implemented

```php
// Anonymous function
$multiply = function($x, $y) {
    return $x * $y;
};
echo $multiply(5, 3);  // 15

// Arrow function (PHP 7.4+)
$double = fn($x) => $x * 2;
echo $double(5);  // 10

// Closure with use
$factor = 10;
$scale = function($x) use ($factor) {
    return $x * $factor;
};
```

**What's needed:**
- Parse anonymous function expressions
- Closure variable capture (use clause)
- Arrow function syntax (fn)
- Function pointers/references in WASM
- Closure environment structs

**Why important:** Very common in modern PHP, used in array_filter/array_map, callbacks, event handlers.

**Estimated effort:** 5-7 hours

---

### 3. Constants 🟡 **MEDIUM PRIORITY**
Status: ❌ Not implemented

```php
const MY_CONSTANT = 42;
define('ANOTHER_CONSTANT', 'value');
echo MY_CONSTANT;
echo ANOTHER_CONSTANT;

class MyClass {
    const CLASS_CONSTANT = 100;
}
echo MyClass::CLASS_CONSTANT;
```

**What's needed:**
- `const` keyword parsing
- `define()` function
- Constant resolution at compile time
- Class constants
- Global constant table

**Why important:** Used everywhere in PHP for configuration, settings, magic values.

**Estimated effort:** 2-3 hours (relatively easy)

---

### 4. Static Properties/Methods 🟡 **MEDIUM PRIORITY**
Status: ❌ Not implemented

```php
class Counter {
    public static $count = 0;

    public static function increment() {
        self::$count++;
    }

    public static function getCount() {
        return self::$count;
    }
}

Counter::increment();
echo Counter::$count;  // 1
echo Counter::getCount();  // 1
```

**What's needed:**
- `static` keyword recognition
- Static storage separate from instances
- `self::` and `ClassName::` resolution
- Static initializers

**Why important:** Common for singletons, factory methods, shared state.

**Estimated effort:** 3-4 hours

---

### 5. Magic Methods 🟡 **MEDIUM PRIORITY**
Status: ❌ Not implemented (classes exist but no magic methods)

```php
class MyClass {
    private $data = [];

    public function __construct($x) {
        // Constructor
    }

    public function __destruct() {
        // Destructor
    }

    public function __toString() {
        return "MyClass instance";
    }

    public function __get($name) {
        return $this->data[$name] ?? null;
    }

    public function __set($name, $value) {
        $this->data[$name] = $value;
    }

    public function __call($name, $args) {
        // Handle undefined method calls
    }

    public function __invoke($x) {
        // Make object callable
    }
}
```

**What's needed:**
- __construct (constructor)
- __destruct (destructor)
- __toString (string conversion)
- __get, __set (property access)
- __call (method calls)
- __invoke (callable objects)
- __clone, __sleep, __wakeup, etc.

**Why important:** Core to PHP OOP, used heavily in frameworks.

**Estimated effort:** 4-6 hours

---

### 6. Interfaces & Traits 🟡 **MEDIUM PRIORITY**
Status: ❌ Not implemented

```php
interface Drawable {
    public function draw();
}

trait Timestampable {
    private $created_at;

    public function setTimestamp() {
        $this->created_at = time();
    }
}

class Shape implements Drawable {
    use Timestampable;

    public function draw() {
        echo "Drawing shape";
    }
}
```

**What's needed:**
- `interface` keyword and parsing
- `trait` keyword and parsing
- `implements` clause compilation
- `use` clause in classes (trait usage)
- Interface method validation
- Trait method composition and conflict resolution

**Why important:** Essential for modern PHP architecture, dependency injection, contracts.

**Estimated effort:** 5-6 hours

---

### 7. Abstract Classes 🟡 **MEDIUM PRIORITY**
Status: ❌ Not implemented

```php
abstract class Animal {
    protected $name;

    abstract public function makeSound();

    public function getName() {
        return $this->name;
    }
}

class Dog extends Animal {
    public function makeSound() {
        echo "Woof!";
    }
}

// $animal = new Animal();  // Error: cannot instantiate abstract class
$dog = new Dog();
$dog->makeSound();
```

**What's needed:**
- `abstract` keyword
- Abstract method declarations
- Prevent instantiation of abstract classes
- Enforce implementation in child classes

**Why important:** Used for base classes, frameworks, design patterns.

**Estimated effort:** 2-3 hours

---

### 8. Namespaces & Use Statements 🟡 **MEDIUM PRIORITY**
Status: ⚠️ Parsed in AST but NOT compiled (no-op)

```php
namespace MyApp\Controllers;

use MyApp\Models\User;
use MyApp\Services\AuthService as Auth;

class UserController {
    public function show($id) {
        $user = User::find($id);
        Auth::check();
    }
}
```

**What's needed:**
- Namespace compilation
- Class name resolution with namespaces
- `use` statement imports
- Aliasing with `as`
- Global namespace `\`
- Autoloading integration

**Why important:** Required for any non-trivial PHP application, PSR-4 autoloading.

**Estimated effort:** 4-5 hours

---

### 9. More Array Functions 🟡 **MEDIUM PRIORITY**
Status: ⚠️ Many common functions missing

**High-value missing functions:**

```php
// Functional programming
array_filter($arr, function($x) { return $x > 5; });  // ❌
array_map(function($x) { return $x * 2; }, $arr);     // ❌
array_reduce($arr, function($carry, $x) { }, 0);      // ❌

// Searching & extraction
array_search($needle, $haystack);                      // ❌
array_slice($arr, $offset, $length);                   // ❌
array_splice($arr, $offset, $length, $replacement);    // ❌

// Sorting
sort($arr);                                            // ❌
rsort($arr);                                           // ❌
asort($arr);  // Sort preserving keys                  // ❌
ksort($arr);  // Sort by keys                          // ❌
usort($arr, function($a, $b) { });                     // ❌

// Manipulation
array_reverse($arr);                                   // ❌
array_unique($arr);                                    // ❌
array_flip($arr);                                      // ❌
array_column($arr, 'column_name');                     // ❌
array_combine($keys, $values);                         // ❌
array_chunk($arr, $size);                              // ❌

// Set operations
array_intersect($arr1, $arr2);                         // ❌
array_diff($arr1, $arr2);                              // ❌
array_union($arr1, $arr2);                             // ❌

// Checking
array_key_exists($key, $arr);                          // ❌
```

**Why important:** These are heavily used in real applications, especially array_filter/map/reduce.

**Estimated effort:** 6-8 hours for top 10 most common ones

---

### 10. More String Functions 🟡 **MEDIUM PRIORITY**
Status: ⚠️ Basic ones work, many missing

**High-value missing functions:**

```php
// String manipulation
str_replace($search, $replace, $subject);              // ⚠️ Stub
str_ireplace($search, $replace, $subject);             // ❌
str_repeat($str, $times);                              // ❌
str_pad($str, $length, $pad);                          // ❌
str_split($str, $length);                              // ❌

// Splitting/joining
explode($delimiter, $string);                          // ⚠️ Stub
implode($glue, $array);                                // ⚠️ Stub
join($glue, $array);                                   // ❌

// Searching/replacing
str_contains($haystack, $needle);  // PHP 8            // ❌
str_starts_with($haystack, $needle);  // PHP 8         // ❌
str_ends_with($haystack, $needle);  // PHP 8           // ❌
strstr($haystack, $needle);                            // ❌
stripos($haystack, $needle);  // Case-insensitive      // ❌
strripos($haystack, $needle);                          // ❌
strrpos($haystack, $needle);  // Last occurrence       // ❌

// Formatting
sprintf($format, ...$args);                            // ❌
printf($format, ...$args);                             // ❌
number_format($number, $decimals);                     // ❌

// Regular expressions
preg_match($pattern, $subject);                        // ❌
preg_match_all($pattern, $subject);                    // ❌
preg_replace($pattern, $replacement, $subject);        // ❌
preg_split($pattern, $subject);                        // ❌

// Encoding
json_encode($value);                                   // ❌
json_decode($json);                                    // ❌
base64_encode($str);                                   // ❌
base64_decode($str);                                   // ❌
urlencode($str);                                       // ❌
urldecode($str);                                       // ❌
htmlspecialchars($str);                                // ❌
```

**Why important:** String processing is core to web development.

**Estimated effort:** 8-10 hours for top 15 functions

---

### 11. References 🟢 **LOW PRIORITY**
Status: ❌ Not implemented

```php
function modify(&$var) {
    $var = 42;
}

$x = 10;
modify($x);
echo $x;  // 42

// Reference assignment
$a = 5;
$b = &$a;
$b = 10;
echo $a;  // 10
```

**What's needed:**
- `&` prefix in function parameters
- Reference assignment `=&`
- Reference tracking in runtime
- Shared memory semantics

**Why important:** Used in legacy code, performance optimization, some patterns.

**Estimated effort:** 5-6 hours

---

### 12. Global Keyword 🟢 **LOW PRIORITY**
Status: ❌ Not implemented

```php
$global_var = 42;

function test() {
    global $global_var;
    echo $global_var;  // 42
}
```

**What's needed:**
- Global variable scope
- `global` keyword parsing
- Access to global scope from functions

**Why important:** Common in legacy PHP, less so in modern code.

**Estimated effort:** 2-3 hours

---

### 13. Include/Require 🟢 **LOW PRIORITY**
Status: ❌ Not implemented

```php
require 'config.php';
require_once 'functions.php';
include 'optional.php';
include_once 'helpers.php';
```

**What's needed:**
- File resolution
- Parse and compile included files
- Track inclusion to prevent duplicates (_once variants)
- Module system in WASM
- Shared global state

**Why important:** Required for multi-file applications.

**Estimated effort:** 6-8 hours (complex due to module system)

---

### 14. Variable Variables 🟢 **LOW PRIORITY**
Status: ❌ Not implemented

```php
$name = "var";
$$name = "value";  // $var = "value"
echo $var;  // "value"

// Dynamic property access
$prop = "property";
echo $obj->$prop;
```

**What's needed:**
- Parse `$$var` syntax
- Runtime variable name resolution
- Dynamic symbol table lookup

**Why important:** Rarely used in modern code, but exists in legacy systems.

**Estimated effort:** 3-4 hours

---

### 15. Generators 🟢 **LOW PRIORITY**
Status: ❌ Not implemented

```php
function numbers() {
    yield 1;
    yield 2;
    yield 3;
}

foreach (numbers() as $num) {
    echo $num;
}

// Generator with keys
function pairs() {
    yield "a" => 1;
    yield "b" => 2;
}
```

**What's needed:**
- `yield` keyword
- Generator state machine
- Iterator protocol
- Resume/suspend mechanism in WASM

**Why important:** Memory-efficient iteration over large datasets.

**Estimated effort:** 8-10 hours (complex state management)

---

### 16. Heredoc/Nowdoc 🟢 **LOW PRIORITY**
Status: ❌ Not implemented

```php
$text = <<<EOT
Multi-line
string with $interpolation
EOT;

$literal = <<<'EOT'
No interpolation here
EOT;
```

**What's needed:**
- Parse heredoc syntax
- Handle interpolation in heredoc
- Nowdoc (no interpolation)

**Why important:** Nice for multi-line text, SQL queries, HTML templates.

**Estimated effort:** 2-3 hours

---

### 17. More Operators 🟢 **LOW PRIORITY**

**Null Coalescing:**
```php
$x = $y ?? 'default';  // ❌
$x ??= 'default';      // ❌
```

**Spaceship:**
```php
$cmp = $a <=> $b;  // Returns -1, 0, or 1  // ❌
```

**Bitwise:**
```php
$x & $y   // AND      // ❌
$x | $y   // OR       // ❌
$x ^ $y   // XOR      // ❌
$x << $y  // Left shift   // ❌
$x >> $y  // Right shift  // ❌
~$x       // NOT      // ❌
```

**Why important:** Used for configuration, flags, comparisons.

**Estimated effort:** 2-3 hours

---

### 18. More Math Functions 🟢 **LOW PRIORITY**

```php
// Trigonometry
sin($x), cos($x), tan($x)                    // ❌
asin($x), acos($x), atan($x)                 // ❌

// Logarithms
log($x), log10($x), log1p($x)                // ❌
exp($x), expm1($x)                           // ❌

// Random
rand($min, $max)                             // ❌
mt_rand($min, $max)                          // ❌
random_int($min, $max)                       // ❌

// Other
pi()                                         // ❌
deg2rad($deg), rad2deg($rad)                 // ❌
hypot($x, $y)                                // ❌
```

**Why important:** Scientific computing, graphics, games.

**Estimated effort:** 3-4 hours (WASM has many built-in math ops)

---

### 19. File System Functions 🟢 **LOW PRIORITY**
Status: ❌ Not implemented (requires WASI or host functions)

```php
// Reading/writing
file_get_contents($path);           // ❌
file_put_contents($path, $data);    // ❌
fopen($path, $mode);                // ❌
fread($handle, $length);            // ❌
fwrite($handle, $data);             // ❌
fclose($handle);                    // ❌

// File info
file_exists($path);                 // ❌
is_file($path);                     // ❌
is_dir($path);                      // ❌
filesize($path);                    // ❌
filemtime($path);                   // ❌

// Directory operations
scandir($dir);                      // ❌
mkdir($path);                       // ❌
rmdir($path);                       // ❌
```

**Why important:** Required for file-based applications, but tricky in WASM.

**Estimated effort:** 10-12 hours (requires WASI integration)

---

### 20. Date/Time Functions 🟢 **LOW PRIORITY**
Status: ❌ Not implemented

```php
time();                           // ❌
date($format);                    // ❌
strtotime($str);                  // ❌
microtime($as_float);             // ❌
DateTime, DateInterval classes    // ❌
```

**Why important:** Common in applications, but WASM has limited time access.

**Estimated effort:** 4-6 hours

---

## 📋 Known Code TODOs

From codebase grep:

### In `arrays.rs`:
- Line 938: TODO: Implement proper value extraction with reindexing
- Line 1243, 1272: TODO: Implement proper collision chaining

### In `expressions.rs`:
- Line 1250: TODO: Implement proper array casting
- Line 1371: TODO: Handle floats properly in increment/decrement

### In `builtins.rs`:
- Line 1363: TODO: proper pow implementation (currently approximates)

### In `runtime.rs`:
- Line 886: TODO: Implement proper array/object comparison
- Line 1350: TODO: Implement proper float to string conversion
- Line 1608: TODO: Implement proper float and array conversion

---

## 🎯 Recommended Roadmap

### Phase 13 - Error Handling (6-8 hours) 🔴 **MOST CRITICAL**
**Why:** Essential for production applications
- try/catch/finally blocks
- throw statement
- Exception class hierarchy
- Stack unwinding

### Phase 14 - Closures (5-7 hours) 🔴 **VERY IMPORTANT**
**Why:** Used everywhere in modern PHP
- Anonymous functions
- Arrow functions (fn)
- Use clauses (variable capture)
- Function references

### Phase 15 - Constants (2-3 hours) 🟡 **EASY WIN**
**Why:** Quick to implement, high value
- const keyword
- define() function
- Class constants

### Phase 16 - Array Functions Part 2 (6-8 hours) 🟡 **HIGH VALUE**
**Why:** Core PHP functionality
- array_filter, array_map, array_reduce
- array_search, array_slice
- sort, rsort, ksort

### Phase 17 - String Functions Part 2 (6-8 hours) 🟡 **HIGH VALUE**
**Why:** Core PHP functionality
- str_replace (finish stub)
- explode/implode (finish stubs)
- Regular expressions (basic)
- JSON encode/decode

### Phase 18 - Static & Magic Methods (6-8 hours) 🟡
**Why:** Required for OOP
- Static properties/methods
- __construct, __toString
- __get, __set

### Phase 19 - Advanced OOP (8-10 hours) 🟡
**Why:** Modern PHP requires these
- Interfaces
- Traits
- Abstract classes
- instanceof operator

### Phase 20 - More Operators (2-3 hours) 🟢
- Null coalescing (??, ??=)
- Spaceship (<=>)
- Bitwise operators

### Phase 21+ - Lower Priority
- References
- Global keyword
- Include/require
- Variable variables
- Generators
- Heredoc/Nowdoc
- File system (requires WASI)
- Date/Time

---

## 📊 Summary Statistics

### Implemented Features
- ✅ **Core Language**: ~95% complete
- ✅ **Control Flow**: 100% (if, while, for, foreach, do-while, switch, break, continue)
- ✅ **Operators**: ~80% (missing bitwise, null coalescing, spaceship)
- ✅ **Functions**: ~70% (missing closures, default params, variadic)
- ✅ **Arrays**: ~30% (9/30+ common functions)
- ✅ **Strings**: ~25% (8/32+ common functions)
- ✅ **Math**: ~50% (8/16+ common functions)
- ✅ **Classes**: ~40% (basic OOP, missing static, magic, abstract, interface, trait)

### Overall Completion
**Estimated: 60% of common PHP functionality implemented**

EdgePHP is **highly functional** for:
- ✅ Algorithmic problems
- ✅ Data processing
- ✅ Basic web logic
- ✅ Control flow heavy code

EdgePHP **needs work** for:
- ❌ Production web applications (needs error handling)
- ❌ Modern PHP frameworks (needs closures, namespaces, interfaces)
- ❌ File-based applications (needs file I/O)
- ❌ Heavy string/array processing (needs more functions)

---

## 🚀 Next Steps

**Immediate priorities to reach production-ready:**
1. Error handling (try/catch/throw)
2. Closures and anonymous functions
3. Constants
4. More array functions (filter, map, reduce, search)
5. More string functions (str_replace, explode, implode, regex)
6. Static methods and magic methods
7. Interfaces and traits

**With these 7 phases complete, EdgePHP would be ~80% production-ready for modern PHP applications!**
