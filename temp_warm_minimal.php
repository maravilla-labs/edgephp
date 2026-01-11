<?php
    $iterations = 10000;
    $code = 'echo "test";';
    
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
  ?>