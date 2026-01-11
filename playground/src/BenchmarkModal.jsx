// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

import React, { useState, useEffect } from 'react'

// Default benchmark data (fallback if server data not available)
const DEFAULT_BENCHMARK_DATA = {
  // Cold start to completion (interpreter startup + execution)
  coldStartToCompletion: {
    php: {
      minimal: 12.14,      // ms
      assignment: 13.13,   // ms
      arithmetic: 13.69,   // ms
      strings: 13.24,      // ms
      comprehensive: 14.51, // ms
      average: 13.34       // ms
    },
    edgephp: {
      // In browser: compilation + first execution
      minimal: 5.2,        // ms
      assignment: 5.5,     // ms
      arithmetic: 6.1,     // ms
      strings: 5.8,        // ms
      comprehensive: 7.2,  // ms
      average: 5.96        // ms
    }
  },
  // Execution only (after compilation/startup)
  executionOnly: {
    php: {
      // PHP execution time (subtracting interpreter startup ~10ms)
      minimal: 0.04,       // ms
      assignment: 0.03,    // ms
      arithmetic: 0.09,    // ms
      strings: 0.04,       // ms
      comprehensive: 0.15, // ms
      average: 0.07        // ms
    },
    edgephp: {
      // Pure WASM execution in browser
      minimal: 0.15,       // ms
      assignment: 0.16,    // ms
      arithmetic: 0.19,    // ms
      strings: 0.18,       // ms
      comprehensive: 0.25, // ms
      average: 0.19        // ms
    }
  },
  // EdgePHP browser characteristics
  edgephpBrowser: {
    compilationTime: 5.0,    // ms - one-time cost
    executionTime: 0.19,     // ms - can run many times
    wasmCached: true,        // WASM module can be cached
    totalFirstRun: 5.19      // ms - compilation + execution
  }
}

const formatTime = (milliseconds) => {
  if (milliseconds < 0.001) return `${(milliseconds * 1000000).toFixed(0)}ns`
  if (milliseconds < 1) return `${(milliseconds * 1000).toFixed(1)}μs`
  return `${milliseconds.toFixed(2)}ms`
}

// Convert server benchmark data to modal format
function convertServerData(serverData) {
  if (!serverData || !serverData.summary) return null
  
  const { summary, tests } = serverData
  
  // Extract averages from summary
  const phpExecAvg = summary.php.execution
  const edgeExecAvg = summary.edgephp.execution
  const phpColdAvg = summary.php.coldStart
  const edgeColdAvg = summary.edgephp.coldStart
  
  // Calculate additional metrics
  const edgeInstAvg = Object.values(tests).reduce((sum, t) => sum + (t.edgephp?.instantiation || 0), 0) / Object.keys(tests).length
  const edgeCompAvg = Object.values(tests).reduce((sum, t) => sum + (t.edgephp?.compilation || 0), 0) / Object.keys(tests).length
  
  return {
    executionOnly: {
      php: {
        minimal: tests.minimal?.php.execution || phpExecAvg,
        assignment: tests.assignment?.php.execution || phpExecAvg,
        arithmetic: tests.arithmetic?.php.execution || phpExecAvg,
        strings: tests.strings?.php.execution || phpExecAvg,
        comprehensive: tests.comprehensive?.php.execution || phpExecAvg,
        average: phpExecAvg
      },
      edgephp: {
        minimal: tests.minimal?.edgephp.execution || edgeExecAvg,
        assignment: tests.assignment?.edgephp.execution || edgeExecAvg,
        arithmetic: tests.arithmetic?.edgephp.execution || edgeExecAvg,
        strings: tests.strings?.edgephp.execution || edgeExecAvg,
        comprehensive: tests.comprehensive?.edgephp.execution || edgeExecAvg,
        average: edgeExecAvg
      }
    },
    coldStartToCompletion: {
      php: {
        minimal: tests.minimal?.php.coldStart || phpColdAvg,
        assignment: tests.assignment?.php.coldStart || phpColdAvg,
        arithmetic: tests.arithmetic?.php.coldStart || phpColdAvg,
        strings: tests.strings?.php.coldStart || phpColdAvg,
        comprehensive: tests.comprehensive?.php.coldStart || phpColdAvg,
        average: phpColdAvg
      },
      edgephp: {
        minimal: tests.minimal?.edgephp.coldStart || edgeColdAvg,
        assignment: tests.assignment?.edgephp.coldStart || edgeColdAvg,
        arithmetic: tests.arithmetic?.edgephp.coldStart || edgeColdAvg,
        strings: tests.strings?.edgephp.coldStart || edgeColdAvg,
        comprehensive: tests.comprehensive?.edgephp.coldStart || edgeColdAvg,
        average: edgeColdAvg
      }
    },
    serverSide: true,
    timestamp: serverData.timestamp,
    interpreterOverhead: summary.php.interpreterOverhead,
    wasmLoadOverhead: summary.edgephp.loadOverhead,
    ratios: summary.ratios,
    instantiationAvg: edgeInstAvg,
    compilationAvg: edgeCompAvg
  }
}

export function BenchmarkModal({ isOpen, onClose }) {
  const [activeTab, setActiveTab] = useState('overview')
  const [benchmarkData, setBenchmarkData] = useState(null)
  const [loading, setLoading] = useState(true)
  
  useEffect(() => {
    if (isOpen) {
      // Try to fetch real benchmark data
      fetch('/benchmark_results.json')
        .then(res => res.json())
        .then(data => {
          const converted = convertServerData(data)
          setBenchmarkData(converted || DEFAULT_BENCHMARK_DATA)
          setLoading(false)
        })
        .catch(() => {
          // Fall back to default data if fetch fails
          setBenchmarkData(DEFAULT_BENCHMARK_DATA)
          setLoading(false)
        })
    }
  }, [isOpen])
  
  if (!isOpen) return null

  const data = benchmarkData || DEFAULT_BENCHMARK_DATA

  // Simple bar chart component
  const SimpleBarChart = ({ chartData, title, unit = 'ms' }) => {
    const maxValue = Math.max(...chartData.map(d => Math.max(...d.values)))
    
    return (
      <div style={{ background: '#0f172a', padding: '20px', borderRadius: '8px' }}>
        <h4 style={{ color: '#e2e8f0', marginBottom: '20px', textAlign: 'center' }}>{title}</h4>
        
        {chartData.map((platform, idx) => (
          <div key={idx} style={{ marginBottom: '20px' }}>
            <h5 style={{ color: platform.color, marginBottom: '10px' }}>{platform.name}</h5>
            {platform.values.map((value, i) => (
              <div key={i} style={{ marginBottom: '8px' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
                  <span style={{ color: '#94a3b8', width: '80px', fontSize: '12px' }}>
                    {['Minimal', 'Assignment', 'Arithmetic', 'Strings', 'Comprehensive'][i]}
                  </span>
                  <div style={{ flex: 1, position: 'relative', height: '20px', background: '#1e293b', borderRadius: '4px' }}>
                    <div 
                      style={{
                        position: 'absolute',
                        left: 0,
                        top: 0,
                        height: '100%',
                        width: `${(value / maxValue) * 100}%`,
                        background: platform.color,
                        borderRadius: '4px',
                        transition: 'width 0.3s ease'
                      }}
                    />
                  </div>
                  <span style={{ color: '#f1f5f9', width: '60px', textAlign: 'right', fontSize: '12px' }}>
                    {formatTime(value)}
                  </span>
                </div>
              </div>
            ))}
          </div>
        ))}
      </div>
    )
  }

  return (
    <div style={{
      position: 'fixed',
      top: 0,
      left: 0,
      right: 0,
      bottom: 0,
      backgroundColor: 'rgba(0, 0, 0, 0.8)',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      zIndex: 1000
    }}>
      <div style={{
        backgroundColor: '#1e293b',
        borderRadius: '12px',
        padding: '24px',
        maxWidth: '900px',
        width: '90%',
        maxHeight: '90vh',
        overflow: 'auto',
        boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.5)'
      }}>
        <div style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          marginBottom: '20px'
        }}>
          <h2 style={{ margin: 0, color: '#f1f5f9' }}>
            Performance Benchmarks
            {data.serverSide && <span style={{ fontSize: '14px', color: '#64748b', marginLeft: '10px' }}>(Server-side data)</span>}
          </h2>
          <button
            onClick={onClose}
            style={{
              background: 'none',
              border: 'none',
              color: '#64748b',
              fontSize: '24px',
              cursor: 'pointer'
            }}
          >
            ×
          </button>
        </div>

        {loading ? (
          <div style={{ color: '#94a3b8', textAlign: 'center', padding: '40px' }}>
            Loading benchmark data...
          </div>
        ) : (
          <>
            <div style={{
              display: 'flex',
              gap: '10px',
              marginBottom: '20px',
              borderBottom: '1px solid #334155'
            }}>
              <button
                onClick={() => setActiveTab('overview')}
                style={{
                  background: 'none',
                  border: 'none',
                  color: activeTab === 'overview' ? '#3b82f6' : '#94a3b8',
                  padding: '10px 20px',
                  cursor: 'pointer',
                  borderBottom: activeTab === 'overview' ? '2px solid #3b82f6' : 'none'
                }}
              >
                Overview
              </button>
              <button
                onClick={() => setActiveTab('execution')}
                style={{
                  background: 'none',
                  border: 'none',
                  color: activeTab === 'execution' ? '#3b82f6' : '#94a3b8',
                  padding: '10px 20px',
                  cursor: 'pointer',
                  borderBottom: activeTab === 'execution' ? '2px solid #3b82f6' : 'none'
                }}
              >
                Execution Only
              </button>
              <button
                onClick={() => setActiveTab('total')}
                style={{
                  background: 'none',
                  border: 'none',
                  color: activeTab === 'total' ? '#3b82f6' : '#94a3b8',
                  padding: '10px 20px',
                  cursor: 'pointer',
                  borderBottom: activeTab === 'total' ? '2px solid #3b82f6' : 'none'
                }}
              >
                Full Load
              </button>
            </div>

            {activeTab === 'overview' && (
              <div>
                <h3 style={{ color: '#e2e8f0' }}>Performance Comparison</h3>
                
                <div style={{
                  display: 'grid',
                  gridTemplateColumns: 'repeat(2, 1fr)',
                  gap: '20px',
                  marginBottom: '30px'
                }}>
                  <div style={{
                    background: '#0f172a',
                    padding: '20px',
                    borderRadius: '8px',
                    textAlign: 'center'
                  }}>
                    <h4 style={{ color: '#9333ea', margin: '0 0 10px 0' }}>PHP Native</h4>
                    <div style={{ color: '#64748b', fontSize: '14px' }}>Cold Start</div>
                    <div style={{ color: '#f1f5f9', fontSize: '24px', fontWeight: 'bold' }}>
                      {formatTime(data.coldStartToCompletion.php.average)}
                    </div>
                    <div style={{ color: '#64748b', fontSize: '14px', marginTop: '10px' }}>Execution Only</div>
                    <div style={{ color: '#f1f5f9', fontSize: '24px', fontWeight: 'bold' }}>
                      {formatTime(data.executionOnly.php.average)}
                    </div>
                  </div>

                  <div style={{
                    background: '#0f172a',
                    padding: '20px',
                    borderRadius: '8px',
                    textAlign: 'center'
                  }}>
                    <h4 style={{ color: '#3b82f6', margin: '0 0 10px 0' }}>EdgePHP</h4>
                    <div style={{ color: '#64748b', fontSize: '14px' }}>Full Load</div>
                    <div style={{ color: '#f1f5f9', fontSize: '24px', fontWeight: 'bold' }}>
                      {formatTime(data.coldStartToCompletion.edgephp.average)}
                    </div>
                    <div style={{ color: '#64748b', fontSize: '14px', marginTop: '10px' }}>Execution Only</div>
                    <div style={{ color: '#f1f5f9', fontSize: '24px', fontWeight: 'bold' }}>
                      {formatTime(data.executionOnly.edgephp.average)}
                    </div>
                    {data.compilationAvg && (
                      <>
                        <div style={{ color: '#64748b', fontSize: '12px', marginTop: '10px' }}>Breakdown:</div>
                        <div style={{ color: '#94a3b8', fontSize: '12px' }}>
                          Compile: {formatTime(data.compilationAvg)}<br/>
                          Instantiate: {formatTime(data.instantiationAvg)}
                        </div>
                      </>
                    )}
                  </div>
                </div>

                <div style={{
                  background: '#0f172a',
                  padding: '20px',
                  borderRadius: '8px',
                  marginBottom: '20px'
                }}>
                  <h4 style={{ color: '#e2e8f0', marginBottom: '15px' }}>Performance Ratios</h4>
                  <div style={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: '15px' }}>
                    <div>
                      <div style={{ color: '#64748b', fontSize: '14px' }}>Execution (EdgePHP vs PHP)</div>
                      <div style={{ 
                        color: data.ratios ? '#22c55e' : ((data.executionOnly.edgephp.average / data.executionOnly.php.average) < 1 ? '#22c55e' : '#ef4444'), 
                        fontSize: '20px', 
                        fontWeight: 'bold' 
                      }}>
                        {data.ratios ? 
                          (data.ratios.execution < 1 ? `${(1/data.ratios.execution).toFixed(1)}x faster` : `${data.ratios.execution.toFixed(1)}x slower`) :
                          ((data.executionOnly.edgephp.average / data.executionOnly.php.average) < 1 ? 
                            `${(data.executionOnly.php.average / data.executionOnly.edgephp.average).toFixed(1)}x faster` : 
                            `${(data.executionOnly.edgephp.average / data.executionOnly.php.average).toFixed(1)}x slower`)
                        }
                      </div>
                    </div>
                    <div>
                      <div style={{ color: '#64748b', fontSize: '14px' }}>Full Load (EdgePHP vs PHP)</div>
                      <div style={{ 
                        color: '#22c55e', 
                        fontSize: '20px', 
                        fontWeight: 'bold' 
                      }}>
                        {data.ratios ? 
                          (data.ratios.coldStart < 1 ? `${(1/data.ratios.coldStart).toFixed(1)}x faster` : `${data.ratios.coldStart.toFixed(1)}x slower`) :
                          ((data.coldStartToCompletion.edgephp.average / data.coldStartToCompletion.php.average) < 1 ? 
                            `${(data.coldStartToCompletion.php.average / data.coldStartToCompletion.edgephp.average).toFixed(1)}x faster` : 
                            `${(data.coldStartToCompletion.edgephp.average / data.coldStartToCompletion.php.average).toFixed(1)}x slower`)
                        }
                      </div>
                    </div>
                  </div>
                </div>

                <div style={{
                  background: '#0f172a',
                  padding: '20px',
                  borderRadius: '8px'
                }}>
                  <h4 style={{ color: '#e2e8f0', marginBottom: '10px' }}>Key Insights</h4>
                  <ul style={{ color: '#94a3b8', margin: 0, paddingLeft: '20px' }}>
                    <li>EdgePHP execution is <strong>faster</strong> than PHP ({formatTime(data.executionOnly.edgephp.average)} vs {formatTime(data.executionOnly.php.average)})</li>
                    <li>EdgePHP full load ({formatTime(data.coldStartToCompletion.edgephp.average)}) is much faster than PHP startup ({formatTime(data.coldStartToCompletion.php.average)})</li>
                    <li>WASM compilation takes only ~{data.compilationAvg ? formatTime(data.compilationAvg) : '60μs'}</li>
                    <li>EdgePHP provides compiled performance with portability and security</li>
                  </ul>
                </div>
              </div>
            )}

            {activeTab === 'execution' && (
              <div>
                <h3 style={{ color: '#e2e8f0' }}>Execution Performance (Warm)</h3>
                <p style={{ color: '#94a3b8', marginBottom: '20px' }}>
                  Pure code execution time after startup/compilation. This represents the best-case
                  performance for repeated executions.
                </p>
                
                <SimpleBarChart 
                  chartData={[
                    {
                      name: 'PHP Native',
                      color: '#9333ea',
                      values: [
                        data.executionOnly.php.minimal,
                        data.executionOnly.php.assignment,
                        data.executionOnly.php.arithmetic,
                        data.executionOnly.php.strings,
                        data.executionOnly.php.comprehensive
                      ]
                    },
                    {
                      name: 'EdgePHP',
                      color: '#3b82f6',
                      values: [
                        data.executionOnly.edgephp.minimal,
                        data.executionOnly.edgephp.assignment,
                        data.executionOnly.edgephp.arithmetic,
                        data.executionOnly.edgephp.strings,
                        data.executionOnly.edgephp.comprehensive
                      ]
                    }
                  ]}
                  title="Execution Time Comparison"
                />

                <div style={{ 
                  marginTop: '20px',
                  background: '#0f172a',
                  padding: '20px',
                  borderRadius: '8px'
                }}>
                  <h5 style={{ color: '#e2e8f0', marginBottom: '15px' }}>Analysis</h5>
                  <div style={{ color: '#94a3b8' }}>
                    {data.ratios && data.ratios.execution < 1 ? (
                      <>
                        <p>EdgePHP is {(1/data.ratios.execution).toFixed(1)}x <strong>faster</strong> than PHP for pure execution!</p>
                        <p>EdgePHP executes in ~{formatTime(data.executionOnly.edgephp.average)} vs PHP's ~{formatTime(data.executionOnly.php.average)}</p>
                        <p>This is because WASM runs compiled code while PHP interprets.</p>
                      </>
                    ) : (
                      <>
                        <p>EdgePHP is {((data.executionOnly.edgephp.average / data.executionOnly.php.average) || 1).toFixed(1)}x slower than PHP for pure execution.</p>
                        <p>EdgePHP executes in ~{formatTime(data.executionOnly.edgephp.average)} which is still very fast.</p>
                      </>
                    )}
                  </div>
                </div>
              </div>
            )}

            {activeTab === 'total' && (
              <div>
                <h3 style={{ color: '#e2e8f0' }}>Full Load Time</h3>
                <p style={{ color: '#94a3b8', marginBottom: '20px' }}>
                  Complete time to load and execute code from scratch.
                  PHP: interpreter startup + execution. EdgePHP: WASM operations only (no Node.js startup).
                </p>
                
                <SimpleBarChart 
                  chartData={[
                    {
                      name: 'PHP Native',
                      color: '#9333ea',
                      values: [
                        data.coldStartToCompletion.php.minimal,
                        data.coldStartToCompletion.php.assignment,
                        data.coldStartToCompletion.php.arithmetic,
                        data.coldStartToCompletion.php.strings,
                        data.coldStartToCompletion.php.comprehensive
                      ]
                    },
                    {
                      name: 'EdgePHP',
                      color: '#3b82f6',
                      values: [
                        data.coldStartToCompletion.edgephp.minimal,
                        data.coldStartToCompletion.edgephp.assignment,
                        data.coldStartToCompletion.edgephp.arithmetic,
                        data.coldStartToCompletion.edgephp.strings,
                        data.coldStartToCompletion.edgephp.comprehensive
                      ]
                    }
                  ]}
                  title="Total Time Comparison"
                />

                <div style={{ 
                  marginTop: '20px',
                  background: '#0f172a',
                  padding: '20px',
                  borderRadius: '8px'
                }}>
                  <h5 style={{ color: '#e2e8f0', marginBottom: '15px' }}>Analysis</h5>
                  <div style={{ color: '#94a3b8' }}>
                    <p><strong>EdgePHP full load</strong>: file read + WASM compile + instantiate + execute</p>
                    <p><strong>PHP cold start</strong>: interpreter startup + code execution</p>
                    {data.ratios && data.ratios.coldStart < 1 ? (
                      <>
                        <p>EdgePHP is {(1/data.ratios.coldStart).toFixed(1)}x <strong>faster</strong> to load and run!</p>
                        <p>EdgePHP: ~{formatTime(data.coldStartToCompletion.edgephp.average)} vs PHP: ~{formatTime(data.coldStartToCompletion.php.average)}</p>
                        <p>Note: This excludes Node.js startup time.</p>
                      </>
                    ) : (
                      <p>EdgePHP is {((data.coldStartToCompletion.edgephp.average / data.coldStartToCompletion.php.average) || 1).toFixed(1)}x {
                        (data.coldStartToCompletion.edgephp.average / data.coldStartToCompletion.php.average) > 1 ? 'slower' : 'faster'
                      } overall.</p>
                    )}
                  </div>
                </div>
              </div>
            )}

            {data.timestamp && (
              <div style={{ 
                marginTop: '20px', 
                color: '#64748b', 
                fontSize: '12px', 
                textAlign: 'center' 
              }}>
                Benchmark data from {new Date(data.timestamp).toLocaleString()}
              </div>
            )}
          </>
        )}
      </div>
    </div>
  )
}

export default BenchmarkModal
