#[cfg(test)]
mod luau_execution_tests {
    use crate::{compile_with_options, CompileOptions, OptimizationLevel};
    use std::fs;
    use crate::test_utils::validate_luau;
    
    // Helper function to compile Rust code to Luau and validate syntax
    fn compile_and_validate(rust_code: &str) -> Result<String, String> {
        // Compile with default options
        let compile_result = compile_with_options(rust_code, CompileOptions {
            include_runtime: true,
            debug_mode: false,
            enable_parallelization: false,
            optimization_level: OptimizationLevel::Default,
            generate_type_declarations: false,
            type_declarations_dir: None,
            rojo_project_path: None,
            serve_with_rojo: false,
            open_rojo_browser: false,
        })?;
        
        // Validate the Luau syntax
        match validate_luau(&compile_result) {
            Ok(_) => Ok(compile_result),
            Err(e) => Err(format!("Luau validation failed: {}", e))
        }
    }
    
    #[test]
    fn test_arithmetic_operations() {
        let rust_code = r#"
            // Simple addition function
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;
        
        // Compile the Rust code to Luau and validate syntax
        let result = compile_and_validate(rust_code);
        assert!(result.is_ok(), "Compilation or validation failed: {:?}", result.err());
        
        let luau_code = result.unwrap();
        println!("Compiled Luau code:\n{}", luau_code);
        
        // Verify the compiled code contains our function
        assert!(luau_code.contains("function add"), "Function not found in compiled code");
        assert!(luau_code.contains("return a + b"), "Function body not correctly compiled");
        
        // Save the compiled code to a file for reference
        let _ = fs::write("/tmp/roblox_rs_add_test.lua", &luau_code);
        println!("Compiled Luau code saved to /tmp/roblox_rs_add_test.lua for manual testing");
    }
    
    #[test]
    fn test_multiplication_operation() {
        let rust_code = r#"
            // Simple function that multiplies two numbers
            fn multiply(a: i32, b: i32) -> i32 {
                a * b
            }
        "#;
        
        // Compile the Rust code to Luau and validate syntax
        let result = compile_and_validate(rust_code);
        assert!(result.is_ok(), "Compilation or validation failed: {:?}", result.err());
        
        let luau_code = result.unwrap();
        println!("Compiled Luau code:\n{}", luau_code);
        
        // Verify the compiled code contains our function
        assert!(luau_code.contains("function multiply"), "Function not found in compiled code");
        assert!(luau_code.contains("return a * b"), "Function body not correctly compiled");
        
        // Save the compiled code to a file for reference
        let _ = fs::write("/tmp/roblox_rs_multiply_test.lua", &luau_code);
        println!("Compiled Luau code saved to /tmp/roblox_rs_multiply_test.lua for manual testing");
    }
    
    #[test]
    fn test_complex_expressions() {
        let rust_code = r#"
            // Function with more complex expressions
            fn calculate(a: i32, b: i32, c: i32) -> i32 {
                a * b + c
            }
        "#;
        
        // Compile the Rust code to Luau and validate syntax
        let result = compile_and_validate(rust_code);
        assert!(result.is_ok(), "Compilation or validation failed: {:?}", result.err());
        
        let luau_code = result.unwrap();
        println!("Compiled Luau code:\n{}", luau_code);
        
        // Verify the compiled code contains our function
        assert!(luau_code.contains("function calculate"), "Function not found in compiled code");
        assert!(luau_code.contains("return a * b + c"), "Function body not correctly compiled");
        
        // Save the compiled code to a file for reference
        let _ = fs::write("/tmp/roblox_rs_calculate_test.lua", &luau_code);
        println!("Compiled Luau code saved to /tmp/roblox_rs_calculate_test.lua for manual testing");
    }
    
    #[test]
    fn test_control_flow() {
        let rust_code = r#"
            // Function with if/else control flow
            fn max(a: i32, b: i32) -> i32 {
                if a > b {
                    a
                } else {
                    b
                }
            }
        "#;
        
        // Compile the Rust code to Luau and validate syntax
        let result = compile_and_validate(rust_code);
        assert!(result.is_ok(), "Compilation or validation failed: {:?}", result.err());
        
        let luau_code = result.unwrap();
        println!("Compiled Luau code:\n{}", luau_code);
        
        // Verify the compiled code contains our function and control flow
        assert!(luau_code.contains("function max"), "Function not found in compiled code");
        assert!(luau_code.contains("if"), "If statement not found in compiled code");
        assert!(luau_code.contains("else"), "Else statement not found in compiled code");
        assert!(luau_code.contains("a > b"), "Comparison not correctly compiled");
        
        // Save the compiled code to a file for reference
        let _ = fs::write("/tmp/roblox_rs_max_test.lua", &luau_code);
        println!("Compiled Luau code saved to /tmp/roblox_rs_max_test.lua for manual testing");
    }
}
