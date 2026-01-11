// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

// Edge PHP Runtime - Executes compiled PHP WASM modules

export class EdgePHPRuntime {
  constructor(debug = false) {
    this.output = [];
    this.memory = null;
    this.instance = null;
    this.debug = debug;
    this.moduleCache = new Map(); // Cache compiled modules
  }

  // Import object with host functions
  createImports() {
    const runtime = this;
    return {
      env: {
        // Print function - called by echo statements
        // Expects null-terminated string at ptr
        print: (ptr) => {
          if (runtime.debug) console.log('[print called]', { ptr });
          
          try {
            // Get memory from the instance
            const memory = runtime.instance?.exports?.memory;
            if (memory) {
              const view = new Uint8Array(memory.buffer);
              
              // Find the length of the null-terminated string
              let len = 0;
              let i = ptr;
              while (view[i] !== 0) {
                len++;
                i++;
              }
              
              // Use TextDecoder for proper UTF-8 handling
              const bytes = new Uint8Array(memory.buffer, ptr, len);
              const str = new TextDecoder('utf-8').decode(bytes);
              
              runtime.output.push(str);
              if (runtime.debug) console.log('[PHP Output]:', str);
            } else {
              if (runtime.debug) console.error('Memory not available');
              runtime.output.push(`[No memory access - ptr:${ptr}]`);
            }
          } catch (err) {
            if (runtime.debug) console.error('Error reading memory:', err);
            runtime.output.push(`[Memory error: ${err.message}]`);
          }
        },
        
        // Memory management
        malloc: (size) => {
          // Simple memory allocation (would need proper implementation)
          return 0;
        },
        
        free: (ptr) => {
          // Memory deallocation (placeholder)
        }
      }
    };
  }

  async execute(wasmBytes) {
    try {
      this.output = [];
      
      const perfStart = performance.now();
      
      // Create a cache key based on the WASM bytes
      const cacheKey = this.getCacheKey(wasmBytes);
      
      let module = this.moduleCache.get(cacheKey);
      if (!module) {
        const compileStart = performance.now();
        // Only compile if not cached
        module = await WebAssembly.compile(wasmBytes);
        const compileEnd = performance.now();
        this.moduleCache.set(cacheKey, module);
        if (this.debug) console.log(`CACHE MISS: WASM compile time: ${compileEnd - compileStart}ms (cache key: ${cacheKey})`);
      } else {
        if (this.debug) console.log(`CACHE HIT: Using cached module (cache key: ${cacheKey})`);
      }
      
      const instantiateStart = performance.now();
      // Create import object
      const imports = this.createImports();
      
      // Instantiate from cached module (much faster)
      this.instance = await WebAssembly.instantiate(module, imports);
      const instantiateEnd = performance.now();
      
      if (this.debug) console.log(`WASM instantiate time: ${instantiateEnd - instantiateStart}ms`);
      
      // Get memory export if available
      if (this.instance.exports.memory) {
        this.memory = this.instance.exports.memory;
      }
      
      const executeStart = performance.now();
      // Execute the _start function
      if (this.instance.exports._start) {
        this.instance.exports._start();
      } else if (this.instance.exports.main) {
        this.instance.exports.main();
      } else {
        throw new Error('No entry point found (_start or main)');
      }
      const executeEnd = performance.now();
      
      const totalEnd = performance.now();
      
      if (this.debug) {
        console.log(`Pure execution time: ${executeEnd - executeStart}ms`);
        console.log(`Total runtime overhead: ${totalEnd - perfStart}ms`);
      }
      
      return {
        success: true,
        output: this.output.join(''),
        error: null,
        executionTime: executeEnd - executeStart
      };
    } catch (error) {
      return {
        success: false,
        output: this.output.join(''),
        error: error.message,
        executionTime: 0
      };
    }
  }
  
  getCacheKey(wasmBytes) {
    // Create a more robust hash using crypto API
    const hash = wasmBytes.reduce((hash, byte, i) => {
      // Sample every 100th byte for performance + first/last 32 bytes
      if (i < 32 || i >= wasmBytes.length - 32 || i % 100 === 0) {
        hash = ((hash << 5) - hash + byte) & 0xffffffff;
      }
      return hash;
    }, 0);
    return `${wasmBytes.length}-${hash}`;
  }
  
  getOutput() {
    return this.output.join('');
  }
}
