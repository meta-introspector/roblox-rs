#[cfg(test)]
mod basic_tests {
    use crate::{compile_with_options, CompileOptions, OptimizationLevel};
    
    #[test]
    fn test_runtime_helper_inclusion() {
        // Simple addition function
        let rust_code = r#"
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;
        
        // Compile with runtime helpers
        let result = compile_with_options(rust_code, CompileOptions {
            include_runtime: true,
            debug_mode: false,
            enable_parallelization: false,
            optimization_level: OptimizationLevel::Default,
            generate_type_declarations: false,
            type_declarations_dir: None,
            rojo_project_path: None,
            serve_with_rojo: false,
            open_rojo_browser: false,
        });
        
        assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
        let luau_code = result.unwrap();
        
        // Check for runtime helper components
        assert!(luau_code.contains("RobloxRS = {}"), "Runtime helper not included");
        assert!(luau_code.contains("RobloxRS.Pool"), "Object pooling not included");
        assert!(luau_code.contains("RobloxRS.Debug"), "Debug support not included");
        assert!(luau_code.contains("RobloxRS.Parallel"), "Parallel execution not included");
        
        // Now compile without runtime helpers
        let result_no_rt = compile_with_options(rust_code, CompileOptions {
            include_runtime: false,
            debug_mode: false,
            enable_parallelization: false,
            optimization_level: OptimizationLevel::Default,
            generate_type_declarations: false,
            type_declarations_dir: None,
            rojo_project_path: None,
            serve_with_rojo: false,
            open_rojo_browser: false,
        });
        
        assert!(result_no_rt.is_ok(), "Compilation failed: {:?}", result_no_rt.err());
        let luau_code_no_rt = result_no_rt.unwrap();
        
        // Check that runtime helpers are not included
        assert!(!luau_code_no_rt.contains("RobloxRS.Pool"), "Object pooling included when it shouldn't be");
    }
    
    #[test]
    fn test_debug_mode() {
        // Simple function with some variables
        let rust_code = r#"
            fn calculate(x: i32, y: i32) -> i32 {
                let z = x * y;
                z + 10
            }
        "#;
        
        // Compile with debug mode
        let result = compile_with_options(rust_code, CompileOptions {
            include_runtime: true,
            debug_mode: true,
            enable_parallelization: false,
            optimization_level: OptimizationLevel::Default,
            generate_type_declarations: false,
            type_declarations_dir: None,
            rojo_project_path: None,
            serve_with_rojo: false,
            open_rojo_browser: false,
        });
        
        assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
        let luau_code = result.unwrap();
        
        // Check for debug symbols
        assert!(luau_code.contains("DEBUG_SYMBOLS"), "Debug symbols not included");
    }
    
    #[test]
    fn test_basic_arithmetic() {
        // Test a variety of arithmetic operations
        let rust_code = r#"
            fn test_arithmetic(a: i32, b: i32) -> i32 {
                let c = a + b;
                let d = a - b;
                let e = a * b;
                let f = a / b;
                c + d + e + f
            }
        "#;
        
        // Compile with default options
        let result = compile_with_options(rust_code, CompileOptions {
            include_runtime: true,
            debug_mode: false,
            enable_parallelization: false,
            optimization_level: OptimizationLevel::Default,
            generate_type_declarations: false,
            type_declarations_dir: None,
            rojo_project_path: None,
            serve_with_rojo: false,
            open_rojo_browser: false,
        });
        
        assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
        let luau_code = result.unwrap();
        
        // Check for arithmetic operators
        assert!(luau_code.contains("+"), "Addition operator not found");
        assert!(luau_code.contains("-"), "Subtraction operator not found");
        assert!(luau_code.contains("*"), "Multiplication operator not found");
        assert!(luau_code.contains("/"), "Division operator not found");
    }
}
