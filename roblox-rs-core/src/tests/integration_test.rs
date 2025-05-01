#[cfg(test)]
mod integration_tests {
    use crate::{compile_with_options, CompileOptions, OptimizationLevel};
    use crate::debug::RobloxRsDebugger;
    use crate::parallel::ParallelTransformer;

    #[test]
    fn test_compile_with_runtime_helpers() {
        let rust_code = r#"
            fn test_function(x: i32, y: i32) -> i32 {
                x + y
            }
        "#;

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
        
        // Runtime helpers should be included
        assert!(luau_code.contains("RobloxRS ="), "Runtime helpers not included in output");
        assert!(luau_code.contains("Pool"), "Object pooling not included in output");
        assert!(luau_code.contains("Parallel"), "Parallel execution not included in output");
    }

    #[test]
    fn test_compile_with_debug_mode() {
        let input = r#"
            fn test_function() {
                let x = 42;
                println!("{}", x);
            }
        "#;

        let options = CompileOptions {
            debug_mode: true,
            include_runtime: true,
            ..Default::default()
        };

        let output = compile_with_options(input, options)
            .expect("Failed to compile with debug mode");

        // Verify debug info is included
        assert!(output.contains("RobloxRS.Debug.line_info"), "Debug namespace not included");
        assert!(output.contains("variables = {}"), "Variable info not included");
        assert!(output.contains("test_function"), "Function name not included in debug info");
        
        // Debug info should be present
        assert!(output.contains("line_info"), "Line info not included in debug output");
        assert!(output.contains("variable_info"), "Variable info not included in debug output");
        assert!(output.contains("source_map"), "Source map not included in debug output");
    }

    #[test]
    fn test_compile_with_parallelization() {
        let rust_code = r#"
            fn multiply(a: i32, b: i32) -> i32 {
                a * b
            }
        "#;

        let result = compile_with_options(rust_code, CompileOptions {
            include_runtime: true,
            debug_mode: false,
            enable_parallelization: true,
            optimization_level: OptimizationLevel::Default,
            generate_type_declarations: false,
            type_declarations_dir: None,
            rojo_project_path: None,
            serve_with_rojo: false,
            open_rojo_browser: false,
        });

        assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn test_aggressive_optimization() {
        let rust_code = r#"
            fn add_three_numbers(x: i32, y: i32, z: i32) -> i32 {
                x + y + z
            }
            
            fn create_name(first: i32, last: i32) -> i32 {
                first * 1000 + last
            }
        "#;

        let result = compile_with_options(rust_code, CompileOptions {
            include_runtime: true,
            debug_mode: false,
            enable_parallelization: false,
            optimization_level: OptimizationLevel::Aggressive,
            generate_type_declarations: false,
            type_declarations_dir: None,
            rojo_project_path: None,
            serve_with_rojo: false,
            open_rojo_browser: false,
        });

        assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
        let luau_code = result.unwrap();
        
        // Optimization should produce a valid function
        assert!(luau_code.contains("function add_three_numbers"), 
            "Function not found in optimized output");
    }

    #[test]
    fn test_debugger() {
        let mut debugger = RobloxRsDebugger::new();
        
        // Initial state should be empty
        let initial_info = debugger.generate_debug_info();
        assert!(initial_info.source_map.is_empty());
        assert!(initial_info.variable_types.is_empty());
        
        // Add breakpoints and watches
        debugger.add_breakpoint(10);
        debugger.add_breakpoint(20);
        debugger.add_watch("test_var", "test_expression");
        
        // After adding breakpoints, source map should have entries
        let debug_info = debugger.generate_debug_info();
        assert_eq!(debug_info.source_map.len(), 2); // One entry per breakpoint
        assert!(debug_info.variable_types.is_empty());
        
        // Add some source mappings and type info for better test coverage
        debugger.add_source_mapping("line_10", "luau_line_20");
        debugger.add_type_info("test_var", "number");
        
        // Generate updated debug info
        let updated_debug_info = debugger.generate_debug_info();
        
        // Verify the updated source map and variable types
        assert_eq!(updated_debug_info.source_map.len(), 3); // 2 breakpoints + 1 manual mapping
        assert_eq!(updated_debug_info.variable_types.len(), 1);
    }
}
