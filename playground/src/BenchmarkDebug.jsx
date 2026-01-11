// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

import React, { useState, useEffect } from 'react'

export function BenchmarkDebug() {
  const [data, setData] = useState(null)
  
  useEffect(() => {
    fetch('/benchmark_results.json')
      .then(res => res.json())
      .then(data => {
        setData(data)
      })
      .catch(err => console.error('Error loading benchmark:', err))
  }, [])
  
  if (!data) return <div>Loading...</div>
  
  const execRatio = data.summary.ratios.execution
  const coldRatio = data.summary.ratios.coldStart
  
  return (
    <div style={{ padding: '20px', background: '#1e293b', color: '#f1f5f9' }}>
      <h2>Benchmark Debug</h2>
      
      <h3>Execution Performance</h3>
      <p>PHP: {data.summary.php.execution.toFixed(4)}ms</p>
      <p>EdgePHP: {data.summary.edgephp.execution.toFixed(4)}ms</p>
      <p>Ratio: {execRatio.toFixed(4)} (EdgePHP / PHP)</p>
      <p>Interpretation: EdgePHP is {execRatio < 1 ? `${(1/execRatio).toFixed(1)}x FASTER` : `${execRatio.toFixed(1)}x SLOWER`}</p>
      
      <h3>Cold Start Performance</h3>
      <p>PHP: {data.summary.php.coldStart.toFixed(4)}ms</p>
      <p>EdgePHP: {data.summary.edgephp.coldStart.toFixed(4)}ms</p>
      <p>Ratio: {coldRatio.toFixed(4)} (EdgePHP / PHP)</p>
      <p>Interpretation: EdgePHP is {coldRatio < 1 ? `${(1/coldRatio).toFixed(1)}x FASTER` : `${coldRatio.toFixed(1)}x SLOWER`}</p>
    </div>
  )
}

export default BenchmarkDebug
