const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

// Test examples
const examples = {
  minimal: 'echo "test";',
  assignment: '$x = 42; echo $x;',
  arithmetic: `
    $x = 15;
    $y = 3;
    echo "Numbers: x = " . $x . ", y = " . $y . "\\n";
    echo "Addition: " . ($x + $y) . "\\n";
    echo "Subtraction: " . ($x - $y) . "\\n";
    echo "Multiplication: " . ($x * $y) . "\\n";
    echo "Division: " . ($x / $y) . "\\n";
  `,
  strings: `
    $first = "Hello";
    $second = "World";
    $name = "EdgePHP";
    echo $first . " " . $second . "!\\n";
    echo "Welcome to " . $name . "\\n";
    echo "The answer is: " . 42 . "\\n";
  `,
  hello: `
    $message = "EdgePHP is working!";
    $number = 42;
    echo $message . "\\n";
    echo "The answer is: " . $number;
  `
};

// Measure cold start time (includes interpreter startup)
function measureColdStart(command, iterations = 100) {
  const times = [];
  
  for (let i = 0; i < iterations; i++) {
    const start = process.hrtime.bigint();
    try {
      execSync(command, { stdio: 'pipe' });
    } catch (e) {}
    const end = process.hrtime.bigint();
    times.push(Number(end - start) / 1000); // Convert to microseconds
  }
  
  // Remove outliers
  times.sort((a, b) => a - b);
  const trimmed = times.slice(Math.floor(times.length * 0.1), Math.floor(times.length * 0.9));
  
  return {
    avg: trimmed.reduce((a, b) => a + b) / trimmed.length,
    min: trimmed[0],
    max: trimmed[trimmed.length - 1]
  };
}

// Measure warm execution (within same process)
function measureWarmExecution(code, iterations = 10000) {
  // For Node.js
  const func = new Function(code);
  
  // Warmup
  for (let i = 0; i < 100; i++) {
    func();
  }
  
  // Measure
  const start = process.hrtime.bigint();
  for (let i = 0; i < iterations; i++) {
    func();
  }
  const end = process.hrtime.bigint();
  
  const totalTime = Number(end - start) / 1000; // microseconds
  return totalTime / iterations;
}

// Convert PHP code to JavaScript for Node.js testing
function phpToJs(phpCode) {
  // Simple conversion for basic PHP to JS
  let js = phpCode
    .replace(/\$(\w+)/g, 'var_$1')
    .replace(/echo\s+"([^"]+)"/g, '/* output */ "$1"')
    .replace(/echo\s+'([^']+)'/g, "/* output */ '$1'")
    .replace(/echo\s+([^;]+);/g, '/* output */ $1;')
    .replace(/echo\s+([^;]+)/g, '/* output */ $1')
    .replace(/\.\s*([^;]+)/g, '+ $1')
    .replace(/;$/gm, ';');
  
  // For warm execution, we just want the computation without I/O
  return js;
}

console.log('=== Comprehensive Benchmark: PHP vs Node.js vs EdgePHP ===\n');

const results = {
  php: { coldStart: {}, warmExecution: {} },
  nodejs: { coldStart: {}, warmExecution: {} },
  timestamp: new Date().toISOString()
};

// Benchmark each example
for (const [name, phpCode] of Object.entries(examples)) {
  console.log(`\nBenchmarking: ${name}`);
  
  // Create temporary PHP file
  const tempPhpFile = path.join(__dirname, `temp_${name}.php`);
  fs.writeFileSync(tempPhpFile, `<?php ${phpCode} ?>`);
  
  // PHP Cold Start
  console.log('  PHP cold start...');
  results.php.coldStart[name] = measureColdStart(`php ${tempPhpFile}`, 50);
  
  // Node.js Cold Start
  const jsCode = phpToJs(phpCode);
  const tempJsFile = path.join(__dirname, `temp_${name}.js`);
  fs.writeFileSync(tempJsFile, jsCode);
  
  console.log('  Node.js cold start...');
  results.nodejs.coldStart[name] = measureColdStart(`node ${tempJsFile}`, 50);
  
  // PHP Warm Execution (using eval within running PHP process)
  console.log('  PHP warm execution...');
  const phpWarmScript = `<?php
    $iterations = 10000;
    $code = '${phpCode.replace(/'/g, "\\'")}';
    
    // Warmup
    for ($i = 0; $i < 100; $i++) {
      ob_start();
      eval($code);
      ob_clean();
    }
    ob_end_clean();
    
    // Measure
    ob_start();
    $start = microtime(true);
    for ($i = 0; $i < $iterations; $i++) {
      ob_clean();
      eval($code);
    }
    $end = microtime(true);
    ob_end_clean();
    
    $avgTime = (($end - $start) * 1000000) / $iterations;
    echo json_encode(['avgTime' => $avgTime]);
  ?>`;
  
  const phpWarmFile = path.join(__dirname, `temp_warm_${name}.php`);
  fs.writeFileSync(phpWarmFile, phpWarmScript);
  
  try {
    const phpResult = JSON.parse(execSync(`php ${phpWarmFile}`, { encoding: 'utf8' }));
    results.php.warmExecution[name] = phpResult.avgTime;
  } catch (e) {
    results.php.warmExecution[name] = null;
  }
  
  // Node.js Warm Execution
  console.log('  Node.js warm execution...');
  // Redirect stdout for the measurement
  const originalWrite = process.stdout.write;
  process.stdout.write = () => {};
  
  results.nodejs.warmExecution[name] = measureWarmExecution(jsCode);
  
  process.stdout.write = originalWrite;
  
  // Cleanup
  fs.unlinkSync(tempPhpFile);
  fs.unlinkSync(tempJsFile);
  fs.unlinkSync(phpWarmFile);
}

// Calculate averages
function calculateAverage(obj) {
  const values = Object.values(obj).filter(v => v !== null);
  if (obj[Object.keys(obj)[0]].avg !== undefined) {
    // Cold start data
    return {
      avg: values.reduce((a, b) => a + b.avg, 0) / values.length,
      min: Math.min(...values.map(v => v.min)),
      max: Math.max(...values.map(v => v.max))
    };
  } else {
    // Warm execution data
    return values.reduce((a, b) => a + b, 0) / values.length;
  }
}

results.summary = {
  php: {
    coldStart: calculateAverage(results.php.coldStart),
    warmExecution: calculateAverage(results.php.warmExecution)
  },
  nodejs: {
    coldStart: calculateAverage(results.nodejs.coldStart),
    warmExecution: calculateAverage(results.nodejs.warmExecution)
  }
};

// Display results
console.log('\n=== Results Summary ===\n');

console.log('Cold Start Times (including interpreter startup):');
console.log('------------------------------------------------');
for (const [name, data] of Object.entries(results.php.coldStart)) {
  console.log(`${name}:`);
  console.log(`  PHP:     ${(data.avg / 1000).toFixed(2)}ms (${data.avg.toFixed(0)}μs)`);
  console.log(`  Node.js: ${(results.nodejs.coldStart[name].avg / 1000).toFixed(2)}ms (${results.nodejs.coldStart[name].avg.toFixed(0)}μs)`);
}

console.log('\nWarm Execution Times (within running process):');
console.log('----------------------------------------------');
for (const [name, time] of Object.entries(results.php.warmExecution)) {
  console.log(`${name}:`);
  console.log(`  PHP:     ${time ? time.toFixed(1) : 'N/A'}μs`);
  console.log(`  Node.js: ${results.nodejs.warmExecution[name].toFixed(1)}μs`);
}

console.log('\n=== Overall Averages ===');
console.log(`PHP Cold Start:      ${(results.summary.php.coldStart.avg / 1000).toFixed(2)}ms`);
console.log(`Node.js Cold Start:  ${(results.summary.nodejs.coldStart.avg / 1000).toFixed(2)}ms`);
console.log(`PHP Warm Execution:  ${results.summary.php.warmExecution.toFixed(1)}μs`);
console.log(`Node.js Warm Execution: ${results.summary.nodejs.warmExecution.toFixed(1)}μs`);

// Save results
fs.writeFileSync('benchmark_results.json', JSON.stringify(results, null, 2));
console.log('\nResults saved to benchmark_results.json');