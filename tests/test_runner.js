const fs = require('fs');

// Take WASM file from command line
const wasmFile = process.argv[2] || 'test_array_basic.wasm';

// Read the WASM file
const wasmBytes = fs.readFileSync(wasmFile);

// Host imports - provide print function
const imports = {
    env: {
        print: (ptr) => {
            const memory = instance.exports.memory;
            const view = new Uint8Array(memory.buffer);
            
            // Read null-terminated string from memory
            let str = '';
            let i = ptr;
            while (view[i] !== 0 && i < memory.buffer.byteLength) {
                str += String.fromCharCode(view[i]);
                i++;
            }
            process.stdout.write(str);
        }
    }
};

let instance;

// Instantiate and run the WASM module
WebAssembly.instantiate(wasmBytes, imports).then(result => {
    instance = result.instance;
    
    console.log('Running WASM module...');
    
    try {
        // Call the _start function
        instance.exports._start();
        console.log('\nWASM execution completed successfully!');
    } catch (error) {
        console.error('Error running WASM:', error);
    }
}).catch(err => {
    console.error('Error loading WASM:', err);
});
