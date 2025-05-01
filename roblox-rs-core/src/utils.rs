//! Utility functions for roblox-rs-core.

use std::path::{Path, PathBuf};

/// Check if a file has a specific extension.
pub fn has_extension(path: &Path, ext: &str) -> bool {
    path.extension()
        .map(|e| e.to_string_lossy().to_lowercase() == ext.to_lowercase())
        .unwrap_or(false)
}

/// Convert a relative path to an absolute path.
pub fn to_absolute_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map(|cwd| cwd.join(path))
            .unwrap_or_else(|_| path.to_path_buf())
    }
}

/// Create a temporary file with the given content and extension.
#[cfg(feature = "tempfile")]
pub fn create_temp_file(content: &str, extension: &str) -> std::io::Result<PathBuf> {
    use std::io::Write;
    
    let mut temp_file = tempfile::Builder::new()
        .suffix(&format!(".{}", extension))
        .tempfile()?;
    
    temp_file.write_all(content.as_bytes())?;
    let path = temp_file.into_temp_path();
    
    // Convert to PathBuf and leak the temporary file so it's not deleted
    Ok(path.to_path_buf())
}

#[cfg(not(feature = "tempfile"))]
pub fn create_temp_file(_content: &str, _extension: &str) -> std::io::Result<PathBuf> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "Temporary file creation requires the 'tempfile' feature"
    ))
}

/// Get the filename without the extension.
pub fn filename_without_extension(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}

/// Convert a Rust identifier to a Luau identifier.
pub fn rust_to_luau_ident(ident: &str) -> String {
    // In Luau, identifiers are more restricted than in Rust
    // This function ensures that Rust identifiers are valid in Luau
    
    // Check if the identifier is a Luau keyword
    let luau_keywords = [
        "and", "break", "do", "else", "elseif", "end", "false", "for", 
        "function", "if", "in", "local", "nil", "not", "or", "repeat", 
        "return", "then", "true", "until", "while", "continue", "export"
    ];
    
    if luau_keywords.contains(&ident) {
        return format!("_{}", ident);
    }
    
    // Convert Rust's snake_case to camelCase for better Luau style
    // (optional, can be disabled)
    let mut result = String::new();
    let mut capitalize_next = false;
    
    for c in ident.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap_or(c));
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    
    result
} 