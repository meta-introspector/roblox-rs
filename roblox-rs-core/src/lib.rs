//! Core library for the roblox-rs compiler.
//!
//! This crate is responsible for translating Rust code to Luau for the Roblox platform.

// Re-export core modules
pub mod compiler;
pub mod error;
pub mod macros;

// Feature-gated modules
#[cfg(feature = "ast-parser")]
pub mod ast;

#[cfg(feature = "llvm-ir")]
pub mod ir;

#[cfg(feature = "roblox-api")]
pub mod roblox;

pub mod luau;
pub mod utils;

// Re-exports for convenience
pub use compiler::{compile, CompileOptions};
pub use error::{Error, Result};

// Re-export macros
pub use crate::macros::*;

/// Current version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let result = compile("fn main() { println!(\"Hello, world!\"); }", Default::default());
        assert!(result.is_ok());
    }
}
