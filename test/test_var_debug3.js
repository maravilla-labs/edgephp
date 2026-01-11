const fs = require('fs');

// Get the WASM file path from command line arguments
const wasmFile = process.argv[2];
if (!wasmFile) {
    console.error('Usage: node test_var_debug3.js <wasm-file>');
    process.exit(1);
}

// Read the WASM file
const wasmBuffer = fs.readFileSync(wasmFile);

// Create the WebAssembly instance
(async () => {
    let output = '';
    let callCount = 0;
    
    const importObject = {
        env: {
            print: (ptr, len) => {
                callCount++;
                console.log(`\nprint() call #${callCount}:`);
                console.log(`  ptr: 0x${ptr.toString(16)} (${ptr})`);
                console.log(`  len: ${len}`);
                
                // Read string from memory
                const memory = instance.exports.memory;
                const mem = new Uint8Array(memory.buffer);
                
                // Read the string data
                let str = '';
                for (let i = 0; i < len; i++) {
                    const ch = mem[ptr + i];
                    str += String.fromCharCode(ch);
                }
                
                output += str;
                console.log(`  content: "${str}"`);
                
                // Show memory around the pointer
                console.log('  Memory dump:');
                for (let i = -8; i < len + 8; i += 8) {
                    const addr = ptr + i;
                    if (addr >= 0 && addr < memory.buffer.byteLength - 8) {
                        const bytes = Array.from(mem.slice(addr, addr + 8))
                            .map(b => b.toString(16).padStart(2, '0'))
                            .join(' ');
                        console.log(`    [0x${addr.toString(16).padStart(6, '0')}]: ${bytes}`);
                    }
                }
            }
        }
    };
    
    const module = await WebAssembly.compile(wasmBuffer);
    const instance = await WebAssembly.instantiate(module, importObject);
    
    // Show initial memory state
    const memory = instance.exports.memory;
    const view = new DataView(memory.buffer);
    console.log('\nInitial state:');
    console.log('  Heap pointer at 0x0:', '0x' + view.getUint32(0, true).toString(16));
    
    // Run the _start function
    console.log('\nCalling _start()...');
    try {
        instance.exports._start();
    } catch (e) {
        console.error('\nError during execution:', e);
    }
    
    console.log('\n=== Final output: ===');
    console.log(output);
    
    // Show final state
    console.log('\nFinal state:');
    console.log('  Heap pointer at 0x0:', '0x' + view.getUint32(0, true).toString(16));
    
    // Show variable at 0x200000
    console.log('\n  Variable at 0x200000:', '0x' + view.getUint32(0x200000, true).toString(16));
    
    // Show memory allocations
    console.log('\nAllocated memory:');
    const mem = new Uint8Array(memory.buffer);
    for (let i = 0x100000; i < Math.min(0x100100, view.getUint32(0, true)); i += 16) {
        const type = mem[i];
        if (type !== 0) {
            console.log(`  [0x${i.toString(16)}]: Type=${type}`);
            const bytes = Array.from(mem.slice(i, i + 16))
                .map(b => b.toString(16).padStart(2, '0'))
                .join(' ');
            console.log(`    ${bytes}`);
        }
    }
})().catch(err => {
    console.error('Error:', err);
});