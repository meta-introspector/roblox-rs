use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn validate_luau(code: &str) -> Result<(), String> {
    // Create a temporary file
    let temp_file = NamedTempFile::new()
        .map_err(|e| format!("Failed to create temp file: {}", e))?;

    // Write the Luau code to the file
    fs::write(&temp_file, code)
        .map_err(|e| format!("Failed to write to temp file: {}", e))?;

    // Get the path as a string
    let path = temp_file.path().to_str()
        .ok_or_else(|| "Invalid temp file path".to_string())?;
    
    // Basic syntax validation checks
    
    // 1. Check for mismatched braces/parentheses/brackets
    let mut brace_count = 0;
    let mut paren_count = 0;
    let mut bracket_count = 0;
    
    for c in code.chars() {
        match c {
            '{' => brace_count += 1,
            '}' => brace_count -= 1,
            '(' => paren_count += 1,
            ')' => paren_count -= 1,
            '[' => bracket_count += 1,
            ']' => bracket_count -= 1,
            _ => {}
        }
        
        // Check for negative counts (closing without opening)
        if brace_count < 0 || paren_count < 0 || bracket_count < 0 {
            return Err(format!("Syntax error: Unmatched closing bracket/brace/parenthesis"));
        }
    }
    
    // Check for unclosed brackets/braces/parentheses
    if brace_count > 0 || paren_count > 0 || bracket_count > 0 {
        return Err(format!(
            "Syntax error: Unclosed brackets/braces/parentheses. Braces: {}, Parentheses: {}, Brackets: {}", 
            brace_count, paren_count, bracket_count
        ));
    }
    
    // 2. Check for function declarations
    if !code.contains("function ") {
        return Err("No function declarations found in generated code".to_string());
    }
    
    // 3. Check for basic Lua keywords
    let keywords = ["local", "return", "end", "function"];
    let contains_keywords = keywords.iter().any(|&keyword| code.contains(keyword));
    if !contains_keywords {
        return Err("No Lua keywords found in generated code".to_string());
    }

    // Print the generated code for debugging
    println!("Generated Luau code passed validation");

    Ok(())
}
