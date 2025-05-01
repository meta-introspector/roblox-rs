#[cfg(test)]
mod rojo_integration_tests {
    use crate::{compile_with_options, CompileOptions, OptimizationLevel};
    use std::path::{PathBuf};
    use std::fs;
    use std::env;
    
    #[test]
    fn test_compile_and_rojo_project() {
        // Simple Rust function
        let rust_code = r#"
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;
        
        // Create a temporary directory for testing
        let temp_dir = env::temp_dir().join("roblox_rs_test_rojo_integration");
        let _ = fs::create_dir_all(&temp_dir);
        let rojo_path = temp_dir.join("default.project.json");
        
        // Compile with Rojo project path
        let result = compile_with_options(rust_code, CompileOptions {
            include_runtime: true,
            debug_mode: false,
            enable_parallelization: false,
            optimization_level: OptimizationLevel::Default,
            generate_type_declarations: false,
            type_declarations_dir: None,
            rojo_project_path: Some(rojo_path.to_str().unwrap().to_string()),
            serve_with_rojo: false,
            open_rojo_browser: false,
        });
        
        assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
        let luau_code = result.unwrap();
        
        // Verify the compiled code looks good
        assert!(luau_code.contains("function add"), "Function not found in compiled code");
        assert!(luau_code.contains("return a + b"), "Function body not correct");
        
        // Verify that a Rojo project file was created
        assert!(rojo_path.exists(), "Rojo project file was not created");
        
        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
