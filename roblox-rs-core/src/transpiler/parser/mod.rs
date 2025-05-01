// Roblox-RS Parser
// Provides utilities for parsing Rust code into a syn AST

use std::error::Error;
use syn;

/// Parse Rust code into a syn AST
pub fn parse_rust(source: &str) -> Result<syn::File, Box<dyn Error>> {
    let ast = syn::parse_file(source)?;
    Ok(ast)
}

/// Parse a Rust file into a syn AST
pub fn parse_rust_file(file_path: &str) -> Result<syn::File, Box<dyn Error>> {
    let source = std::fs::read_to_string(file_path)?;
    parse_rust(&source)
}
