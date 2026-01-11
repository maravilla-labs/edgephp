const fs = require('fs');
const wasmFile = process.argv[2] || 'test_int_to_string.wasm';

async function runWasm() {
    try {
        const wasmBuffer = fs.readFileSync(wasmFile);
        
        // Import object with print function
        const importObject = {
            env: {
                print: (ptr, len) => {
                    const bytes = new Uint8Array(memory.buffer, ptr, len);
                    process.stdout.write(new TextDecoder().decode(bytes));
                }
            }
        };
        
        const wasmModule = await WebAssembly.instantiate(wasmBuffer, importObject);
        const { memory, _start } = wasmModule.instance.exports;
        
        // Run the start function
        _start();
    } catch (error) {
        console.error('Error running WASM:', error);
    }
}

runWasm();