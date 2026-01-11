const fs = require('fs');
const path = require('path');

// Get the WASM file path from command line arguments
const wasmFile = process.argv[2];
if (!wasmFile) {
    console.error('Usage: node test_var_debug2.js <wasm-file>');
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
                
                // Read the value at the pointer (it's a PhpValue pointer)
                console.log('PhpValue at pointer:');
                if (ptr < memory.buffer.byteLength - 16) {
                    const type = mem[ptr];
                    console.log(`  Type: ${type}`);
                    console.log(`  Raw bytes: ${Array.from(mem.slice(ptr, ptr + 16)).map(b => b.toString(16).padStart(2, '0')).join(' ')}`);
                    
                    // If it's a string type (4), read the string pointer
                    if (type === 4) {
                        const strPtr = view.getUint32(ptr + 4, true);
                        console.log(`  String pointer: 0x${strPtr.toString(16)}`);
                        
                        if (strPtr > 0 && strPtr < memory.buffer.byteLength - 8) {
                            const length = view.getUint32(strPtr, true);
                            console.log(`  String length at ${strPtr}: ${length}`);
                            
                            if (length > 0 && length < 1000) {
                                let str = '';
                                for (let i = 0; i < length; i++) {
                                    str += String.fromCharCode(mem[strPtr + 8 + i]);
                                }
                                output += str;
                                console.log(`  String content: "${str}"`);
                            }
                        }
                    }
                }
            }
        }
    };
    
    const module = await WebAssembly.compile(wasmBuffer);
    const instance = await WebAssembly.instantiate(module, importObject);
    
    // Show initial memory state
    const memory = instance.exports.memory;
    const view = new DataView(memory.buffer);
    console.log('\nInitial heap pointer at 0x0:', '0x' + view.getUint32(0, true).toString(16));
    
    // Run the _start function
    console.log('\nCalling _start()...');
    instance.exports._start();
    
    console.log('\nFinal output:', output);
    
    // Show final heap pointer
    const finalHeap = view.getUint32(0, true);
    console.log('Final heap pointer at 0x0:', '0x' + finalHeap.toString(16));
    
    // Show memory from 0x100000 onwards (where variables might be stored)
    console.log('\nMemory around heap start (0x100000):');
    const mem = new Uint8Array(memory.buffer);
    for (let i = 0x100000; i < Math.min(0x100100, memory.buffer.byteLength); i += 16) {
        const bytes = Array.from(mem.slice(i, i + 16)).map(b => b.toString(16).padStart(2, '0')).join(' ');
        const hasNonZero = mem.slice(i, i + 16).some(b => b !== 0);
        if (hasNonZero) {
            console.log(`  [0x${i.toString(16)}]: ${bytes}`);
        }
    }
})().catch(err => {
    console.error('Error:', err);
});