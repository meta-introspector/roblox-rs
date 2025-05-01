//! Error handling for roblox-rs-core.

use thiserror::Error;

/// Custom error type for roblox-rs-core.
#[derive(Error, Debug)]
pub enum Error {
    /// Error during Rust parsing
    #[error("Parse error: {0}")]
    Parse(String),
    
    /// Error during transformation to Luau
    #[error("Transform error: {0}")]
    Transform(String),
    
    /// Error during code generation
    #[error("Code generation error: {0}")]
    CodeGen(String),
    
    /// Error during optimization
    #[error("Optimization error: {0}")]
    Optimization(String),
    
    /// Error related to Roblox API
    #[error("Roblox API error: {0}")]
    RobloxApi(String),
    
    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}

/// Specialized Result type for roblox-rs-core.
pub type Result<T> = std::result::Result<T, Error>; 