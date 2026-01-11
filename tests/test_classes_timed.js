const fs = require('fs');
const wasmFile = process.argv[2] || 'benchmark_classes.wasm';
const wasmBytes = fs.readFileSync(wasmFile);

const imports = {
    env: {
        print: (ptr) => {
            const memory = instance.exports.memory;
            const view = new Uint8Array(memory.buffer);

            // Read null-terminated string
            let str = '';
            let i = ptr;
            while (view[i] !== 0) {
                str += String.fromCharCode(view[i]);
                i++;
            }
            process.stdout.write(str);
        }
    }
};

let instance;

WebAssembly.instantiate(wasmBytes, imports).then(result => {
    instance = result.instance;

    // Measure execution time only
    const start = performance.now();
    instance.exports._start();
    const end = performance.now();

    console.log(`\nExecution time: ${(end - start).toFixed(2)}ms`);
}).catch(err => {
    console.error('Error:', err);
});
