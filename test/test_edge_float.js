const fs = require('fs');

function runWasm(wasmPath) {
    const wasmBuffer = fs.readFileSync(wasmPath);
    
    WebAssembly.instantiate(wasmBuffer, {
        env: {
            print: (ptr, len) => {
                const memory = instance.exports.memory;
                const view = new DataView(memory.buffer);
                
                // Echo passes raw string data pointer and length
                const bytes = [];
                for (let i = 0; i < len; i++) {
                    bytes.push(view.getUint8(ptr + i));
                }
                
                const str = new TextDecoder().decode(new Uint8Array(bytes));
                process.stdout.write(str);
            }
        }
    }).then(result => {
        instance = result.instance;
        instance.exports._start();
    }).catch(error => {
        console.error('Error:', error);
    });
}

if (process.argv.length < 3) {
    console.log('Usage: node test_edge_float.js <wasm-file>');
    process.exit(1);
}

runWasm(process.argv[2]);