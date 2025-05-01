//! Core compiler functionality for roblox-rs-core.

use crate::error::{Error, Result};

/// Optimization level for the compilation process
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    /// Minimal optimizations (faster compilation)
    Minimal,
    /// Default optimizations (balanced)
    Default,
    /// Aggressive optimizations (slower compilation, potentially faster execution)
    Aggressive,
}

/// Compilation options
#[derive(Debug, Clone)]
pub struct CompileOptions {
    /// Whether to include runtime helpers in the compiled output
    pub include_runtime: bool,
    /// Whether to add debug information to the compiled output
    pub debug_mode: bool,
    /// Whether to attempt automatic parallelization
    pub enable_parallelization: bool,
    /// Optimization level to apply
    pub optimization_level: OptimizationLevel,
    /// Target directory for compiled output
    pub target_dir: Option<String>,
    /// Additional compiler flags
    pub flags: Vec<String>,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            include_runtime: true,
            debug_mode: false,
            enable_parallelization: false,
            optimization_level: OptimizationLevel::Default,
            target_dir: None,
            flags: Vec::new(),
        }
    }
}

/// Compile Rust code to Luau with the given options.
#[cfg(feature = "ast-parser")]
pub fn compile(input: &str, options: CompileOptions) -> Result<String> {
    use crate::ast::parser::parse_rust;
    
    // Parse the Rust code
    let ast = parse_rust(input).map_err(|e| Error::Parse(e.to_string()))?;
    
    // Transform to Luau code
    #[cfg(feature = "ast-parser")]
    {
        let luau_ast = crate::ast::transformer::transform_ast(&ast)?;
        
        // Generate Luau code
        let mut luau_code = crate::luau::generator::generate_code(&luau_ast)
            .map_err(|e| Error::CodeGen(e.to_string()))?;
        
        // Apply optimizations if requested
        if options.optimization_level != OptimizationLevel::Minimal {
            luau_code = crate::luau::optimizer::optimize_code(&luau_code, &options)
                .map_err(|e| Error::Optimization(e.to_string()))?;
        }
        
        // Add debug information if requested
        if options.debug_mode {
            luau_code = format!("--!optimize 2\n--!native\n\n{}", luau_code);
        }
        
        Ok(luau_code)
    }
    
    #[cfg(not(feature = "ast-parser"))]
    {
        Err(Error::Other("AST parsing feature is not enabled".to_string()))
    }
}

#[cfg(not(feature = "ast-parser"))]
pub fn compile(_input: &str, _options: CompileOptions) -> Result<String> {
    Err(Error::Other("No compiler implementation enabled. Enable either 'ast-parser' or 'llvm-ir' feature.".to_string()))
}

/// Compile Rust code to Luau with default options.
pub fn compile_default(input: &str) -> Result<String> {
    compile(input, CompileOptions::default())
} 