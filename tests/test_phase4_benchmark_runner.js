const fs = require('fs');

const wasmFile = process.argv[2] || 'test_phase4_benchmark.wasm';
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

console.log('EdgePHP - Phase 4 Functions Benchmark');
console.log('=====================================');
console.log('');

// Compile WASM
const compileStart = performance.now();
WebAssembly.instantiate(wasmBytes, imports).then(result => {
    const compileEnd = performance.now();
    const compileTime = compileEnd - compileStart;

    instance = result.instance;

    // First execution (cold start)
    const coldStart = performance.now();
    instance.exports._start();
    const coldEnd = performance.now();
    const coldTime = coldEnd - coldStart;

    console.log('');
    console.log('Performance Metrics:');
    console.log('-------------------');
    console.log(`WASM Compile Time: ${compileTime.toFixed(3)}ms`);
    console.log(`Cold Start (first execution): ${coldTime.toFixed(3)}ms`);
    console.log('');

    // Warm runs
    console.log('Warm runs (5 iterations):');
    const warmTimes = [];
    for (let i = 0; i < 5; i++) {
        const warmStart = performance.now();
        instance.exports._start();
        const warmEnd = performance.now();
        const warmTime = warmEnd - warmStart;
        warmTimes.push(warmTime);
        console.log(`  Run ${i + 1}: ${warmTime.toFixed(3)}ms`);
    }

    const avgWarm = warmTimes.reduce((a, b) => a + b, 0) / warmTimes.length;
    const minWarm = Math.min(...warmTimes);
    const maxWarm = Math.max(...warmTimes);

    console.log('');
    console.log(`Average warm time: ${avgWarm.toFixed(3)}ms`);
    console.log(`Best warm time: ${minWarm.toFixed(3)}ms`);
    console.log(`Worst warm time: ${maxWarm.toFixed(3)}ms`);
    console.log('');
    console.log(`Throughput (avg): ${(30100 / avgWarm * 1000).toFixed(0)} ops/second`);
    console.log(`Throughput (best): ${(30100 / minWarm * 1000).toFixed(0)} ops/second`);
    console.log('');
    console.log(`Module Size: ${(wasmBytes.length / 1024).toFixed(2)}KB`);

}).catch(err => {
    console.error('Error loading WASM:', err);
});
