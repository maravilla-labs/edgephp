const fs = require('fs');

async function checkWasmExports(wasmPath) {
    const wasmBytes = fs.readFileSync(wasmPath);
    const module = await WebAssembly.compile(wasmBytes);
    const instance = await WebAssembly.instantiate(module, {
        env: { print: () => {} }
    });
    
    console.log(`Exports for ${wasmPath}:`);
    console.log(Object.keys(instance.exports));
}

checkWasmExports('./compiled/minimal.wasm').catch(console.error);