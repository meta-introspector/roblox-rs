// Roblox-RS Transpiler
// This module contains the core transpiler logic for converting Rust to Luau

pub mod ast;
pub mod codegen;
pub mod parser;
pub mod transformer;

use std::fs;
use std::path::Path;
use std::error::Error;

/// The main transpile function that converts Rust source code to Luau
pub fn transpile(source: &str) -> Result<String, Box<dyn Error>> {
    // Parse Rust code into a syn AST
    let rust_ast = syn::parse_file(source)?;
    
    // Transform Rust AST to our Luau AST
    let luau_ast = ast::map_rust_to_luau(&rust_ast)
        .map_err(|e| format!("AST transformation error: {}", e))?;
    
    // Generate Luau code from our AST
    let luau_code = codegen::generate_luau(&luau_ast)
        .map_err(|e| format!("Code generation error: {}", e))?;
    
    Ok(luau_code)
}

/// Transpiles a Rust file to a Luau file
pub fn transpile_file(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn Error>> {
    // Read the input file
    let source = fs::read_to_string(input_path)?;
    
    // Transpile the source
    let luau_code = transpile(&source)?;
    
    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write the output file
    fs::write(output_path, luau_code)?;
    
    Ok(())
}

/// Transpile an entire directory recursively
pub fn transpile_directory(
    input_dir: &Path,
    output_dir: &Path,
    file_extension: &str,
) -> Result<(), Box<dyn Error>> {
    // Ensure output directory exists
    fs::create_dir_all(output_dir)?;
    
    // Iterate through entries in the input directory
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == file_extension) {
            // Construct output path
            let file_stem = path.file_stem().unwrap_or_default();
            let relative_path = path.strip_prefix(input_dir).unwrap_or(&path);
            let output_path = output_dir.join(relative_path)
                .with_file_name(file_stem)
                .with_extension("lua");
            
            // Transpile the file
            transpile_file(&path, &output_path)?;
        } else if path.is_dir() {
            // Recursively transpile subdirectories
            let subdir_name = path.file_name().unwrap_or_default();
            let output_subdir = output_dir.join(subdir_name);
            transpile_directory(&path, &output_subdir, file_extension)?;
        }
    }
    
    Ok(())
}
