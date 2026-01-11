const fs = require('fs');
const { WASI } = require('wasi');

const wasmFile = process.argv[2];
if (!wasmFile) {
    console.error('Usage: node test_runner_debug.js <wasm-file>');
    process.exit(1);
}

const wasi = new WASI({
    version: 'preview1',
    env: {},
    preopens: {}
});

(async () => {
    const wasm = await WebAssembly.compile(fs.readFileSync(wasmFile));
    const memory = new WebAssembly.Memory({ initial: 1 });
    
    const importObject = {
        env: {
            print: (ptr) => {
                // Read string from memory
                const view = new Uint8Array(memory.buffer);
                let str = '';
                let i = 0;
                // Read until we hit a null terminator or max 1000 chars
                while (i < 1000 && view[ptr + i] !== 0) {
                    str += String.fromCharCode(view[ptr + i]);
                    i++;
                }
                console.log("Output:", str || `(empty string at ${ptr})`);
            },
            memory: memory
        },
        wasi_snapshot_preview1: wasi.wasiImport
    };
    
    const instance = await WebAssembly.instantiate(wasm, importObject);
    
    // Call _start
    try {
        instance.exports._start();
    } catch (e) {
        console.error("Error:", e.message);
    }
})();