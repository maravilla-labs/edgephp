const fs = require('fs');
const { WASI } = require('wasi');

const wasmFile = process.argv[2];
if (!wasmFile) {
    console.error('Usage: node test_runner_detailed.js <wasm-file>');
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
                // Read PhpValue structure
                const view = new Uint8Array(memory.buffer);
                const dataView = new DataView(memory.buffer);
                
                console.log(`\nPrint called with ptr: ${ptr}`);
                
                // Check if it's a PhpValue pointer (should be 16-byte aligned typically)
                console.log(`Memory at ${ptr}:`);
                console.log(`  Type: ${view[ptr]} (0=null, 1=bool, 2=int, 3=float, 4=string, 5=array)`);
                console.log(`  RefCount: ${view[ptr + 1]}`);
                console.log(`  Bytes 2-3: ${view[ptr + 2]}, ${view[ptr + 3]}`);
                
                // Read the value based on type
                const type = view[ptr];
                if (type === 4) { // String
                    const strPtr = dataView.getUint32(ptr + 4, true);
                    console.log(`  String pointer: ${strPtr}`);
                    
                    if (strPtr > 0 && strPtr < memory.buffer.byteLength) {
                        // String structure: length (4 bytes), hash (4 bytes), data
                        const strLen = dataView.getUint32(strPtr, true);
                        console.log(`  String length: ${strLen}`);
                        
                        if (strLen > 0 && strLen < 1000) {
                            let str = '';
                            for (let i = 0; i < strLen; i++) {
                                str += String.fromCharCode(view[strPtr + 8 + i]);
                            }
                            console.log(`  String value: "${str}"`);
                        }
                    }
                } else if (type === 2) { // Int
                    const intValue = dataView.getBigInt64(ptr + 4, true);
                    console.log(`  Int value: ${intValue}`);
                } else if (type === 5) { // Array
                    const arrayPtr = dataView.getUint32(ptr + 4, true);
                    console.log(`  Array pointer: ${arrayPtr}`);
                }
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