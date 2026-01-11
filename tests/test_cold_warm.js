const fs = require('fs');
const { performance } = require('perf_hooks');

const wasmFile = process.argv[2] || 'test.wasm';

async function runColdStart() {
    console.log("=== COLD START TEST ===");
    const startLoad = performance.now();
    const wasmBytes = fs.readFileSync(wasmFile);
    const loadTime = performance.now() - startLoad;

    const imports = {
        env: {
            print: (ptr) => {
                const memory = instance.exports.memory;
                const view = new Uint8Array(memory.buffer);
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

    const startCompile = performance.now();
    const result = await WebAssembly.instantiate(wasmBytes, imports);
    instance = result.instance;
    const compileTime = performance.now() - startCompile;

    const startExec = performance.now();
    instance.exports._start();
    const execTime = performance.now() - startExec;

    console.log(`\nFile Load Time: ${loadTime.toFixed(3)}ms`);
    console.log(`WASM Compile Time: ${compileTime.toFixed(3)}ms`);
    console.log(`Execution Time: ${execTime.toFixed(3)}ms`);
    console.log(`Total Cold Start: ${(loadTime + compileTime + execTime).toFixed(3)}ms`);

    return instance;
}

async function runWarmStart(wasmBytes, compiledModule) {
    console.log("\n=== WARM START TEST (5 runs) ===");

    for (let i = 1; i <= 5; i++) {
        let instance;

        const imports = {
            env: {
                print: (ptr) => {
                    const memory = instance.exports.memory;
                    const view = new Uint8Array(memory.buffer);
                    let str = '';
                    let j = ptr;
                    while (view[j] !== 0) {
                        str += String.fromCharCode(view[j]);
                        j++;
                    }
                    // Silent execution for warm runs
                }
            }
        };

        const startExec = performance.now();
        instance = await WebAssembly.instantiate(compiledModule, imports);
        instance.exports._start();
        const execTime = performance.now() - startExec;

        console.log(`Run ${i}: ${execTime.toFixed(3)}ms`);
    }
}

async function main() {
    // Cold start with fresh module
    await runColdStart();

    // Warm start with cached module
    const wasmBytes = fs.readFileSync(wasmFile);
    const compiledModule = await WebAssembly.compile(wasmBytes);
    await runWarmStart(wasmBytes, compiledModule);
}

main().catch(err => {
    console.error('Error:', err);
    process.exit(1);
});
