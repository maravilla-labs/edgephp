const fs = require('fs');
const path = require('path');

// Get the WASM file path from command line arguments
const wasmFile = process.argv[2] || 'php-tags.wasm';

// Read the WASM file
const wasmBuffer = fs.readFileSync(wasmFile);

// Create the WebAssembly instance
(async () => {
    let output = '';
    
    const importObject = {
        env: {
            print: (dataPtr, length) => {
                // The compiler passes (data_ptr, length) directly
                const memory = instance.exports.memory;
                const mem = new Uint8Array(memory.buffer);
                
                // Read string data directly from dataPtr
                let str = '';
                for (let i = 0; i < length; i++) {
                    str += String.fromCharCode(mem[dataPtr + i]);
                }
                
                output += str;
                process.stdout.write(str);
            }
        }
    };
    
    const module = await WebAssembly.compile(wasmBuffer);
    const instance = await WebAssembly.instantiate(module, importObject);
    
    // Run the _start function
    instance.exports._start();
    
    console.log('\n=== Final output ===');
    console.log(output);
    
    // Compare with expected PHP output
    const expectedOutput = `if you want to serve PHP code in XHTML or XML documents,
                use these tags
  You can use the short echo tag to print this string.
    It's equivalent to print this string.

  this code is within short tags, but will only work if short_open_tag is enabled`;
    
    console.log('\n=== Comparison ===');
    console.log('Output matches expected:', output === expectedOutput);
})().catch(err => {
    console.error('Error:', err);
    process.exit(1);
});