const fs = require('fs');
const path = require('path');

// Get the WASM file path from command line arguments
const wasmFile = process.argv[2];
if (!wasmFile) {
    console.error('Usage: node test_var_debug.js <wasm-file>');
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
                console.log('print() called with pointer:', ptr);
                
                // Read string from memory
                const memory = instance.exports.memory;
                const mem = new Uint8Array(memory.buffer);
                const view = new DataView(memory.buffer);
                
                // Debug: show memory around the pointer
                console.log('Memory at pointer:');
                for (let i = 0; i < 32; i += 4) {
                    const val = view.getUint32(ptr + i, true);
                    console.log(`  [${ptr + i}]: 0x${val.toString(16).padStart(8, '0')} (${val})`);
                }
                
                // Read length (first 4 bytes)
                const length = view.getUint32(ptr, true);
                console.log('String length:', length);
                
                // Read string data (after 8-byte header)
                let str = '';
                for (let i = 0; i < length; i++) {
                    const ch = mem[ptr + 8 + i];
                    str += String.fromCharCode(ch);
                }
                
                output += str;
                console.log('String content:', str);
            }
        }
    };
    
    const module = await WebAssembly.compile(wasmBuffer);
    const instance = await WebAssembly.instantiate(module, importObject);
    
    // Show initial memory state
    const memory = instance.exports.memory;
    const view = new DataView(memory.buffer);
    console.log('\nInitial heap pointer at 0x0:', view.getUint32(0, true));
    
    // Run the _start function
    console.log('\nCalling _start()...');
    instance.exports._start();
    
    console.log('\nFinal output:', output);
    
    // Show final heap pointer
    console.log('Final heap pointer at 0x0:', view.getUint32(0, true));
    
    // Show variable storage area (around 0x110000)
    console.log('\nVariable storage area:');
    for (let i = 0x110000; i < 0x110020; i += 4) {
        const val = view.getUint32(i, true);
        if (val !== 0) {
            console.log(`  [0x${i.toString(16)}]: 0x${val.toString(16).padStart(8, '0')} (${val})`);
        }
    }
})().catch(err => {
    console.error('Error:', err);
});