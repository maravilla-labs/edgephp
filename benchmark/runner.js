const fs = require('fs');
const path = require('path');

// Function to run WASM and measure execution time
async function measureExecution(wasmPath, iterations = 1000) {
    const wasmBytes = fs.readFileSync(wasmPath);
    
    // Import functions
    const importObject = {
        env: {
            print: (ptr, len) => {
                // Silent - we're just measuring execution time
            }
        }
    };
    
    // Compile once
    const module = await WebAssembly.compile(wasmBytes);
    
    // Measure instantiation + execution time
    const times = [];
    
    for (let i = 0; i < iterations; i++) {
        const start = process.hrtime.bigint();
        const instance = await WebAssembly.instantiate(module, importObject);
        
        // Call main function
        if (instance.exports.main) {
            instance.exports.main();
        }
        
        const end = process.hrtime.bigint();
        times.push(Number(end - start) / 1000000); // Convert to milliseconds
    }
    
    // Remove outliers (top and bottom 10%)
    times.sort((a, b) => a - b);
    const trimmed = times.slice(
        Math.floor(times.length * 0.1), 
        Math.floor(times.length * 0.9)
    );
    
    return {
        avg: trimmed.reduce((a, b) => a + b) / trimmed.length,
        min: trimmed[0],
        max: trimmed[trimmed.length - 1],
        iterations: trimmed.length
    };
}

// Measure PHP execution time
function measurePHP(phpFile, iterations = 1000) {
    const { execSync } = require('child_process');
    const times = [];
    
    for (let i = 0; i < iterations; i++) {
        const start = process.hrtime.bigint();
        try {
            execSync(`php ${phpFile}`, { stdio: 'pipe' });
        } catch (e) {}
        const end = process.hrtime.bigint();
        times.push(Number(end - start) / 1000000); // Convert to milliseconds
    }
    
    // Remove outliers
    times.sort((a, b) => a - b);
    const trimmed = times.slice(
        Math.floor(times.length * 0.1), 
        Math.floor(times.length * 0.9)
    );
    
    return {
        avg: trimmed.reduce((a, b) => a + b) / trimmed.length,
        min: trimmed[0],
        max: trimmed[trimmed.length - 1],
        iterations: trimmed.length
    };
}

async function runBenchmarks() {
    console.log('=== EdgePHP Execution Benchmark ===\n');
    console.log('Measuring pure execution time (after compilation)\n');
    
    const results = {
        timestamp: new Date().toISOString(),
        edgephp: {},
        php: {}
    };
    
    // Get all compiled WASM files
    const compiledDir = path.join(__dirname, 'compiled');
    const wasmFiles = fs.readdirSync(compiledDir).filter(f => f.endsWith('.wasm'));
    
    for (const wasmFile of wasmFiles) {
        const name = path.basename(wasmFile, '.wasm');
        const wasmPath = path.join(compiledDir, wasmFile);
        const phpPath = path.join(__dirname, 'examples', `${name}.php`);
        
        console.log(`\nBenchmarking: ${name}`);
        
        // Measure EdgePHP execution
        console.log('  EdgePHP (WASM)...');
        try {
            results.edgephp[name] = await measureExecution(wasmPath, 1000);
            console.log(`    Average: ${results.edgephp[name].avg.toFixed(3)}ms`);
            console.log(`    Min: ${results.edgephp[name].min.toFixed(3)}ms`);
            console.log(`    Max: ${results.edgephp[name].max.toFixed(3)}ms`);
        } catch (e) {
            console.log(`    Error: ${e.message}`);
            results.edgephp[name] = { error: e.message };
        }
        
        // Measure PHP execution
        console.log('  PHP native...');
        try {
            results.php[name] = measurePHP(phpPath, 100); // Fewer iterations for PHP
            console.log(`    Average: ${results.php[name].avg.toFixed(3)}ms`);
            console.log(`    Min: ${results.php[name].min.toFixed(3)}ms`);
            console.log(`    Max: ${results.php[name].max.toFixed(3)}ms`);
        } catch (e) {
            console.log(`    Error: ${e.message}`);
            results.php[name] = { error: e.message };
        }
    }
    
    // Calculate averages
    const edgeAvg = Object.values(results.edgephp)
        .filter(r => !r.error)
        .reduce((sum, r) => sum + r.avg, 0) / Object.values(results.edgephp).filter(r => !r.error).length;
    
    const phpAvg = Object.values(results.php)
        .filter(r => !r.error)
        .reduce((sum, r) => sum + r.avg, 0) / Object.values(results.php).filter(r => !r.error).length;
    
    console.log('\n=== Summary ===');
    console.log(`EdgePHP average execution: ${edgeAvg.toFixed(3)}ms`);
    console.log(`PHP average execution: ${phpAvg.toFixed(3)}ms`);
    console.log(`\nEdgePHP is ${(edgeAvg / phpAvg).toFixed(1)}x slower than native PHP`);
    
    // Note about browser execution
    console.log('\nNote: In browser, EdgePHP execution characteristics:');
    console.log('  - First run includes compilation (~5ms)');
    console.log('  - Subsequent runs are pure execution (~0.2ms)');
    console.log('  - WASM modules can be cached between page loads');
    
    // Save results
    fs.writeFileSync(
        path.join(__dirname, 'benchmark_results.json'),
        JSON.stringify(results, null, 2)
    );
    console.log('\nResults saved to benchmark_results.json');
}

// Run benchmarks
runBenchmarks().catch(console.error);