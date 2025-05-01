#[cfg(test)]
mod parallel_tests {
    use crate::{compile_with_options, CompileOptions, OptimizationLevel};
    
    #[test]
    fn test_parallel_runtime_inclusion() {
        // Simple function that will include the runtime helper
        let rust_code = r#"
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;
        
        // Compile with runtime helpers
        let result = compile_with_options(rust_code, CompileOptions {
            include_runtime: true,
            debug_mode: false,
            enable_parallelization: false, // We're just testing inclusion
            optimization_level: OptimizationLevel::Default,
            generate_type_declarations: false,
            type_declarations_dir: None,
            rojo_project_path: None,
            serve_with_rojo: false,
            open_rojo_browser: false,
        });
        
        assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
        let luau_code = result.unwrap();
        
        // Check for parallel execution utilities in the runtime
        assert!(luau_code.contains("RobloxRS.Parallel"), "Parallel execution utilities not included");
        assert!(luau_code.contains("function RobloxRS.Parallel.forEach") || 
                luau_code.contains("RobloxRS.Parallel.forEach = function"), 
                "forEach function not found");
        assert!(luau_code.contains("function RobloxRS.Parallel.map") || 
                luau_code.contains("RobloxRS.Parallel.map = function"), 
                "map function not found");
        assert!(luau_code.contains("function RobloxRS.Parallel.filter") || 
                luau_code.contains("RobloxRS.Parallel.filter = function"), 
                "filter function not found");
    }
    
    #[test]
    fn test_coroutine_support() {
        // Simple function for runtime inclusion
        let rust_code = r#"
            fn sample() -> i32 { 42 }
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
        
        // Verify coroutine support in the runtime
        assert!(luau_code.contains("coroutine"), "Coroutine implementation not found");
        assert!(luau_code.contains("thread") || luau_code.contains("threads"), "Thread management not found");
        assert!(luau_code.contains("coroutine.create"), "Coroutine creation not found");
        assert!(luau_code.contains("coroutine.resume"), "Coroutine resumption not found");
    }
}
