#!/usr/bin/env node

const fs = require('fs');
const { spawn } = require('child_process');

// Test string concatenation with our compiler
const testCode = `<?php
$a = "Hello";
$b = "World";
echo $a . " " . $b;`;

console.log('Testing string concatenation...');
console.log('PHP Code:');
console.log(testCode);
console.log('\n--- Compiling with EdgePHP Compiler ---');

// Write test file
fs.writeFileSync('/tmp/test_concat.php', testCode);

// Compile with EdgePHP
const compileProcess = spawn('./target/debug/edge-php', ['compile', '-o', '/tmp/test_concat.wasm', '/tmp/test_concat.php'], {
    cwd: '/workspaces/edgephp',
    stdio: 'pipe'
});

compileProcess.stdout.on('data', (data) => {
    console.log(`Compile stdout: ${data}`);
});

compileProcess.stderr.on('data', (data) => {
    console.log(`Compile stderr: ${data}`);
});

compileProcess.on('close', (code) => {
    console.log(`Compilation finished with code: ${code}`);
    
    if (code === 0) {
        console.log('✅ Compilation successful!');
        
        // Check if WASM file was created and show its size
        try {
            const wasmStats = fs.statSync('/tmp/test_concat.wasm');
            console.log(`WASM file size: ${wasmStats.size} bytes`);
            
            // Try to load and validate the WASM
            const wasmBuffer = fs.readFileSync('/tmp/test_concat.wasm');
            console.log('WASM file created successfully');
            
            // Try to parse it with Node.js WebAssembly
            try {
                const wasmModule = new WebAssembly.Module(wasmBuffer);
                console.log('✅ WASM module is valid!');
                
                // List the exports
                const exports = WebAssembly.Module.exports(wasmModule);
                console.log('WASM exports:');
                exports.forEach(exp => {
                    console.log(`  - ${exp.name} (${exp.kind})`);
                });
                
                // List the imports  
                const imports = WebAssembly.Module.imports(wasmModule);
                console.log('WASM imports:');
                imports.forEach(imp => {
                    console.log(`  - ${imp.module}.${imp.name} (${imp.kind})`);
                });
                
            } catch (wasmError) {
                console.log('❌ WASM validation failed:');
                console.log(wasmError.message);
            }
            
        } catch (err) {
            console.log('❌ Failed to read WASM file:', err.message);
        }
    } else {
        console.log('❌ Compilation failed!');
    }
    
    // No need to test old compiler anymore
});