#[cfg(test)]
mod pooling_tests {
    use crate::{compile_with_options, CompileOptions, OptimizationLevel};
    
    #[test]
    fn test_runtime_includes_pooling() {
        // Simple function that could benefit from object pooling
        let rust_code = r#"
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;
        
        // Compile with aggressive optimization and runtime helpers
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
        
        // Check for runtime helper components
        assert!(luau_code.contains("RobloxRS.Pool"), "Object pooling not included");
        assert!(luau_code.contains("function RobloxRS.Pool.new"), "Pool.new function not found");
        assert!(luau_code.contains("function pool:get()"), "Pool get method not found");
        assert!(luau_code.contains("function pool:release"), "Pool release method not found");
    }
    
    #[test]
    fn test_runtime_pool_features() {
        // Verify all the required pooling features are included
        let rust_code = r#"
            fn sample() -> i32 { 42 }
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
        
        // Check for specific pooling features
        assert!(luau_code.contains("pre-allocate") || 
                luau_code.contains("Pre-allocate") || 
                luau_code.contains("initialSize"), 
                "Pre-allocation feature not found");
                
        assert!(luau_code.contains("self.available"), "Available pool tracking not found");
        assert!(luau_code.contains("allocated"), "Allocation tracking not found");
    }
}
