// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

import React, { useState, useEffect } from 'react'
import Editor from '@monaco-editor/react'
import init, { compile_php, parse_php } from './wasm/edge_php_wasm.js'
import { EdgePHPRuntime } from './runtime.js'
import { PerformanceBaseline } from './PerformanceBaseline.jsx'
import { BenchmarkModal } from './BenchmarkModal.jsx'

const DEFAULT_CODE = `<?php
// EdgePHP Compiler Demo
echo "🚀 Welcome to EdgePHP! 🚀\\n";

$x = 10;
$y = 5;
echo "Computing: ";
echo $x;
echo " + ";
echo $y;
echo " = ";
echo $x + $y;
echo "\\n";

echo "String concat: ";
echo "Hello" . " " . "World!";
echo "\\n";

echo "Comparison: 10 > 5 is ";
echo $x > $y;
echo " (1=true)\\n";

echo "✨ Try the examples above! ✨";`;

// Performance utilities
const formatBytes = (bytes) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

const formatTime = (ms) => {
  if (ms < 1) return (ms * 1000).toFixed(0) + 'μs'
  if (ms < 1000) return ms.toFixed(1) + 'ms'
  return (ms / 1000).toFixed(2) + 's'
}

const EXAMPLES = {
  arithmetic: {
    name: "🧮 Arithmetic Operations",
    code: `<?php
// Comprehensive arithmetic test
echo "=== EdgePHP Arithmetic Demo ===\\n";

$x = 15;
$y = 3;
echo "Numbers: x = ";
echo $x;
echo ", y = ";
echo $y;
echo "\\n";

echo "Addition: ";
echo $x;
echo " + ";
echo $y;
echo " = ";
echo $x + $y;
echo "\\n";

echo "Subtraction: ";
echo $x;
echo " - ";
echo $y;
echo " = ";
echo $x - $y;
echo "\\n";

echo "Multiplication: ";
echo $x;
echo " * ";
echo $y;
echo " = ";
echo $x * $y;
echo "\\n";

echo "Division: ";
echo $x;
echo " / ";
echo $y;
echo " = ";
echo $x / $y;
echo "\\n";`
  },
  comparisons: {
    name: "⚖️ Comparison Operators",
    code: `<?php
// Complete comparison operators test with integers and floats
echo "=== Comparison Operators Test ===\\n\\n";

// Integer comparisons
$a = 10;
$b = 5;
$c = 10;

echo "Integer values: a = ";
echo $a;
echo ", b = ";
echo $b;
echo ", c = ";
echo $c;
echo "\\n\\n";

// Equality tests
echo "Integer Equality Tests:\\n";
echo "a == b: ";
echo $a == $b;
echo " (false = empty)\\n";
echo "a == c: ";
echo $a == $c;
echo " (true = 1)\\n";

echo "a != b: ";
echo $a != $b;
echo " (true = 1)\\n";
echo "a != c: ";
echo $a != $c;
echo " (false = empty)\\n\\n";

// Integer comparison tests
echo "Integer Comparison Tests:\\n";
echo "a > b: ";
echo $a > $b;
echo " (true = 1)\\n";
echo "b > a: ";
echo $b > $a;
echo " (false = empty)\\n";
echo "a < b: ";
echo $a < $b;
echo " (false = empty)\\n";
echo "b < a: ";
echo $b < $a;
echo " (true = 1)\\n\\n";

// Float comparisons
$x = 10.5;
$y = 3.14;
$z = 10.5;

echo "Float values: x = ";
echo $x;
echo ", y = ";
echo $y;
echo ", z = ";
echo $z;
echo "\\n\\n";

echo "Float Comparison Tests:\\n";
echo "x > y: ";
echo $x > $y;
echo " (true = 1)\\n";
echo "y > x: ";
echo $y > $x;
echo " (false = empty)\\n";
echo "x < y: ";
echo $x < $y;
echo " (false = empty)\\n";
echo "y < x: ";
echo $y < $x;
echo " (true = 1)\\n\\n";

// Mixed int/float comparisons
echo "Mixed Int/Float Comparisons:\\n";
echo "10 > 3.14: ";
echo $a > $y;
echo " (true = 1)\\n";
echo "3.14 < 10: ";
echo $y < $a;
echo " (true = 1)\\n";
echo "5 > 10.5: ";
echo $b > $x;
echo " (false = empty)\\n";
echo "10.5 > 5: ";
echo $x > $b;
echo " (true = 1)\\n";`
  },
  typeCoercion: {
    name: "🔄 Complete Type Coercion (NEW!)",
    code: `<?php
// Complete Type Coercion in == operator (strings to numbers)
echo "=== Complete Type Coercion Demo ===\\n\\n";

echo "PHP == operator automatically converts strings to numbers!\\n\\n";

// ==========================================
// 1. STRING TO INTEGER COERCION
// ==========================================
echo "1. STRING TO INTEGER COERCION:\\n\\n";

echo "Basic integer comparisons:\\n";
echo '"10" == 10: ';
echo "10" == 10;
echo " (true = 1)\\n";

echo '10 == "10": ';
echo 10 == "10";
echo " (true = 1)\\n";

echo '"0" == 0: ';
echo "0" == 0;
echo " (true = 1)\\n";

echo '"-5" == -5: ';
echo "-5" == -5;
echo " (true = 1)\\n\\n";

// ==========================================
// 2. STRING TO FLOAT COERCION (NEW!)
// ==========================================
echo "2. STRING TO FLOAT COERCION (NEW!):\\n\\n";

echo "Float comparisons now working:\\n";
echo '"10.5" == 10.5: ';
echo "10.5" == 10.5;
echo " (true = 1) ✅\\n";

echo '"3.14" == 3.14: ';
echo "3.14" == 3.14;
echo " (true = 1) ✅\\n";

echo '"-2.5" == -2.5: ';
echo "-2.5" == -2.5;
echo " (true = 1) ✅\\n";

echo '"0.0" == 0.0: ';
echo "0.0" == 0.0;
echo " (true = 1) ✅\\n";

// Edge cases
echo "\\nEdge cases:\\n";
echo '".5" == 0.5: ';
echo ".5" == 0.5;
echo " (true = 1) ✅\\n";

echo '"-.5" == -0.5: ';
echo "-.5" == -0.5;
echo " (true = 1) ✅\\n\\n";

// ==========================================
// 3. MIXED TYPE COERCION
// ==========================================
echo "3. MIXED TYPE COERCION:\\n\\n";

echo "String to float, int to float:\\n";
echo '"10" == 10.0: ';
echo "10" == 10.0;
echo " (true = 1)\\n";

echo '10.0 == "10": ';
echo 10.0 == "10";
echo " (true = 1)\\n";

echo '"10.0" == 10: ';
echo "10.0" == 10;
echo " (true = 1)\\n";

echo 'Int/Float: 10 == 10.0: ';
echo 10 == 10.0;
echo " (true = 1)\\n\\n";

// ==========================================
// 4. STRICT COMPARISON (NO COERCION)
// ==========================================
echo "4. STRICT COMPARISON === (NO COERCION):\\n\\n";

echo '"10" === 10: ';
echo "10" === 10;
echo " (false = empty)\\n";

echo '"10.5" === 10.5: ';
echo "10.5" === 10.5;
echo " (false = empty)\\n";

echo '"10" === "10": ';
echo "10" === "10";
echo " (true = 1)\\n";

echo '10.5 === 10.5: ';
echo 10.5 === 10.5;
echo " (true = 1)\\n\\n";

// ==========================================
// 5. INVALID STRINGS (NO CONVERSION)
// ==========================================
echo "5. INVALID STRINGS (NO CONVERSION):\\n\\n";

echo "Invalid numeric strings stay as strings:\\n";
echo '"10.5a" == 10.5: ';
echo "10.5a" == 10.5;
echo " (false = empty)\\n";

echo '"abc" == 10: ';
echo "abc" == 10;
echo " (false = empty)\\n";

echo '"" == 0: ';
echo "" == 0;
echo " (false = empty)\\n";

echo '"hello" == 0.0: ';
echo "hello" == 0.0;
echo " (false = empty)\\n\\n";

// ==========================================
// 6. REAL-WORLD EXAMPLES
// ==========================================
echo "6. REAL-WORLD EXAMPLES:\\n\\n";

// Form data example
$user_input = "25.5";  // String from HTML form
$required_score = 25.5; // Float from database

echo 'User score: "25.5" (string from form)\\n';
echo 'Required: 25.5 (float from database)\\n';
echo 'Score check: ';
echo $user_input == $required_score;
echo " ✅ Works!\\n\\n";

// API data example  
$api_response = "3.14159"; // String from JSON API
$pi = 3.14159;             // Float constant

echo 'API returned: "3.14159" (string)\\n';
echo 'PI constant: 3.14159 (float)\\n';
echo 'Comparison: ';
echo $api_response == $pi;
echo " ✅ Perfect!\\n\\n";

echo "🎉 Complete type coercion working perfectly!\\n";
echo "✅ String-to-int: \\"10\\" == 10\\n";
echo "✅ String-to-float: \\"10.5\\" == 10.5\\n"; 
echo "✅ Mixed types: \\"10\\" == 10.0\\n";
echo "✅ Edge cases: \\".5\\" == 0.5\\n";`
  },
  strings: {
    name: "🔤 String Operations",
    code: `<?php
// String operations showcase
echo "=== String Operations Demo ===\\n";

$greeting = "Hello";
$target = "EdgePHP";
$version = "v1.0";

echo "Variables:\\n";
echo "greeting = ";
echo $greeting;
echo "\\n";
echo "target = ";
echo $target;
echo "\\n";
echo "version = ";
echo $version;
echo "\\n\\n";

echo "String Concatenation:\\n";
$simple = $greeting . " " . $target;
echo "Simple: ";
echo $simple;
echo "\\n";

$complex = $greeting . " " . $target . " " . $version;
echo "Complex: ";
echo $complex;
echo "!\\n\\n";

echo "Mixed with numbers:\\n";
$count = 42;
$result = "The answer is " . $count;
echo $result;
echo "\\n";`
  },
  variables: {
    name: "📦 Variables & Assignment",
    code: `<?php
// Variable assignment and retrieval
echo "=== Variables & Assignment ===\\n";

// Integer variables
$number = 25;
echo "Integer: ";
echo $number;
echo "\\n";

// String variables
$text = "EdgePHP rocks!";
echo "String: ";
echo $text;
echo "\\n";

// Variable reassignment
$number = 50;
echo "Reassigned integer: ";
echo $number;
echo "\\n";

$text = "Now different text";
echo "Reassigned string: ";
echo $text;
echo "\\n";

// Using variables in expressions
$x = 10;
$y = 20;
$result = $x + $y;
echo "Expression result: ";
echo $result;
echo "\\n";`
  },
  mixed: {
    name: "🎯 Mixed Operations",
    code: `<?php
// Complex mixed operations with string interpolation
echo "=== Mixed Operations Demo ===\\n";

$price = 500;
$discount = 0;
$tax = 8;

echo "Price: \\$$price\\n";
echo "Discount: $discount%\\n";
echo "Tax: $tax%\\n\\n";

$discounted = $price - $discount;
echo "After discount: \\$$discounted\\n";

$final = $discounted + $tax;
echo "Final price: \\$$final\\n\\n";

// Boolean results
echo "Is expensive? ";
echo $final > 80;
echo " (1=yes, empty=no)\\n";

echo "Is cheap? ";
echo $final < 50;
echo " (1=yes, empty=no)\\n";`
  },
  comprehensive: {
    name: "🚀 Complete Feature Test",
    code: `<?php
// Comprehensive test of all features
echo "=== EdgePHP Complete Feature Test ===\\n";

// Integer Variables
$x = 15;
$y = 3;
echo "Integer Variables: x=";
echo $x;
echo ", y=";
echo $y;
echo "\\n\\n";

// Integer Arithmetic
echo "Integer Arithmetic:\\n";
echo "x + y = ";
echo $x + $y;
echo "\\n";
echo "x - y = ";
echo $x - $y;
echo "\\n";
echo "x * y = ";
echo $x * $y;
echo "\\n";
echo "x / y = ";
echo $x / $y;
echo "\\n\\n";

// Float Variables
$a = 10.5;
$b = 3.14;
echo "Float Variables: a=";
echo $a;
echo ", b=";
echo $b;
echo "\\n\\n";

// Float Arithmetic
echo "Float Arithmetic:\\n";
echo "a + b = ";
echo $a + $b;
echo "\\n";
echo "a - b = ";
echo $a - $b;
echo "\\n";
echo "a * b = ";
echo $a * $b;
echo "\\n";
echo "a / b = ";
echo $a / $b;
echo "\\n\\n";

// Integer Comparisons
echo "Integer Comparisons:\\n";
echo "x == y: ";
echo $x == $y;
echo "\\n";
echo "x != y: ";
echo $x != $y;
echo "\\n";
echo "x > y: ";
echo $x > $y;
echo "\\n";
echo "x < y: ";
echo $x < $y;
echo "\\n\\n";

// Float Comparisons
echo "Float Comparisons:\\n";
echo "a > b: ";
echo $a > $b;
echo "\\n";
echo "a < b: ";
echo $a < $b;
echo "\\n\\n";

// Strings
$greeting = "Hello";
$name = "World";
echo "String Operations:\\n";
echo "greeting = ";
echo $greeting;
echo "\\n";
echo "name = ";
echo $name;
echo "\\n";
echo "Combined: ";
echo $greeting . " " . $name . "!";
echo "\\n\\n";

// Complex expressions
$result = ($x > $y) == 1;
echo "Complex: (x > y) == 1 is ";
echo $result;
echo "\\n\\n";

echo "🎉 All features working! 🎉\\n";`
  },
  basic: {
    name: "👋 Hello World",
    code: `<?php
// Classic Hello World
echo "Hello, World!";
echo "\\n";
echo "Welcome to EdgePHP!";`
  },
  simple: {
    name: "✨ Quick Demo",
    code: `<?php
// Quick demonstration with string interpolation
$message = "EdgePHP";
$number = 42;
$price = 19.99;

// Using string interpolation (double quotes)
echo "Welcome to $message!\\n";
echo "The answer is: $number\\n";
echo "Price: \\$$price\\n\\n";

// Compare with concatenation
echo "Same result with concat: " . $message . " rocks!\\n";`
  },
  minimal: {
    name: "⚡ Minimal Test",
    code: `<?php
echo "test";`
  },
  assignment: {
    name: "📋 Simple Assignment",
    code: `<?php
$x = 42;
echo $x;`
  },
  phpTags: {
    name: "🏷️ PHP Tags & Inline Content",
    code: `<?php echo 'if you want to serve PHP code in XHTML or XML documents,
                use these tags'; ?>

  You can use the short echo tag to <?= 'print this string' ?>.
    It's equivalent to <?php echo 'print this string' ?>.

  <? echo 'this code is within short tags, but will only work '.
            'if short_open_tag is enabled'; ?>`
  },
  interpolation: {
    name: "📝 String Interpolation & Escapes",
    code: `<?php
// PHP String Interpolation and Escape Sequences Demo
echo "=== String Interpolation & Escape Sequences ===\\n\\n";

// Variables for demonstration
$name = "EdgePHP";
$price = 99.99;
$count = 42;
$user = "developer";

// ==========================================
// 1. DOUBLE QUOTES - Full interpolation and escape sequences
// ==========================================
echo "1. DOUBLE QUOTES - Variables interpolated, escapes processed:\\n";
echo "   Basic interpolation: Welcome to $name!\\n";
echo "   Numbers work too: $count items cost \\$$price\\n";
echo "   At start: $user is here\\n";
echo "   At end: Hello $user\\n";
echo "   Multiple vars: $name has $count users\\n\\n";

// Escape sequences in double quotes
echo "   Escape sequences in double quotes:\\n";
echo "   Newline: Line1\\nLine2\\n";
echo "   Tab: Col1\\tCol2\\tCol3\\n";
echo "   Backslash: C:\\\\path\\\\to\\\\file\\n";
echo "   Quote: She said \\"Hello\\"\\n";
echo "   Dollar: Price is \\$$price USD\\n";  // \\$ = literal $, then $price interpolated
echo "   Return: Text\\rOverwrite\\n";        // \\r = carriage return
echo "\\n";

// ==========================================
// 2. SINGLE QUOTES - NO interpolation, minimal escapes
// ==========================================
echo "2. SINGLE QUOTES - Variables NOT interpolated, only \\' and \\\\ work:\\n";
echo '   No interpolation: Welcome to $name!';
echo "\\n";
echo '   Dollar signs: $count items cost $$price';
echo "\\n";
echo '   Escape sequences stay literal: \\n \\t \\r stay as text';
echo "\\n";
echo '   Only these work: I\\'m here and C:\\\\path';  // Only \\' and \\\\ are processed
echo "\\n\\n";

// ==========================================
// 3. STRING CONCATENATION - Explicit joining
// ==========================================
echo "3. CONCATENATION - Works with both quote types:\\n";
echo "   Using dot: " . $name . " version " . $count . "\\n";
echo "   Mixed quotes: " . 'Price is $' . $price . ' for ' . "$count items\\n";
echo "   Complex: Welcome to " . $name . "! You have " . $count . " items.\\n\\n";

// ==========================================
// 4. SPECIAL CASES & EDGE CASES
// ==========================================
echo "4. SPECIAL CASES:\\n";

// Double dollar signs
$currency = "USD";
echo "   Double dollar: $$currency means \\$ followed by variable\\n";  // $$ = one literal $

// Variables with underscores and numbers
$var_name = "test";
$var2 = "value";
echo "   Underscore vars: $var_name and $var2 work fine\\n";

// Adjacent variables
$first = "Hello";
$second = "World";
echo "   Adjacent: $first$second (no space between)\\n";

// Variable at string boundaries
echo "$name"." at start, at end "."$name\\n";

// ==========================================
// 5. ESCAPE SEQUENCES IN ACTION
// ==========================================
echo "\\n5. ESCAPE SEQUENCES IN ACTION:\\n";
echo "   This line has a\\nnewline in the middle\\n";
echo "   Items:\\tOne\\tTwo\\tThree (tabs)\\n";
echo "   Path: C:\\\\Users\\\\Name\\\\Documents\\n";
echo "   She said: \\"That's amazing!\\"\\n";
echo "   Cost: \\$500.00 (literal dollar sign)\\n";
echo "   Single quote: It\\'s working\\n";
echo "\\n";

// Show the difference between single and double quotes
echo "6. SINGLE vs DOUBLE QUOTE COMPARISON:\\n";
echo "   Double: \\\\n produces newline, \\\\t produces tab\\n";
echo '   Single: \\n stays as \\n, \\t stays as \\t';
echo "\\n\\n";

// ==========================================
// 7. REAL-WORLD EXAMPLES
// ==========================================
echo "7. REAL-WORLD EXAMPLES:\\n";

// SQL-like string
$table = "users";
$id = 123;
echo "   SQL: SELECT * FROM $table WHERE id = $id\\n";

// Path construction
$base = "/home";
$folder = "projects";
$file = "index.php";
echo "   Path: $base/$folder/$file\\n";

// HTML generation
$title = "My Page";
$class = "highlight";
echo "   HTML: <div class=\\"$class\\">$title</div>\\n";

// JSON-like output
$key = "status";
$value = "success";
echo "   JSON: {\\"$key\\": \\"$value\\"}\\n";

echo "\\n✅ Complete string interpolation demo finished!";`
  },
  arrays: {
    name: "🗂️ Array Operations & count()",
    code: `<?php
// PHP Array Operations with count() function
echo "=== PHP Array Operations ===\\n\\n";

// Test 1: Empty array
echo "Test 1: Empty array\\n";
$empty = array();
echo "Empty array: count() = ";
echo count($empty);
echo "\\n\\n";

// Test 2: Array with 3 elements
echo "Test 2: Array with elements\\n";
$numbers = array(1, 2, 3);
echo "array(1, 2, 3): count() = ";
echo count($numbers);
echo "\\n\\n";

// Test 3: Large array
echo "Test 3: Large array\\n";
$large = array(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
echo "array(1-10): count() = ";
echo count($large);
echo "\\n\\n";

// Test 4: Mixed types
echo "Test 4: Mixed types\\n";
$mixed = array(42, "hello", 3, "world");
echo "Mixed array: count() = ";
echo count($mixed);
echo "\\n\\n";

// Test 5: String and number arrays
echo "Test 5: String array\\n";
$words = array("PHP", "is", "awesome");
echo "Word array: count() = ";
echo count($words);
echo "\\n\\n";

// Test 6: Associative arrays (string keys)
echo "Test 6: Associative arrays\\n";
$person = ["name" => "John", "age" => 30];
echo "person array: count() = ";
echo count($person);
echo "\\n\\n";
// Test 7: Short array syntax []
echo "Test 7: Short syntax []\\n";
$short = [1, 2, 3, "four", "five"];
echo "[1, 2, 3, 'four', 'five']: count() = ";
echo count($short);
echo "\\n\\n";
// Test 8: Mixed key types
echo "Test 8: Mixed key types\\n";
$mixedKeys = [0 => "first", "key" => "value", 2 => "third"];
echo "Mixed keys array: count() = ";
echo count($mixedKeys);
echo "\\n\\n";
echo "✅ Array operations completed!\\n";
echo "📊 PHP count() function working perfectly!\\n";
echo "🔥 Associative arrays & short syntax working!";`
  },
  
  arrayAssignment: {
    name: "📝 Array Assignment",
    code: `<?php
// Array element assignment demonstration
echo "=== Array Assignment Demo ===\\n\\n";

// Test 1: Simple array assignment
echo "Test 1: Simple array assignment\\n";
$numbers = [10, 20, 30];
echo "Original: ";
echo $numbers[0] . ", " . $numbers[1] . ", " . $numbers[2];
echo "\\n";

$numbers[0] = 100;
$numbers[1] = 200;
$numbers[2] = 300;
echo "Modified: ";
echo $numbers[0] . ", " . $numbers[1] . ", " . $numbers[2];
echo "\\n\\n";

// Test 2: Associative array assignment
echo "Test 2: Associative array assignment\\n";
$person = ["name" => "John", "age" => 25, "city" => "Boston"];
echo "Original: " . $person["name"] . " is " . $person["age"] . " from " . $person["city"];
echo "\\n";

$person["name"] = "Jane";
$person["age"] = 30;
$person["city"] = "NYC";
echo "Modified: " . $person["name"] . " is " . $person["age"] . " from " . $person["city"];
echo "\\n\\n";

// Test 3: Adding new elements
echo "Test 3: Adding new elements\\n";
$data = ["first" => 1];
echo "Initial count: " . count($data) . "\\n";

$data["second"] = 2;
$data["third"] = 3;
echo "After adding: count = " . count($data);
echo ", second = " . $data["second"];
echo ", third = " . $data["third"];
echo "\\n\\n";

// Test 4: Mixed key types
echo "Test 4: Mixed key types\\n";
$mixed = [];
$mixed[0] = "zero";
$mixed["one"] = 1;
$mixed[2] = "two";
$mixed["three"] = 3;

echo "mixed[0] = " . $mixed[0] . "\\n";
echo "mixed['one'] = " . $mixed["one"] . "\\n";
echo "mixed[2] = " . $mixed[2] . "\\n";
echo "mixed['three'] = " . $mixed["three"] . "\\n";
echo "Total count: " . count($mixed) . "\\n\\n";

echo "✅ Array assignment working perfectly!";`
  },
  
  arrayOperations: {
    name: "🛠️ Array Operations",
    code: `<?php
// Comprehensive array operations
echo "=== Array Operations Demo ===\\n\\n";

// Test 1: Array creation and access
echo "Test 1: Array creation and access\\n";
$fruits = ["apple", "banana", "orange", "grape"];
echo "Fruits: " . $fruits[0] . ", " . $fruits[1] . ", " . $fruits[2] . ", " . $fruits[3];
echo " (count: " . count($fruits) . ")\\n\\n";

// Test 2: Associative arrays
echo "Test 2: Associative arrays\\n";
$prices = [
    "apple" => 1.50,
    "banana" => 0.75,
    "orange" => 2.00
];
echo "Apple: $" . $prices["apple"] . "\\n";
echo "Banana: $" . $prices["banana"] . "\\n";
echo "Orange: $" . $prices["orange"] . "\\n\\n";

// Test 3: Nested arrays
echo "Test 3: Nested arrays\\n";
$matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
];
echo "Matrix[1][1] = " . $matrix[1][1] . " (should be 5)\\n";
echo "Matrix[0][2] = " . $matrix[0][2] . " (should be 3)\\n";
echo "Matrix[2][0] = " . $matrix[2][0] . " (should be 7)\\n\\n";

// Test 4: Array modification
echo "Test 4: Array modification\\n";
$scores = [85, 90, 78];
echo "Original scores: " . $scores[0] . ", " . $scores[1] . ", " . $scores[2] . "\\n";

$scores[0] = $scores[0] + 5;
$scores[1] = $scores[1] - 2;
$scores[2] = $scores[2] + 10;
echo "Updated scores: " . $scores[0] . ", " . $scores[1] . ", " . $scores[2] . "\\n\\n";

// Test 5: Dynamic array building
echo "Test 5: Dynamic array building\\n";
$dynamic = [];
$dynamic["first"] = "Hello";
$dynamic["second"] = "World";
$dynamic[0] = "EdgePHP";
$dynamic[1] = "Arrays";

echo $dynamic["first"] . " " . $dynamic["second"] . "! ";
echo $dynamic[0] . " " . $dynamic[1] . " work great!\\n\\n";

// Test 6: Empty array to hash table conversion
echo "Test 6: Empty array conversion\\n";
$empty = [];
echo "Initial empty array count: " . count($empty) . "\\n";
$empty["key1"] = "value1";
echo "After adding string key, count: " . count($empty) . "\\n";
$empty[0] = "numeric";
echo "After adding numeric key, count: " . count($empty) . "\\n";
echo "Values: " . $empty["key1"] . ", " . $empty[0] . "\\n\\n";

// Test 7: Mixed key types in same array
echo "Test 7: Mixed key types\\n";
$mixed = [];
$mixed[0] = "zero";
$mixed["one"] = 1;
$mixed[2] = "two";
$mixed["three"] = 3.0;
echo "Count: " . count($mixed) . "\\n";
echo "Values: " . $mixed[0] . ", " . $mixed["one"] . ", " . $mixed[2] . ", " . $mixed["three"] . "\\n\\n";

// Test 8: Float values in arrays
echo "Test 8: Float values display\\n";
$floats = [
    "pi" => 3.14159,
    "e" => 2.71828,
    "golden" => 1.618,
    "whole" => 5.0
];
echo "Pi: " . $floats["pi"] . "\\n";
echo "E: " . $floats["e"] . "\\n";
echo "Golden ratio: " . $floats["golden"] . "\\n";
echo "Whole number float: " . $floats["whole"] . "\\n\\n";

// Test 9: Key normalization
echo "Test 9: Key normalization (string->int)\\n";
$keys = [];
$keys["456"] = "string 456";
$keys[456] = "int 456 (overwrites)";
$keys["0789"] = "leading zero (stays string)";
$keys["abc"] = "pure string";
echo "keys['456'] = " . $keys["456"] . "\\n";
echo "keys[456] = " . $keys[456] . "\\n";
echo "keys['0789'] = " . $keys["0789"] . "\\n";
echo "keys['abc'] = " . $keys["abc"] . "\\n";
echo "Count: " . count($keys) . " (should be 3)\\n\\n";

echo "✅ All array operations completed successfully!";`
  },
  
  controlFlow: {
    name: "🔄 Control Flow (if/else/while)",
    code: `<?php
// PHP Control Flow Statements Demo
echo "=== Control Flow Demo ===\\n\\n";

// ==========================================
// 1. IF/ELSE STATEMENTS
// ==========================================
echo "1. IF/ELSE STATEMENTS:\\n\\n";

$score = 85;
echo "Score: $score\\n";

// Simple if with elseif blocks (PHP standard syntax)
if ($score > 90) {
    echo "Grade: A (Excellent!)\\n";
} elseif ($score > 80) {
    echo "Grade: B (Good job!)\\n";
} elseif ($score > 70) {
    echo "Grade: C (Satisfactory)\\n";
} elseif ($score > 60) {
    echo "Grade: D (Needs improvement)\\n";
} else {
    echo "Grade: F (Failed)\\n";
}

// Nested if statements
$age = 25;
$hasLicense = 1; // true
echo "\\nAge: $age, Has License: $hasLicense\\n";

if ($age >= 18) {
    echo "You are an adult.\\n";
    if ($hasLicense) {
        echo "You can drive!\\n";
    } else {
        echo "But you need a license to drive.\\n";
    }
} else {
    echo "You are a minor.\\n";
}

// ==========================================
// 2. WHILE LOOPS
// ==========================================
echo "\\n2. WHILE LOOPS:\\n\\n";

// Count down from 5
echo "Countdown: ";
$count = 5;
while ($count > 0) {
    echo $count . " ";
    $count = $count - 1;
}
echo "Liftoff!\\n";

// Sum numbers 1 to 10
$num = 1;
$sum = 0;
while ($num <= 10) {
    $sum = $sum + $num;
    $num = $num + 1;
}
echo "Sum of 1 to 10: $sum\\n";

// Fibonacci sequence
echo "\\nFibonacci (first 10 numbers): ";
$a = 0;
$b = 1;
$counter = 0;
while ($counter < 10) {
    echo $a . " ";
    $temp = $a + $b;
    $a = $b;
    $b = $temp;
    $counter = $counter + 1;
}
echo "\\n";

// ==========================================
// 3. FOR LOOPS
// ==========================================
echo "\\n3. FOR LOOPS:\\n\\n";

// Basic for loop
echo "Count to 5: ";
for ($i = 1; $i <= 5; $i = $i + 1) {
    echo $i . " ";
}
echo "\\n";

// For loop with array
echo "\\nSquares of 1-5: ";
for ($j = 1; $j <= 5; $j = $j + 1) {
    $square = $j * $j;
    echo $square . " ";
}
echo "\\n";

// Nested for loops (multiplication table)
echo "\\nMultiplication table (3x3):\\n";
for ($row = 1; $row <= 3; $row = $row + 1) {
    for ($col = 1; $col <= 3; $col = $col + 1) {
        $product = $row * $col;
        echo $product . "\\t";
    }
    echo "\\n";
}

// ==========================================
// 4. COMBINING IF AND WHILE
// ==========================================
echo "\\n4. COMBINING IF AND LOOPS:\\n\\n";

// Find first number divisible by 7 after 50
$n = 51;
$found = 0;
while ($found == 0) {
    if ($n % 7 == 0) {
        echo "First number divisible by 7 after 50: $n\\n";
        $found = 1;
    } else {
        $n = $n + 1;
    }
}

// Count even and odd numbers
echo "\\nCounting even/odd from 1 to 10:\\n";
$i = 1;
$even = 0;
$odd = 0;
while ($i <= 10) {
    if ($i % 2 == 0) {
        $even = $even + 1;
    } else {
        $odd = $odd + 1;
    }
    $i = $i + 1;
}
echo "Even numbers: $even\\n";
echo "Odd numbers: $odd\\n";

// ==========================================
// 5. TERNARY OPERATOR (?:)
// ==========================================
echo "\\n5. TERNARY OPERATOR (?:):\\n\\n";

// Basic ternary
$age = 25;
$status = $age >= 18 ? "adult" : "minor";
echo "Age $age: You are an $status\\n";

// Numeric ternary
$score = 85;
$grade = $score >= 90 ? "A" : ($score >= 80 ? "B" : "C");
echo "Score $score: Grade $grade\\n";

// Ternary with expressions
$x = 10;
$y = 5;
$max = $x > $y ? $x : $y;
echo "Max of $x and $y is: $max\\n";

// Nested ternary
$hour = 14;
$greeting = $hour < 12 ? "Good morning" : ($hour < 18 ? "Good afternoon" : "Good evening");
echo "At hour $hour: $greeting\\n";

// Ternary in echo
$items = 1;
echo "You have " . $items . " item" . ($items == 1 ? "" : "s") . "\\n";

echo "\\n✅ Control flow statements working!";`
  },
  
  breakContinue: {
    name: "🔄 Break & Continue Statements",
    code: `<?php
// PHP Break and Continue Statements Demo
echo "=== Break & Continue Statements Demo ===\\n\\n";

// ==========================================
// 1. BREAK - Exit the loop entirely
// ==========================================
echo "1. BREAK - Exit loop entirely:\\n\\n";

// Break with FOR loop - stop at specific value
echo "FOR loop with break (stop at 3):\\n";
for ($i = 1; $i <= 5; $i = $i + 1) {
    if ($i == 3) {
        echo "Breaking at i = $i\\n";
        break;
    }
    echo "i = $i\\n";
}
echo "After for loop\\n\\n";

// Break with WHILE loop - stop at specific value
echo "WHILE loop with break (stop at 3):\\n";
$j = 1;
while ($j <= 5) {
    if ($j == 3) {
        echo "Breaking at j = $j\\n";
        break;
    }
    echo "j = $j\\n";
    $j = $j + 1;
}
echo "After while loop\\n\\n";

// ==========================================
// 2. CONTINUE - Skip current iteration
// ==========================================
echo "2. CONTINUE - Skip current iteration:\\n\\n";

// Continue with FOR loop - skip specific value
echo "FOR loop with continue (skip 3):\\n";
for ($k = 1; $k <= 5; $k = $k + 1) {
    if ($k == 3) {
        echo "Continuing at k = $k\\n";
        continue;
    }
    echo "k = $k\\n";
}
echo "After for loop with continue\\n\\n";

// Continue with WHILE loop - skip specific value
echo "WHILE loop with continue (skip 3):\\n";
$m = 0;
while ($m < 5) {
    $m = $m + 1;
    if ($m == 3) {
        echo "Continuing at m = $m\\n";
        continue;
    }
    echo "m = $m\\n";
}
echo "After while loop with continue\\n\\n";

// ==========================================
// 3. PRACTICAL EXAMPLES
// ==========================================
echo "3. PRACTICAL EXAMPLES:\\n\\n";

// Skip even numbers using continue
echo "Print odd numbers 1-10 (using continue):\\n";
for ($n = 1; $n <= 10; $n = $n + 1) {
    if ($n % 2 == 0) {
        continue;  // Skip even numbers
    }
    echo $n . " ";
}
echo "\\n\\n";

// Find first occurrence using break
echo "Find first number divisible by 7:\\n";
for ($p = 1; $p <= 20; $p = $p + 1) {
    if ($p % 7 == 0) {
        echo "Found: $p is divisible by 7\\n";
        break;  // Exit loop once found
    }
}
echo "\\n";

// Break in nested loops (only breaks inner loop)
echo "Nested loops with break:\\n";
for ($row = 1; $row <= 3; $row = $row + 1) {
    echo "Row $row: ";
    for ($col = 1; $col <= 5; $col = $col + 1) {
        if ($col == 3) {
            break;  // Only breaks inner loop
        }
        echo "$col ";
    }
    echo "\\n";
}
echo "\\n";

// Continue in nested loops (only affects inner loop)
echo "Nested loops with continue:\\n";
for ($row = 1; $row <= 3; $row = $row + 1) {
    echo "Row $row: ";
    for ($col = 1; $col <= 5; $col = $col + 1) {
        if ($col == 3) {
            continue;  // Only skips in inner loop
        }
        echo "$col ";
    }
    echo "\\n";
}
echo "\\n";

// ==========================================
// 4. IMPORTANT NOTES
// ==========================================
echo "IMPORTANT NOTES:\\n";
echo "- Break exits the current loop completely\\n";
echo "- Continue skips to the next iteration\\n";
echo "- In for loops, continue executes the update expression\\n";
echo "- In nested loops, break/continue only affect the innermost loop\\n";

echo "\\n✅ Break and Continue statements working!";`
  },
  
  foreach: {
    name: "🔁 Foreach Loops",
    code: `<?php
// PHP Foreach Loop Demo
echo "=== Foreach Loop Demo ===\\n\\n";

// ==========================================
// 1. SIMPLE FOREACH - Values only
// ==========================================
echo "1. SIMPLE FOREACH - Values only:\\n\\n";

// Iterate over array values
$fruits = ["apple", "banana", "orange", "grape"];
echo "Fruits:\\n";
foreach ($fruits as $fruit) {
    echo "  - " . $fruit . "\\n";
}

// Numeric array
echo "\\nNumbers 1-5:\\n";
$numbers = [1, 2, 3, 4, 5];
foreach ($numbers as $num) {
    echo "  Number: " . $num . "\\n";
}

// ==========================================
// 2. FOREACH WITH KEY => VALUE
// ==========================================
echo "\\n2. FOREACH WITH KEY => VALUE:\\n\\n";

// Get both index and value
$colors = ["red", "green", "blue"];
echo "Colors with indices:\\n";
foreach ($colors as $index => $color) {
    echo "  [" . $index . "] => " . $color . "\\n";
}

// ==========================================
// 3. ASSOCIATIVE ARRAYS (NEW!)
// ==========================================
echo "\\n3. ASSOCIATIVE ARRAYS:\\n\\n";

// String keys
echo "Person data (string keys):\\n";
$person = array();
$person["name"] = "Alice";
$person["age"] = 25;
$person["city"] = "Seattle";
$person["job"] = "Developer";
foreach ($person as $key => $value) {
    echo "  " . $key . " => " . $value . "\\n";
}

// Mixed key types
echo "\\nMixed key types:\\n";
$mixed = array();
$mixed[0] = "first";
$mixed["key1"] = "value1";
$mixed[1] = "second"; 
$mixed["key2"] = "value2";
foreach ($mixed as $k => $v) {
    echo "  [" . $k . "] = " . $v . "\\n";
}

// ==========================================
// 4. FOREACH WITH BREAK
// ==========================================
echo "\\n4. FOREACH WITH BREAK:\\n\\n";

// Find first number over 50
$values = [10, 25, 37, 52, 48, 65, 71];
echo "Finding first value over 50:\\n";
foreach ($values as $val) {
    echo "  Checking: " . $val;
    if ($val > 50) {
        echo " - Found it!\\n";
        break;
    }
    echo " - Too small\\n";
}

// ==========================================
// 5. FOREACH WITH CONTINUE
// ==========================================
echo "\\n5. FOREACH WITH CONTINUE:\\n\\n";

// Skip even numbers
$range = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
echo "Odd numbers only:\\n";
foreach ($range as $n) {
    if ($n % 2 == 0) {
        continue; // Skip even numbers
    }
    echo "  " . $n . " ";
}
echo "\\n";

// ==========================================
// 6. PRACTICAL EXAMPLES
// ==========================================
echo "\\n6. PRACTICAL EXAMPLES:\\n\\n";

// Calculate sum and average
$grades = [85, 92, 78, 95, 88];
$total = 0;
$count = 0;
echo "Calculating grade average:\\n";
foreach ($grades as $grade) {
    echo "  Grade: " . $grade . "\\n";
    $total = $total + $grade;
    $count = $count + 1;
}
$average = $total / $count;
echo "Total: " . $total . "\\n";
echo "Count: " . $count . "\\n";
echo "Average: " . $average . "\\n";

// Build a string from array
echo "\\nBuilding a string:\\n";
$words = ["EdgePHP", "is", "awesome", "!"];
$sentence = "";
foreach ($words as $i => $word) {
    if ($i > 0) {
        $sentence = $sentence . " ";
    }
    $sentence = $sentence . $word;
}
echo "Result: " . $sentence . "\\n";

// Nested foreach loops
echo "\\nNested foreach (mixed arrays):\\n";
$data = array();
$data["users"] = ["Alice", "Bob", "Charlie"];
$data["products"] = ["Laptop", "Phone", "Tablet"];
foreach ($data as $category => $items) {
    echo $category . ":\\n";
    foreach ($items as $idx => $item) {
        echo "  [" . $idx . "] " . $item . "\\n";
    }
}

echo "\\n✅ Foreach loops with associative arrays working perfectly!";`
  },
  
  arrayMergeSlice: {
    name: "🔀 Array Merge & Slice",
    code: `<?php
// PHP array_merge() and array_slice() functions
echo "=== Array Merge & Slice Demo ===\\n\\n";

// ==========================================
// 1. array_merge() - Combines arrays
// ==========================================
echo "1. ARRAY_MERGE - Combines multiple arrays:\\n\\n";

// Simple numeric arrays (both syntaxes work!)
$arr1 = [1, 2, 3];      // Short array syntax
$arr2 = array(4, 5, 6); // Traditional syntax
$arr3 = [7, 8, 9];      // Both are supported!

echo "Arrays to merge:\\n";
echo "arr1: [1, 2, 3]\\n";
echo "arr2: [4, 5, 6]\\n";
echo "arr3: [7, 8, 9]\\n\\n";

// Merge two arrays
$merged = array_merge($arr1, $arr2);
echo "array_merge(arr1, arr2):\\n";
echo "Result: [" . $merged[0] . ", " . $merged[1] . ", " . $merged[2] . ", " . $merged[3] . ", " . $merged[4] . ", " . $merged[5] . "]\\n";
echo "Count: " . count($merged) . "\\n\\n";

// Merge three arrays
$merged3 = array_merge($arr1, $arr2, $arr3);
echo "array_merge(arr1, arr2, arr3):\\n";
echo "First 3: [" . $merged3[0] . ", " . $merged3[1] . ", " . $merged3[2] . "]\\n";
echo "Last 3: [" . $merged3[6] . ", " . $merged3[7] . ", " . $merged3[8] . "]\\n";
echo "Count: " . count($merged3) . "\\n\\n";

// Merge with string values
$colors1 = ["red", "green"];     // Short syntax
$colors2 = ["blue", "yellow"];   // Works great!
$allColors = array_merge($colors1, $colors2);
echo "Merging string arrays:\\n";
echo "colors1: [red, green]\\n";
echo "colors2: [blue, yellow]\\n";
echo "Merged: [" . $allColors[0] . ", " . $allColors[1] . ", " . $allColors[2] . ", " . $allColors[3] . "]\\n\\n";

// ==========================================
// 2. array_slice() - Extract portion of array
// ==========================================
echo "\\n2. ARRAY_SLICE - Extract portions of arrays:\\n\\n";

// Test array
$numbers = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];  // Short syntax
echo "Original array: [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]\\n\\n";

// Slice from position 2, take 4 elements
$slice1 = array_slice($numbers, 2, 4);
echo "array_slice(numbers, 2, 4) - Start at index 2, take 4 elements:\\n";
echo "Result: [" . $slice1[0] . ", " . $slice1[1] . ", " . $slice1[2] . ", " . $slice1[3] . "]\\n";
echo "Count: " . count($slice1) . "\\n\\n";

// Slice from position 5 to end
$slice2 = array_slice($numbers, 5);
echo "array_slice(numbers, 5) - Start at index 5, take rest:\\n";
echo "Result: [" . $slice2[0] . ", " . $slice2[1] . ", " . $slice2[2] . ", " . $slice2[3] . ", " . $slice2[4] . "]\\n";
echo "Count: " . count($slice2) . "\\n\\n";

// Negative offset - start from end
$slice3 = array_slice($numbers, -3);
echo "array_slice(numbers, -3) - Last 3 elements:\\n";
echo "Result: [" . $slice3[0] . ", " . $slice3[1] . ", " . $slice3[2] . "]\\n";
echo "Count: " . count($slice3) . "\\n\\n";

// Slice with negative length
$slice4 = array_slice($numbers, 1, -7);
echo "array_slice(numbers, 1, -7) - From index 1, exclude last 7:\\n";
echo "Result: [" . $slice4[0] . ", " . $slice4[1] . "]\\n";
echo "Count: " . count($slice4) . "\\n\\n";

// ==========================================
// 3. Practical Examples
// ==========================================
echo "\\n3. PRACTICAL EXAMPLES:\\n\\n";

// Pagination example
$items = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
$page = 2; // Page 2
$perPage = 3;
$offset = ($page - 1) * $perPage;
$pageItems = array_slice($items, $offset, $perPage);
echo "Pagination Example (Page $page, $perPage per page):\\n";
echo "Items: [" . $pageItems[0] . ", " . $pageItems[1] . ", " . $pageItems[2] . "]\\n\\n";

// Combining results
$search1 = ["result1", "result2"];
$search2 = ["result3", "result4"];
$search3 = ["result5"];
$allResults = array_merge($search1, $search2, $search3);
echo "Combining search results:\\n";
echo "Total results: " . count($allResults) . "\\n";
echo "First: " . $allResults[0] . ", Last: " . $allResults[4] . "\\n\\n";

echo "✅ Array merge and slice functions working perfectly!";`
  },

  typeCasting: {
    name: "🎭 Type Casting (Phase 8)",
    code: `<?php
// PHP Type Casting - Explicit type conversions
echo "=== Type Casting Demo ===\\n\\n";

// ==========================================
// 1. (int) - Cast to Integer
// ==========================================
echo "1. (int) - Cast to Integer:\\n\\n";

$floatNum = 10.7;
echo "Float: $floatNum\\n";
echo "Cast to int: ";
echo (int)$floatNum;
echo " (truncates decimal)\\n\\n";

$stringNum = "42";
echo "String: \\"$stringNum\\"\\n";
echo "Cast to int: ";
echo (int)$stringNum;
echo "\\n\\n";

$boolTrue = 1;  // true
echo "Boolean true: $boolTrue\\n";
echo "Cast to int: ";
echo (int)$boolTrue;
echo "\\n\\n";

// ==========================================
// 2. (float) - Cast to Float
// ==========================================
echo "2. (float) - Cast to Float:\\n\\n";

$intNum = 42;
echo "Integer: $intNum\\n";
echo "Cast to float: ";
echo (float)$intNum;
echo "\\n\\n";

$stringFloat = "3.14";
echo "String: \\"$stringFloat\\"\\n";
echo "Cast to float: ";
echo (float)$stringFloat;
echo "\\n\\n";

// ==========================================
// 3. (string) - Cast to String
// ==========================================
echo "3. (string) - Cast to String:\\n\\n";

$num = 123;
echo "Number: $num\\n";
echo "Cast to string: \\"";
echo (string)$num;
echo "\\"\\n\\n";

$floatVal = 45.67;
echo "Float: $floatVal\\n";
echo "Cast to string: \\"";
echo (string)$floatVal;
echo "\\"\\n\\n";

// ==========================================
// 4. (bool) - Cast to Boolean
// ==========================================
echo "4. (bool) - Cast to Boolean:\\n\\n";

$zero = 0;
echo "Zero (0): ";
echo (bool)$zero;
echo " (false = empty output)\\n";

$nonZero = 42;
echo "Non-zero (42): ";
echo (bool)$nonZero;
echo " (true = 1)\\n\\n";

$emptyStr = "";
echo "Empty string: ";
echo (bool)$emptyStr;
echo " (false)\\n";

$nonEmptyStr = "hello";
echo "Non-empty string: ";
echo (bool)$nonEmptyStr;
echo " (true = 1)\\n\\n";

// ==========================================
// 5. Nested Casts
// ==========================================
echo "5. Nested Casts:\\n\\n";

$val = "10.9";
echo "Start: string \\"$val\\"\\n";
echo "Cast (int)(float)\\"$val\\": ";
echo (int)(float)$val;
echo " (10.9 -> 10)\\n\\n";

echo "✅ Type casting working perfectly!";`
  },

  incrementDecrement: {
    name: "➕➖ Increment/Decrement (Phase 9)",
    code: `<?php
// PHP Increment/Decrement Operators
echo "=== Increment/Decrement Operators ===\\n\\n";

// ==========================================
// 1. Pre-increment (++$x)
// ==========================================
echo "1. Pre-increment (++\\$x):\\n";
echo "Increments first, then returns new value\\n\\n";

$a = 5;
echo "Before: \\$a = $a\\n";
echo "++\\$a = ";
echo ++$a;
echo "\\n";
echo "After: \\$a = $a\\n\\n";

// ==========================================
// 2. Post-increment ($x++)
// ==========================================
echo "2. Post-increment (\\$x++):\\n";
echo "Returns current value, then increments\\n\\n";

$b = 5;
echo "Before: \\$b = $b\\n";
echo "\\$b++ = ";
echo $b++;
echo " (returns old value)\\n";
echo "After: \\$b = $b\\n\\n";

// ==========================================
// 3. Pre-decrement (--$x)
// ==========================================
echo "3. Pre-decrement (--\\$x):\\n";
echo "Decrements first, then returns new value\\n\\n";

$c = 5;
echo "Before: \\$c = $c\\n";
echo "--\\$c = ";
echo --$c;
echo "\\n";
echo "After: \\$c = $c\\n\\n";

// ==========================================
// 4. Post-decrement ($x--)
// ==========================================
echo "4. Post-decrement (\\$x--):\\n";
echo "Returns current value, then decrements\\n\\n";

$d = 5;
echo "Before: \\$d = $d\\n";
echo "\\$d-- = ";
echo $d--;
echo " (returns old value)\\n";
echo "After: \\$d = $d\\n\\n";

// ==========================================
// 5. In Expressions
// ==========================================
echo "5. In Expressions:\\n\\n";

$e = 10;
echo "Start: \\$e = $e\\n";
echo "\\$e++ + 5 = ";
echo $e++ + 5;
echo " (10 + 5)\\n";
echo "After: \\$e = $e\\n\\n";

echo "++\\$e + 5 = ";
echo ++$e + 5;
echo " (12 + 5)\\n";
echo "After: \\$e = $e\\n\\n";

echo "✅ Increment/decrement operators working!";`
  },

  compoundAssignment: {
    name: "🔢 Compound Assignment (Phase 9)",
    code: `<?php
// PHP Compound Assignment Operators
echo "=== Compound Assignment Operators ===\\n\\n";

// ==========================================
// 1. += (Addition Assignment)
// ==========================================
echo "1. += (Addition Assignment):\\n\\n";

$a = 10;
echo "Start: \\$a = $a\\n";
$a += 5;
echo "After \\$a += 5: $a\\n";
$a += 15;
echo "After \\$a += 15: $a\\n\\n";

// ==========================================
// 2. -= (Subtraction Assignment)
// ==========================================
echo "2. -= (Subtraction Assignment):\\n\\n";

$b = 100;
echo "Start: \\$b = $b\\n";
$b -= 30;
echo "After \\$b -= 30: $b\\n";
$b -= 20;
echo "After \\$b -= 20: $b\\n\\n";

// ==========================================
// 3. *= (Multiplication Assignment)
// ==========================================
echo "3. *= (Multiplication Assignment):\\n\\n";

$c = 5;
echo "Start: \\$c = $c\\n";
$c *= 3;
echo "After \\$c *= 3: $c\\n";
$c *= 2;
echo "After \\$c *= 2: $c\\n\\n";

// ==========================================
// 4. /= (Division Assignment)
// ==========================================
echo "4. /= (Division Assignment):\\n\\n";

$d = 100;
echo "Start: \\$d = $d\\n";
$d /= 5;
echo "After \\$d /= 5: $d\\n";
$d /= 4;
echo "After \\$d /= 4: $d\\n\\n";

// ==========================================
// 5. Chaining Operations
// ==========================================
echo "5. Chaining All Operations:\\n\\n";

$e = 10;
echo "Start: \\$e = $e\\n";

$e += 20;  // 30
echo "After += 20: $e\\n";

$e -= 5;   // 25
echo "After -= 5: $e\\n";

$e *= 2;   // 50
echo "After *= 2: $e\\n";

$e /= 5;   // 10
echo "After /= 5: $e\\n\\n";

// ==========================================
// 6. With Expressions
// ==========================================
echo "6. With Expressions:\\n\\n";

$f = 100;
echo "Start: \\$f = $f\\n";

$f += 10 + 5;  // Add 15
echo "After \\$f += 10 + 5: $f\\n";

$f -= 3 * 2;   // Subtract 6
echo "After \\$f -= 3 * 2: $f\\n";

$f *= 2 + 1;   // Multiply by 3
echo "After \\$f *= 2 + 1: $f\\n";

$f /= 6 / 2;   // Divide by 3
echo "After \\$f /= 6 / 2: $f\\n\\n";

echo "✅ Compound assignment operators working perfectly!";`
  }
};

function App() {
  const [code, setCode] = useState(DEFAULT_CODE)
  const [output, setOutput] = useState('')
  const [isRunning, setIsRunning] = useState(false)
  const [isExecuting, setIsExecuting] = useState(false)
  const [error, setError] = useState(null)
  const [wasmReady, setWasmReady] = useState(false)
  const [showAst, setShowAst] = useState(false)
  const [compiledWasm, setCompiledWasm] = useState(null)
  const [runtime] = useState(() => new EdgePHPRuntime())
  const [currentExample, setCurrentExample] = useState('default')
  const [lastExecutionTime, setLastExecutionTime] = useState(null)
  const [showBenchmarks, setShowBenchmarks] = useState(false)

  useEffect(() => {
    // Load the Edge PHP WASM module
    loadWasmModule()
  }, [])

  const loadWasmModule = async () => {
    try {
      await init()
      setWasmReady(true)
      setOutput('Edge PHP WASM Compiler loaded successfully!')
    } catch (err) {
      setError(`Failed to load Edge PHP runtime: ${err.message}`)
      console.error('WASM load error:', err)
    }
  }

  const compileCode = async () => {
    if (!wasmReady) {
      setError('WASM module not loaded yet')
      return
    }

    setIsRunning(true)
    setError(null)
    setOutput('Compiling...')
    setCompiledWasm(null)

    try {
      // Start timing compilation
      const compileStart = performance.now()
      
      // Compile the PHP code
      const result = compile_php(code)
      
      const compileEnd = performance.now()
      const compileTime = compileEnd - compileStart
      
      if (result.success) {
        const outputText = []
        
        // Show AST if enabled
        if (showAst && result.ast) {
          outputText.push('=== Abstract Syntax Tree ===')
          outputText.push(result.ast)
          outputText.push('')
        }

        // Show compilation result with performance metrics
        outputText.push('=== Compilation Result ===')
        outputText.push(`✅ Compilation successful!`)
        outputText.push(`📊 Performance Metrics:`)
        outputText.push(`   • Compile time: ${formatTime(compileTime)}`)
        outputText.push(`   • WASM size: ${formatBytes(result.wasm_bytes.length)}`)
        outputText.push(`   • Code size: ${formatBytes(new Blob([code]).size)}`)
        outputText.push('')
        outputText.push('Click "Run" to execute the compiled code!')
        
        setOutput(outputText.join('\n'))
        setCompiledWasm(result.wasm_bytes)
      } else {
        setError(result.error || 'Compilation failed')
      }
    } catch (err) {
      setError(`Compilation error: ${err.message}`)
      console.error('Compilation error:', err)
    } finally {
      setIsRunning(false)
    }
  }

  const runCode = async () => {
    if (!compiledWasm) {
      setError('No compiled code to run. Click "Compile" first.')
      return
    }

    setIsExecuting(true)
    setError(null)
    setOutput('Executing...')

    try {
      // Convert the wasm bytes array to Uint8Array
      const wasmBytes = new Uint8Array(compiledWasm)
      
      // Execute the compiled WASM (runtime will measure pure execution time)
      const result = await runtime.execute(wasmBytes)
      
      if (result.success) {
        const outputText = []
        
        outputText.push('=== Program Output ===')
        outputText.push(result.output || '(no output)')
        outputText.push('')
        outputText.push('✅ Execution completed successfully!')
        outputText.push(`⚡ Execution time: ${formatTime(result.executionTime)}`)
        
        setOutput(outputText.join('\n'))
        setLastExecutionTime(result.executionTime)
      } else {
        setError(`Runtime error: ${result.error}`)
        if (result.output) {
          setOutput(`Partial output:\n${result.output}\n\nError: ${result.error}`)
        }
      }
    } catch (err) {
      setError(`Execution error: ${err.message}`)
      console.error('Execution error:', err)
    } finally {
      setIsExecuting(false)
    }
  }

  const testExamples = async () => {
    setError(null)
    setOutput('Testing example programs...')

    try {
      const examples = [
        { name: 'Hello World', code: EXAMPLES.basic.code },
        { name: 'Arithmetic', code: EXAMPLES.arithmetic.code },
        { name: 'Strings', code: EXAMPLES.strings.code },
        { name: 'Arrays & count()', code: EXAMPLES.arrays.code },
        { name: 'Comprehensive', code: EXAMPLES.comprehensive.code }
      ]

      const results = []
      
      for (const example of examples) {
        try {
          results.push(`\n=== ${example.name} ===`)
          
          // Compile the example
          const compileResult = compile_php(example.code)
          if (!compileResult.success) {
            results.push('❌ COMPILATION FAILED')
            results.push(`Error: ${compileResult.error}`)
            continue
          }
          
          // Execute the compiled WASM
          const wasmBytes = new Uint8Array(compileResult.wasm_bytes)
          const result = await runtime.execute(wasmBytes)
          
          if (result.success) {
            results.push('✅ SUCCESS')
            results.push(`⚡ Execution time: ${formatTime(result.executionTime)}`)
            results.push(`📦 WASM size: ${formatBytes(wasmBytes.length)}`)
            results.push('')
            results.push(result.output || '(no output)')
          } else {
            results.push('❌ EXECUTION FAILED')
            results.push(`Error: ${result.error}`)
            if (result.output) {
              results.push(`Output: ${result.output}`)
            }
          }
        } catch (err) {
          results.push('❌ FAILED')
          results.push(`Error: ${err.message}`)
        }
      }
      
      setOutput(`=== EdgePHP Example Tests ===\n${results.join('\n')}\n\n=== Testing Complete ===`)
    } catch (err) {
      setError(`Test error: ${err.message}`)
    }
  }

  return (
    <div className="app">
      <header className="header">
        <h1>Edge PHP Playground</h1>
        <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
          <select 
            onChange={(e) => {
              if (e.target.value) {
                setCode(EXAMPLES[e.target.value].code)
                setCurrentExample(e.target.value)
              }
            }}
            style={{ padding: '0.25rem 0.5rem' }}
          >
            <option value="">Examples...</option>
            {Object.entries(EXAMPLES).map(([key, example]) => (
              <option key={key} value={key}>{example.name}</option>
            ))}
          </select>
          <button 
            className="test-button" 
            onClick={() => testExamples()}
            style={{ 
              backgroundColor: '#17a2b8',
              color: 'white',
              border: 'none',
              padding: '0.25rem 0.5rem',
              borderRadius: '4px',
              cursor: 'pointer'
            }}
          >
            Test Examples
          </button>
          <label style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
            <input 
              type="checkbox" 
              checked={showAst} 
              onChange={(e) => setShowAst(e.target.checked)}
            />
            Show AST
          </label>
{/* Benchmarks button hidden
          <button
            onClick={() => setShowBenchmarks(true)}
            style={{
              background: '#3b82f6',
              color: 'white',
              border: 'none',
              padding: '0.25rem 0.5rem',
              borderRadius: '4px',
              cursor: 'pointer'
            }}
          >
            📊 Benchmarks
          </button>
*/}
          <button 
            className="run-button" 
            onClick={compileCode}
            disabled={isRunning || !wasmReady}
            style={{ marginRight: '0.5rem' }}
          >
            {!wasmReady ? 'Loading...' : isRunning ? 'Compiling...' : 'Compile'}
          </button>
          <button 
            className="run-button" 
            onClick={runCode}
            disabled={isExecuting || !compiledWasm}
            style={{ 
              backgroundColor: compiledWasm ? '#28a745' : '#6c757d',
              opacity: !compiledWasm ? 0.6 : 1 
            }}
          >
            {isExecuting ? 'Running...' : 'Run'}
          </button>
        </div>
      </header>
      
      <main className="main">
        <div className="editor-panel">
          <div className="panel-header">PHP Code</div>
          <div className="editor-container">
            <Editor
              height="100%"
              language="php"
              theme="vs-dark"
              value={code}
              onChange={setCode}
              options={{
                minimap: { enabled: false },
                fontSize: 14,
                tabSize: 4,
              }}
            />
          </div>
        </div>
        
        <div className="output-panel">
          <div className="panel-header">Output</div>
          <div className="output-container">
            {error && <div className="error">Error: {error}</div>}
            {!error && output}
          </div>
{/* Performance comparison hidden - time shows in output
          {lastExecutionTime !== null && (
            <PerformanceBaseline
              executionTime={lastExecutionTime}
              currentExample={currentExample}
            />
          )}
*/}
        </div>
      </main>
      
{/* BenchmarkModal hidden
      <BenchmarkModal
        isOpen={showBenchmarks}
        onClose={() => setShowBenchmarks(false)}
      />
*/}
    </div>
  )
}

export default App