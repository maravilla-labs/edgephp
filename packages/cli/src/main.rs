// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
use wasm_opt::{OptimizationOptions, Feature};

mod test_v2;

#[derive(Parser)]
#[command(name = "edge-php")]
#[command(about = "Edge PHP - Modern PHP runtime compiled to WebAssembly", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a PHP file
    Run {
        /// The PHP file to run
        file: PathBuf,
    },
    /// Parse a PHP file and output the AST
    Parse {
        /// The PHP file to parse
        file: PathBuf,
    },
    /// Compile a PHP file to WASM
    Compile {
        /// The PHP file to compile
        file: PathBuf,
        /// Output WASM file
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Optimize the generated WASM using wasm-opt
        #[arg(long)]
        optimize: bool,
        /// Deprecated: kept for compatibility
        #[arg(long, hide = true)]
        v2: bool,
    },
    /// Test the new compiler
    TestV2,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => {
            let source = fs::read_to_string(&file)?;
            let mut runtime = edge_php_runtime::Runtime::new()?;
            let output = runtime.execute_php(&source)?;
            println!("{}", output);
        }
        Commands::Parse { file } => {
            let source = fs::read_to_string(&file)?;
            let ast = edge_php_parser::parse(&source)?;
            println!("{:#?}", ast);
        }
        Commands::Compile { file, output, optimize, v2: _ } => {
            let source = fs::read_to_string(&file)?;
            
            // Use CompilerV2 as the main compiler
            let compiler = edge_php_compiler::Compiler::new();
            let mut wasm_bytes = compiler.compile(&source)?;
            
            // Apply optimization if requested and available
            if optimize {
                wasm_bytes = optimize_wasm(wasm_bytes)?;
            }
            
            let output_path = output.unwrap_or_else(|| {
                let mut path = file.clone();
                path.set_extension("wasm");
                path
            });
            
            fs::write(&output_path, &wasm_bytes)?;
            
            if optimize {
                println!("Compiled and optimized successfully to: {}", output_path.display());
            } else {
                println!("Compiled successfully to: {}", output_path.display());
            }
        }
        Commands::TestV2 => {
            test_v2::run_compiler_v2_test();
        }
    }

    Ok(())
}

/// Optimize WASM bytecode using wasm-opt
#[cfg(not(target_arch = "wasm32"))]
fn optimize_wasm(wasm_bytes: Vec<u8>) -> Result<Vec<u8>> {
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    // Create temporary input file
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(&wasm_bytes)?;
    let input_path = input_file.path();
    
    // Create temporary output file  
    let output_file = NamedTempFile::new()?;
    let output_path = output_file.path();
    
    // Apply aggressive optimization (equivalent to -O4/new_opt_level_4)
    // Enable GC features that EdgePHP uses (struct.get, array.new, etc.)
    OptimizationOptions::new_opt_level_4()
        .enable_feature(Feature::Gc)
        .enable_feature(Feature::ReferenceTypes)
        .run(input_path, output_path)?;
    
    // Read optimized bytes from output file
    let optimized_bytes = std::fs::read(output_path)?;
    
    Ok(optimized_bytes)
}

/// Fallback for wasm32 targets where wasm-opt is not available
#[cfg(target_arch = "wasm32")]
fn optimize_wasm(wasm_bytes: Vec<u8>) -> Result<Vec<u8>> {
    eprintln!("Warning: WASM optimization is not available on wasm32 targets");
    Ok(wasm_bytes)
}
