const fs = require('fs');
const wabt = require('wabt');

async function debugWasm() {
    const wasmBuffer = fs.readFileSync('test_empty.wasm');
    
    try {
        const wabtModule = wabt();
        const module = wabtModule.readWasm(wasmBuffer, { readDebugNames: true });
        const wat = module.toText({ foldExprs: false, inlineExport: false });
        
        console.log(wat);
    } catch (e) {
        // If wabt not available, try basic decode
        console.log('wabt not available, using basic analysis');
        
        const module = new WebAssembly.Module(wasmBuffer);
        console.log('Imports:', WebAssembly.Module.imports(module));
        console.log('Exports:', WebAssembly.Module.exports(module));
    }
}

debugWasm().catch(console.error);