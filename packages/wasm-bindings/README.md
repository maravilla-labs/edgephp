# edge-php-wasm

WebAssembly bindings for EdgePHP, enabling PHP compilation in the browser.

## Features

- Compile PHP to WASM directly in the browser
- Parse PHP and return AST as JSON
- Zero server-side dependencies

## Usage (JavaScript)

```javascript
import init, { compile_php, parse_php } from 'edge-php-wasm';

// Initialize the WASM module
await init();

// Compile PHP to WASM
const result = compile_php(`<?php
$x = 10;
echo $x * 2;
`);

if (result.success) {
  // result.wasm_bytes contains the compiled WASM
  const wasmModule = await WebAssembly.compile(result.wasm_bytes);
  // ... instantiate and run
} else {
  console.error(result.error);
}

// Parse PHP to AST
const parseResult = parse_php(`<?php echo "Hello";`);
if (parseResult.success) {
  console.log(parseResult.ast); // JSON AST
}
```

## Building

```bash
# From workspace root
./build-wasm.sh

# Or manually with wasm-pack
cd packages/wasm-bindings
wasm-pack build --target web --out-dir ../../playground/src/wasm
```

## API

### `compile_php(source: string) -> CompileResult`

Compiles PHP source code to WASM bytes.

**Returns:**
- `success: boolean`
- `wasm_bytes: Uint8Array` (if successful)
- `error: string` (if failed)
- `ast: string` (JSON AST)

### `parse_php(source: string) -> ParseResult`

Parses PHP source code and returns the AST.

**Returns:**
- `success: boolean`
- `ast: string` (JSON AST if successful)
- `error: string` (if failed)

## License

Apache-2.0
