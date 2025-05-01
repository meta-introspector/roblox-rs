//! Rust code parser module.
//!
//! This module provides functions for parsing Rust code into an AST.

use syn::{parse_file, File};
use thiserror::Error;

/// Error that can occur during parsing.
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("syntax error: {0}")]
    Syntax(#[from] syn::Error),
    
    #[error("unsupported feature: {0}")]
    UnsupportedFeature(String),
    
    #[error("other parse error: {0}")]
    Other(String),
}

/// Parse Rust code into a syn::File AST.
pub fn parse_rust(input: &str) -> Result<File, ParseError> {
    parse_file(input).map_err(ParseError::Syntax)
}

/// Parse Rust code with additional preprocessing.
///
/// This function applies preprocessing steps before parsing the code.
pub fn parse_rust_with_preprocessing(input: &str) -> Result<File, ParseError> {
    // In a real implementation, we would apply preprocessing here
    // For now, just pass through to the regular parser
    parse_rust(input)
}

/// Extract documentation comments from attributes.
pub fn extract_doc_comments(attrs: &[syn::Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            if let Ok(meta) = attr.meta.clone().require_name_value() {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        return Some(lit_str.value().trim().to_string());
                    }
                }
            }
            None
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_function() {
        let code = r#"
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;
        
        let result = parse_rust(code);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_parse_struct() {
        let code = r#"
            /// A simple point in 2D space.
            struct Point {
                x: f32,
                y: f32,
            }
            
            impl Point {
                fn new(x: f32, y: f32) -> Self {
                    Self { x, y }
                }
                
                fn distance(&self, other: &Point) -> f32 {
                    let dx = self.x - other.x;
                    let dy = self.y - other.y;
                    (dx * dx + dy * dy).sqrt()
                }
            }
        "#;
        
        let result = parse_rust(code);
        assert!(result.is_ok());
    }
} 