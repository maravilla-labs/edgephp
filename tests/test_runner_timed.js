const fs = require('fs');

// Take WASM file from command line
const wasmFile = process.argv[2] || 'test.wasm';

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

    console.log('='.repeat(60));
    console.log('EdgePHP Performance Benchmark');
    console.log('='.repeat(60));
    console.log('');

    try {
        // Warm-up run (ignore timing)
        instance.exports._start();

        console.log('\n' + '='.repeat(60));
        console.log('Performance Metrics:');
        console.log('='.repeat(60));

        // Timed run
        const startTime = performance.now();
        instance.exports._start();
        const endTime = performance.now();

        const executionTime = endTime - startTime;

        console.log('\n');
        console.log(`Execution Time: ${executionTime.toFixed(3)}ms`);
        console.log(`WASM Module Size: ${(wasmBytes.length / 1024).toFixed(2)}KB`);

        // Calculate throughput
        if (executionTime > 0) {
            console.log(`Throughput: ${(1000 / executionTime).toFixed(2)} runs/second`);
        }

        console.log('\n' + '='.repeat(60));
        console.log('Benchmark completed successfully!');
        console.log('='.repeat(60));

    } catch (error) {
        console.error('\n❌ Error running WASM:', error);
        console.error('Stack:', error.stack);
    }
}).catch(err => {
    console.error('❌ Error loading WASM:', err);
    console.error('Stack:', err.stack);
});
