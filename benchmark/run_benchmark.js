const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// Measure PHP cold start + execution (interpreter startup + code execution)
function measurePHPColdStart(phpFile, iterations = 100) {
    const times = [];
    
    for (let i = 0; i < iterations; i++) {
        const start = process.hrtime.bigint();
        try {
            execSync(`php -dxdebug.mode=off ${phpFile}`, { stdio: 'pipe' });
        } catch (e) {}
        const end = process.hrtime.bigint();
        times.push(Number(end - start) / 1000000); // Convert to milliseconds
    }
    
    return processTimings(times);
}

// Measure EdgePHP WASM load + instantiate + execute (within running Node.js)
async function measureEdgePHPColdStart(wasmPath, iterations = 100) {
    const times = [];

    const importObject = {
        env: {
            print: (ptr, len) => {}
        }
    };

    for (let i = 0; i < iterations; i++) {
        const start = process.hrtime.bigint();

        // Load WASM file
        const wasmBytes = fs.readFileSync(wasmPath);

        // Compile WASM
        const module = await WebAssembly.compile(wasmBytes);

        // Instantiate and execute
        const instance = await WebAssembly.instantiate(module, importObject);
        if (instance.exports._start) {
            instance.exports._start();
        }

        const end = process.hrtime.bigint();
        times.push(Number(end - start) / 1000000); // Convert to milliseconds
    }

    return processTimings(times);
}

// Measure PHP execution only (within running interpreter)
function measurePHPExecution(phpFile, iterations = 1000) {
    const benchScript = `<?php
$code = file_get_contents('${phpFile}');
$code = str_replace('<?php', '', $code);

// Warmup
for ($i = 0; $i < 100; $i++) {
    ob_start();
    eval($code);
    ob_clean();
}
ob_end_clean();

// Measure
$times = [];
for ($i = 0; $i < ${iterations}; $i++) {
    ob_start();
    $start = microtime(true);
    eval($code);
    $end = microtime(true);
    ob_end_clean();
    $times[] = ($end - $start) * 1000; // Convert to milliseconds
}

echo json_encode($times);`;

    const tempFile = path.join(__dirname, 'temp_bench.php');
    fs.writeFileSync(tempFile, benchScript);
    
    try {
        const result = execSync(`php -dxdebug.mode=off ${tempFile}`, { encoding: 'utf8' });
        fs.unlinkSync(tempFile);
        const times = JSON.parse(result);
        return processTimings(times);
    } catch (e) {
        fs.unlinkSync(tempFile);
        throw e;
    }
}

// Measure EdgePHP execution only (just the function call)
async function measureEdgePHPExecution(wasmPath, iterations = 1000) {
    const wasmBytes = fs.readFileSync(wasmPath);
    
    const importObject = {
        env: {
            print: (ptr, len) => {}
        }
    };
    
    // Compile module once
    const module = await WebAssembly.compile(wasmBytes);
    
    // Create instance once (like PHP interpreter running)
    const instance = await WebAssembly.instantiate(module, importObject);
    
    if (!instance.exports._start) {
        throw new Error('No _start export found in WASM module');
    }
    
    const times = [];
    
    // Warmup - just call the function
    for (let i = 0; i < 100; i++) {
        instance.exports._start();
    }
    
    // Measure just the function execution
    for (let i = 0; i < iterations; i++) {
        const start = process.hrtime.bigint();
        instance.exports._start();
        const end = process.hrtime.bigint();
        times.push(Number(end - start) / 1000000); // Convert to milliseconds
    }
    
    return processTimings(times);
}

// Measure EdgePHP instantiation time (creating new instances)
async function measureEdgePHPInstantiation(wasmPath, iterations = 1000) {
    const wasmBytes = fs.readFileSync(wasmPath);
    
    const importObject = {
        env: {
            print: (ptr, len) => {}
        }
    };
    
    // Compile module once
    const module = await WebAssembly.compile(wasmBytes);
    
    const times = [];
    
    // Warmup
    for (let i = 0; i < 100; i++) {
        await WebAssembly.instantiate(module, importObject);
    }
    
    // Measure instantiation time
    for (let i = 0; i < iterations; i++) {
        const start = process.hrtime.bigint();
        await WebAssembly.instantiate(module, importObject);
        const end = process.hrtime.bigint();
        times.push(Number(end - start) / 1000000); // Convert to milliseconds
    }
    
    return processTimings(times);
}

// Measure just WASM compilation time
async function measureEdgePHPCompilation(wasmPath, iterations = 100) {
    const wasmBytes = fs.readFileSync(wasmPath);
    const times = [];
    
    // Warmup
    for (let i = 0; i < 10; i++) {
        await WebAssembly.compile(wasmBytes);
    }
    
    // Measure compilation time
    for (let i = 0; i < iterations; i++) {
        const start = process.hrtime.bigint();
        await WebAssembly.compile(wasmBytes);
        const end = process.hrtime.bigint();
        times.push(Number(end - start) / 1000000); // Convert to milliseconds
    }
    
    return processTimings(times);
}

// Process timing arrays to get statistics
function processTimings(times) {
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
        median: trimmed[Math.floor(trimmed.length / 2)]
    };
}

async function runBenchmarks() {
    console.log('=== EdgePHP Benchmark ===\n');
    console.log('Measuring:');
    console.log('  - PHP: interpreter startup + code execution');
    console.log('  - EdgePHP: WASM operations (load, compile, instantiate, execute)');
    console.log('  - Note: Node.js startup time is NOT included\n');
    
    const results = {
        timestamp: new Date().toISOString(),
        description: 'PHP cold start vs EdgePHP WASM load + execution',
        tests: {},
        summary: {}
    };
    
    // Check for compiled examples
    const compiledDir = path.join(__dirname, 'compiled');
    if (!fs.existsSync(compiledDir) || fs.readdirSync(compiledDir).length === 0) {
        console.error('Error: No compiled WASM files found.');
        console.error('Please run ./compile_all.sh first to compile the examples.');
        process.exit(1);
    }
    
    const examples = [
        'minimal',
        'assignment', 
        'arithmetic',
        'strings',
        'comprehensive'
    ];
    
    for (const example of examples) {
        const phpPath = path.join(__dirname, 'examples', `${example}.php`);
        const wasmPath = path.join(compiledDir, `${example}.wasm`);
        
        if (!fs.existsSync(phpPath) || !fs.existsSync(wasmPath)) {
            console.log(`Skipping ${example} - files not found`);
            continue;
        }
        
        console.log(`\n${example.toUpperCase()}`);
        console.log('─'.repeat(50));
        
        results.tests[example] = {
            php: {},
            edgephp: {}
        };
        
        // PHP measurements
        console.log('PHP:');
        
        // PHP execution only (warm)
        const phpExec = measurePHPExecution(phpPath, 1000);
        results.tests[example].php.execution = phpExec.avg;
        console.log(`  Execution only:    ${phpExec.avg.toFixed(3)}ms`);
        
        // PHP cold start (interpreter + execution)
        const phpCold = measurePHPColdStart(phpPath, 100);
        results.tests[example].php.coldStart = phpCold.avg;
        results.tests[example].php.interpreterOverhead = phpCold.avg - phpExec.avg;
        console.log(`  Cold start:        ${phpCold.avg.toFixed(3)}ms`);
        console.log(`  Interpreter time:  ${results.tests[example].php.interpreterOverhead.toFixed(3)}ms`);
        
        // EdgePHP measurements
        console.log('\nEdgePHP (WASM):');
        
        // EdgePHP execution only (just function call)
        const edgeExec = await measureEdgePHPExecution(wasmPath, 1000);
        results.tests[example].edgephp.execution = edgeExec.avg;
        console.log(`  Execution:         ${edgeExec.avg.toFixed(3)}ms (function call)`);
        
        // EdgePHP instantiation time
        const edgeInst = await measureEdgePHPInstantiation(wasmPath, 1000);
        results.tests[example].edgephp.instantiation = edgeInst.avg;
        console.log(`  Instantiation:     ${edgeInst.avg.toFixed(3)}ms`);
        
        // EdgePHP compilation time
        const edgeComp = await measureEdgePHPCompilation(wasmPath, 100);
        results.tests[example].edgephp.compilation = edgeComp.avg;
        console.log(`  Compilation:       ${edgeComp.avg.toFixed(3)}ms`);
        
        // EdgePHP cold start (load + compile + instantiate + execute)
        const edgeCold = await measureEdgePHPColdStart(wasmPath, 50);
        results.tests[example].edgephp.coldStart = edgeCold.avg;
        const fileSize = fs.statSync(wasmPath).size;
        results.tests[example].edgephp.wasmSize = fileSize;
        console.log(`  Full load:         ${edgeCold.avg.toFixed(3)}ms (read + compile + instantiate + execute)`);
        console.log(`  WASM size:         ${(fileSize / 1024).toFixed(1)}KB`);
        
        // Ratios
        console.log('\nRatios:');
        const execRatio = results.tests[example].edgephp.execution / results.tests[example].php.execution;
        const coldRatio = results.tests[example].edgephp.coldStart / results.tests[example].php.coldStart;
        if (execRatio < 1) {
            console.log(`  Execution: EdgePHP is ${(1/execRatio).toFixed(1)}x faster`);
        } else {
            console.log(`  Execution: EdgePHP is ${execRatio.toFixed(1)}x slower`);
        }
        console.log(`  Cold start: EdgePHP is ${coldRatio.toFixed(1)}x ${coldRatio > 1 ? 'slower' : 'faster'}`);
    }
    
    // Calculate summary
    const validTests = Object.values(results.tests);
    
    if (validTests.length > 0) {
        const phpExecAvg = validTests.reduce((sum, t) => sum + t.php.execution, 0) / validTests.length;
        const edgeExecAvg = validTests.reduce((sum, t) => sum + t.edgephp.execution, 0) / validTests.length;
        const phpColdAvg = validTests.reduce((sum, t) => sum + t.php.coldStart, 0) / validTests.length;
        const edgeColdAvg = validTests.reduce((sum, t) => sum + t.edgephp.coldStart, 0) / validTests.length;
        
        results.summary = {
            php: {
                execution: phpExecAvg,
                coldStart: phpColdAvg,
                interpreterOverhead: phpColdAvg - phpExecAvg
            },
            edgephp: {
                execution: edgeExecAvg,
                coldStart: edgeColdAvg,
                loadOverhead: edgeColdAvg - edgeExecAvg
            },
            ratios: {
                execution: edgeExecAvg / phpExecAvg,
                coldStart: edgeColdAvg / phpColdAvg
            }
        };
        
        console.log('\n' + '═'.repeat(50));
        console.log('SUMMARY');
        console.log('═'.repeat(50));
        
        console.log('\nPHP:');
        console.log(`  Execution only:     ${phpExecAvg.toFixed(3)}ms`);
        console.log(`  Cold start:         ${phpColdAvg.toFixed(3)}ms`);
        console.log(`  Interpreter time:   ${results.summary.php.interpreterOverhead.toFixed(3)}ms`);
        
        const edgeInstAvg = validTests.reduce((sum, t) => sum + (t.edgephp.instantiation || 0), 0) / validTests.length;
        const edgeCompAvg = validTests.reduce((sum, t) => sum + (t.edgephp.compilation || 0), 0) / validTests.length;
        
        console.log('\nEdgePHP (WASM):');
        console.log(`  Execution:          ${edgeExecAvg.toFixed(3)}ms`);
        console.log(`  Instantiation:      ${edgeInstAvg.toFixed(3)}ms`);
        console.log(`  Compilation:        ${edgeCompAvg.toFixed(3)}ms`);
        console.log(`  Full load:          ${edgeColdAvg.toFixed(3)}ms`);
        
        console.log('\nPerformance Ratios:');
        if (results.summary.ratios.execution < 1) {
            console.log(`  Execution: EdgePHP is ${(1/results.summary.ratios.execution).toFixed(1)}x faster`);
        } else {
            console.log(`  Execution: EdgePHP is ${results.summary.ratios.execution.toFixed(1)}x slower`);
        }
        console.log(`  Cold start: EdgePHP is ${results.summary.ratios.coldStart.toFixed(1)}x ${results.summary.ratios.coldStart > 1 ? 'slower' : 'faster'}`);
        
        console.log('\nNotes:');
        console.log('  - PHP execution: eval() within running interpreter');
        console.log('  - EdgePHP execution: just the _start() function call');
        console.log('  - EdgePHP instantiation: creating new WASM instance');
        console.log('  - EdgePHP compilation: WebAssembly.compile() from bytecode');
        console.log('  - EdgePHP full load: file read + compile + instantiate + execute');
        console.log('  - PHP cold start: interpreter startup + execution');
        console.log('  - Node.js startup time is NOT included in EdgePHP measurements');
    }
    
    // Save results
    fs.writeFileSync(
        path.join(__dirname, 'benchmark_results.json'),
        JSON.stringify(results, null, 2)
    );
    console.log('\n\nResults saved to benchmark_results.json');
}

// Run benchmarks
runBenchmarks().catch(console.error);