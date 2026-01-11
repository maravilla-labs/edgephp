const fs = require('fs');
const wasmFile = 'test_modular.wasm';
const wasmBytes = fs.readFileSync(wasmFile);

const imports = {
    env: {
        print: (ptr, len) => {
            const memory = instance.exports.memory;
            const view = new Uint8Array(memory.buffer);
            
            let str = '';
            for (let i = 0; i < len; i++) {
                str += String.fromCharCode(view[ptr + i]);
            }
            process.stdout.write(str);
        }
    }
};

let instance;

WebAssembly.instantiate(wasmBytes, imports).then(result => {
    instance = result.instance;
    console.log('WASM instantiated successfully');
    instance.exports._start();
    console.log('\nWASM execution completed');
}).catch(err => {
    console.error('Error:', err);
});