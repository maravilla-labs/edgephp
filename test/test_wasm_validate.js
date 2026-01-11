const fs = require('fs');

// Test WASM that should work
const testWasm = new Uint8Array([
    0x00, 0x61, 0x73, 0x6d, // magic
    0x01, 0x00, 0x00, 0x00, // version
    
    // Type section
    0x01, // section id
    0x05, // section size
    0x01, // 1 type
    0x60, // func type
    0x00, // 0 params
    0x01, 0x7f, // 1 result i32
    
    // Function section
    0x03, // section id
    0x02, // section size
    0x01, // 1 function
    0x00, // type index 0
    
    // Memory section
    0x05, // section id
    0x03, // section size
    0x01, // 1 memory
    0x00, // no maximum
    0x01, // minimum 1 page
    
    // Export section
    0x07, // section id
    0x08, // section size
    0x01, // 1 export
    0x04, 0x74, 0x65, 0x73, 0x74, // "test"
    0x00, // function export
    0x00, // function index 0
    
    // Code section
    0x0a, // section id
    0x0e, // section size
    0x01, // 1 function
    0x0c, // function size
    0x01, // 1 local declaration
    0x02, 0x7f, // 2 locals of type i32
    0x41, 0x00, // i32.const 0
    0x28, 0x02, 0x00, // i32.load align=2 offset=0
    0x22, 0x00, // local.tee 0
    0x0b, // end
]);

try {
    const module = new WebAssembly.Module(testWasm);
    console.log('Test WASM validates!');
} catch (e) {
    console.log('Test WASM error:', e.message);
}

// Now test our generated WASM
const ourWasm = fs.readFileSync('test_minimal.wasm');
try {
    const module = new WebAssembly.Module(ourWasm);
    console.log('Our WASM validates!');
} catch (e) {
    console.log('Our WASM error:', e.message);
}