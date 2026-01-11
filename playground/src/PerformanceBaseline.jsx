// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

import React from 'react'

// PHP native execution baseline data (in microseconds)
// Measured without PHP startup overhead
const PHP_BASELINE = {
  arithmetic: 2.2,
  minimal: 0.7,
  assignment: 0.6,
  hello: 0.7,
  strings: 1.3,
  comparisons: 3.1,
  types: 2.4,
  variables: 1.8,
  floats: 2.6,
  booleans: 2.5,
  complex: 1.9,
  coercion: 2.3,
  typeCoercion: 2.8,
  operators: 2.1,
  phpTags: 1.2,
  // Average for unknown examples
  default: 1.1
}

const formatTime = (microseconds) => {
  if (microseconds < 1) return `${(microseconds * 1000).toFixed(0)}ns`
  if (microseconds < 1000) return `${microseconds.toFixed(1)}μs`
  return `${(microseconds / 1000).toFixed(2)}ms`
}

const getSpeedComparison = (edgeTime, phpTime) => {
  const ratio = edgeTime / phpTime
  if (ratio < 10) return { text: `${ratio.toFixed(1)}x slower`, color: '#22c55e' }
  if (ratio < 50) return { text: `${ratio.toFixed(1)}x slower`, color: '#3b82f6' }
  if (ratio < 100) return { text: `${ratio.toFixed(1)}x slower`, color: '#f59e0b' }
  if (ratio < 200) return { text: `${ratio.toFixed(1)}x slower`, color: '#ef4444' }
  return { text: `${ratio.toFixed(0)}x slower`, color: '#dc2626' }
}

export function PerformanceBaseline({ executionTime, currentExample }) {
  const baselineTime = PHP_BASELINE[currentExample] || PHP_BASELINE.default
  const comparison = getSpeedComparison(executionTime, baselineTime)
  
  return (
    <div style={{
      background: '#1e293b',
      borderRadius: '8px',
      padding: '12px 16px',
      marginTop: '12px',
      fontSize: '14px',
      fontFamily: 'Monaco, monospace',
      lineHeight: '1.6'
    }}>
      <div style={{ marginBottom: '8px', fontWeight: 'bold', color: '#94a3b8' }}>
        Performance Comparison
      </div>
      
      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '12px' }}>
        <div>
          <div style={{ color: '#64748b', fontSize: '12px' }}>EdgePHP (WASM)</div>
          <div style={{ color: '#f1f5f9', fontWeight: 'bold' }}>
            {formatTime(executionTime)}
          </div>
        </div>
        
        <div>
          <div style={{ color: '#64748b', fontSize: '12px' }}>PHP Native</div>
          <div style={{ color: '#f1f5f9', fontWeight: 'bold' }}>
            {formatTime(baselineTime)}
          </div>
        </div>
      </div>
      
      <div style={{
        marginTop: '12px',
        padding: '8px',
        background: '#0f172a',
        borderRadius: '4px',
        textAlign: 'center'
      }}>
        <span style={{ color: comparison.color, fontWeight: 'bold' }}>
          {comparison.text}
        </span>
        <span style={{ color: '#64748b', marginLeft: '8px' }}>
          than native PHP
        </span>
      </div>
      
      <div style={{
        marginTop: '12px',
        fontSize: '11px',
        color: '#475569',
        fontStyle: 'italic'
      }}>
        * Native PHP baseline excludes startup time
      </div>
    </div>
  )
}

export default PerformanceBaseline
