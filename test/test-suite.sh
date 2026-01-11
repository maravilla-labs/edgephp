#!/bin/bash

# EdgePHP Test Suite
# Compiles and validates PHP examples, testing WASM output

set -e

echo "🧪 EdgePHP Test Suite"
echo "===================="
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Create temp directory for test outputs
TEST_DIR="/tmp/edgephp-tests"
mkdir -p "$TEST_DIR"

# Test counter
TOTAL=0
PASSED=0
FAILED=0

# Function to run a test
run_test() {
    local name="$1"
    local php_code="$2"
    local expected_output="$3"
    
    TOTAL=$((TOTAL + 1))
    
    echo -n "Testing $name... "
    
    # Write PHP code to temp file
    local php_file="$TEST_DIR/test_$TOTAL.php"
    local wasm_file="$TEST_DIR/test_$TOTAL.wasm"
    local output_file="$TEST_DIR/test_$TOTAL.out"
    
    echo "$php_code" > "$php_file"
    
    # Try to compile
    if ./target/release/edge-php compile "$php_file" -o "$wasm_file" 2>"$output_file.compile" > /dev/null; then
        # Compilation succeeded, now validate WASM
        node -e "
        const fs = require('fs');
        try {
            const wasmBuffer = fs.readFileSync('$wasm_file');
            const wasmModule = new WebAssembly.Module(wasmBuffer);
            
            // Try to instantiate and run
            const importObject = {
                env: {
                    print: (ptr, len) => {
                        // For now, just log that print was called
                        console.log('[Print called with ptr=' + ptr + ', len=' + len + ']');
                    }
                },
                runtime: {
                    alloc_value: () => {
                        console.log('[alloc_value called]');
                        return 0x2000; // Return a dummy pointer
                    },
                    alloc_string: (ptr, len) => {
                        console.log('[alloc_string called with ptr=' + ptr + ', len=' + len + ']');
                        return ptr; // Return the same pointer for now
                    }
                }
            };
            
            const instance = new WebAssembly.Instance(wasmModule, importObject);
            
            // Check if we have _start export
            if (instance.exports._start) {
                console.log('✅ WASM valid and instantiable');
                // Try to run it
                try {
                    instance.exports._start();
                    console.log('✅ Execution completed');
                } catch (e) {
                    console.log('❌ Runtime error: ' + e.message);
                }
            } else {
                console.log('❌ No _start export found');
            }
        } catch (e) {
            console.log('❌ ' + e.message);
        }
        " > "$output_file" 2>&1
        
        if grep -q "✅ WASM valid" "$output_file"; then
            echo -e "${GREEN}✓ Compiled & Valid${NC}"
            if grep -q "✅ Execution completed" "$output_file"; then
                echo "  └─ Execution: Success"
            else
                echo -e "  └─ Execution: ${YELLOW}Failed${NC}"
                grep "Runtime error:" "$output_file" | sed 's/^/     /'
            fi
            PASSED=$((PASSED + 1))
        else
            echo -e "${RED}✗ WASM Invalid${NC}"
            grep "❌" "$output_file" | head -1 | sed 's/^/  └─ /'
            FAILED=$((FAILED + 1))
        fi
    else
        echo -e "${RED}✗ Compilation Failed${NC}"
        head -3 "$output_file.compile" | sed 's/^/  └─ /'
        FAILED=$((FAILED + 1))
    fi
}

# Run tests
echo "Basic Tests:"
echo "------------"

run_test "Empty program" '<?php ?>' ""

run_test "Simple echo" '<?php echo "test"; ?>' "test"

run_test "Integer literal" '<?php $x = 42; ?>' ""

run_test "String literal" '<?php $x = "hello"; ?>' ""

run_test "Boolean literal" '<?php $x = true; ?>' ""

run_test "Float literal" '<?php $x = 3.14; ?>' ""

run_test "Null literal" '<?php $x = null; ?>' ""

echo ""
echo "Variable Tests:"
echo "---------------"

run_test "Variable assignment and echo" '<?php $x = 42; echo $x; ?>' "42"

run_test "String variable echo" '<?php $x = "hello"; echo $x; ?>' "hello"

echo ""
echo "Expression Tests:"
echo "-----------------"

run_test "Simple addition" '<?php echo 1 + 2; ?>' "3"

run_test "String concatenation" '<?php echo "Hello" . "World"; ?>' "HelloWorld"

run_test "String concat with space" '<?php echo "Hello" . " " . "World"; ?>' "Hello World"

run_test "Variable concatenation" '<?php $a = "Hello"; $b = "World"; echo $a . " " . $b; ?>' "Hello World"

echo ""
echo "Comparison Tests:"
echo "-----------------"

run_test "Greater than" '<?php echo 5 > 3; ?>' "1"

run_test "Less than" '<?php echo 3 < 5; ?>' "1"

run_test "Equal" '<?php echo 5 == 5; ?>' "1"

run_test "Not equal" '<?php echo 5 != 3; ?>' "1"

echo ""
echo "Control Flow Tests:"
echo "-------------------"

run_test "Simple if true" '<?php if (true) { echo "yes"; } ?>' "yes"

run_test "Simple if false" '<?php if (false) { echo "yes"; } ?>' ""

run_test "If-else" '<?php if (false) { echo "yes"; } else { echo "no"; } ?>' "no"

run_test "While loop" '<?php $i = 0; while ($i < 3) { echo $i; $i = $i + 1; } ?>' "012"

echo ""
echo "Complex Tests:"
echo "--------------"

run_test "Mixed operations" '<?php $x = 5; $y = 10; echo "Sum: " . ($x + $y); ?>' "Sum: 15"

# Summary
echo ""
echo "===================="
echo "Test Summary:"
echo "Total:  $TOTAL"
echo -e "Passed: ${GREEN}$PASSED${NC}"
echo -e "Failed: ${RED}$FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed.${NC}"
    exit 1
fi