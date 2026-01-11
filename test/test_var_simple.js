const fs = require('fs');
const path = require('path');

// Get the WASM file path from command line arguments
const wasmFile = process.argv[2];
if (!wasmFile) {
    console.error('Usage: node test_var_simple.js <wasm-file>');
    process.exit(1);
}

// Read the WASM file
const wasmBuffer = fs.readFileSync(wasmFile);

// Create the WebAssembly instance
(async () => {
    let output = '';
    
    const importObject = {
        env: {
            print: (ptr) => {
                // Read string from memory
                const memory = instance.exports.memory;
                const mem = new Uint8Array(memory.buffer);
                
                // Read length (first 4 bytes)
                const length = new DataView(memory.buffer, ptr, 4).getUint32(0, true);
                
                // Read string data (after 8-byte header)
                let str = '';
                for (let i = 0; i < length; i++) {
                    str += String.fromCharCode(mem[ptr + 8 + i]);
                }
                
                output += str;
                console.log('Output:', str);
            }
        }
    };
    
    const module = await WebAssembly.compile(wasmBuffer);
    const instance = await WebAssembly.instantiate(module, importObject);
    
    // Run the _start function
    instance.exports._start();
    
    console.log('Final output:', output);
})().catch(err => {
    console.error('Error:', err);
});