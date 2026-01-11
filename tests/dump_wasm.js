const fs = require('fs');

const wasmFile = process.argv[2];
if (!wasmFile) {
    console.error('Usage: node dump_wasm.js <wasm-file>');
    process.exit(1);
}

const wasmBytes = fs.readFileSync(wasmFile);

try {
    const module = new WebAssembly.Module(wasmBytes);
    
    console.log('=== WASM Module Info ===');
    console.log(`Size: ${wasmBytes.length} bytes`);
    
    console.log('\n=== Imports ===');
    const imports = WebAssembly.Module.imports(module);
    imports.forEach((imp, i) => {
        console.log(`${i}: ${imp.module}.${imp.name} (${imp.kind})`);
    });
    
    console.log('\n=== Exports ===');
    const exports = WebAssembly.Module.exports(module);
    exports.forEach((exp, i) => {
        console.log(`${i}: ${exp.name} (${exp.kind})`);
    });
    
    // Try to instantiate with minimal imports
    console.log('\n=== Attempting Instantiation ===');
    const importObject = {
        env: {
            print: (ptr, len) => console.log(`[print(${ptr}, ${len})]`)
        }
    };
    
    try {
        const instance = new WebAssembly.Instance(module, importObject);
        console.log('✓ Module instantiated successfully');
        
        if (instance.exports._start) {
            console.log('\nRunning _start...');
            instance.exports._start();
            console.log('✓ _start completed');
        }
    } catch (e) {
        console.log('✗ Instantiation failed:', e.message);
    }
} catch (e) {
    console.log('✗ Module validation failed:', e.message);
}