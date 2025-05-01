#[cfg(test)]
mod rojo_tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use crate::{compile_with_options, CompileOptions, OptimizationLevel};
    use crate::rojo::RojoManager;
    
    fn setup_test_dir() -> PathBuf {
        let test_dir = tempfile::tempdir().unwrap();
        let test_path = test_dir.path().to_path_buf();
        
        // We don't call into_path() on tempdir so it gets cleaned up automatically
        test_path
    }
    
    #[test]
    fn test_rojo_project_creation() {
        let test_dir = setup_test_dir();
        let project_path = test_dir.join("default.project.json");
        
        // Create a new Rojo manager and create a default project
        let mut rojo_manager = RojoManager::new(&project_path);
        let result = rojo_manager.create_default_project("test-project", &test_dir.join("src"));
        
        assert!(result.is_ok(), "Failed to create project: {:?}", result.err());
        assert!(project_path.exists(), "Project file was not created");
        
        // Check that the project file contains the expected content
        let content = fs::read_to_string(&project_path).unwrap();
        assert!(content.contains("test-project"), "Project name not found in project file");
        assert!(content.contains("ServerScriptService"), "ServerScriptService not found in project file");
        assert!(content.contains("ReplicatedStorage"), "ReplicatedStorage not found in project file");
    }
    
    #[test]
    fn test_compile_with_rojo_project() {
        let test_dir = setup_test_dir();
        let project_path = test_dir.join("game.project.json");
        
        // Simple Rust function
        let rust_code = r#"
            fn greet(name: &str) -> String {
                format!("Hello, {}!", name)
            }
        "#;
        
        // Compile with Rojo project path
        let result = compile_with_options(rust_code, CompileOptions {
            include_runtime: true,
            debug_mode: false,
            enable_parallelization: false,
            optimization_level: OptimizationLevel::Default,
            generate_type_declarations: false,
            type_declarations_dir: None,
            rojo_project_path: Some(project_path.to_string_lossy().to_string()),
            serve_with_rojo: false,  // Don't actually start the server in tests
            open_rojo_browser: false,
        });
        
        assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
        
        // Verify that the project file was created
        assert!(project_path.exists(), "Project file was not created");
        
        // Verify that the compiled Lua file was created
        let lua_file = test_dir.join("src").join("main.lua");
        assert!(lua_file.exists(), "Compiled Lua file was not created");
        
        // Check the content of the compiled file
        let content = fs::read_to_string(lua_file).unwrap();
        assert!(content.contains("greet"), "Function name not found in compiled code");
        assert!(content.contains("Hello"), "Function body not found in compiled code");
    }
    
    #[test]
    fn test_rojo_source_mapping() {
        let test_dir = setup_test_dir();
        let project_path = test_dir.join("default.project.json");
        
        // Create source structure
        let src_dir = test_dir.join("src");
        let server_dir = src_dir.join("server");
        let shared_dir = src_dir.join("shared");
        
        fs::create_dir_all(&server_dir).unwrap();
        fs::create_dir_all(&shared_dir).unwrap();
        
        // Create Rust files
        let server_file = server_dir.join("main.rs");
        let shared_file = shared_dir.join("utils.rs");
        
        fs::write(&server_file, "fn main() { println!(\"Server\"); }").unwrap();
        fs::write(&shared_file, "fn utils() { println!(\"Shared\"); }").unwrap();
        
        // Create and load Rojo project
        let mut rojo_manager = RojoManager::new(&project_path);
        rojo_manager.create_default_project("test-project", &src_dir).unwrap();
        rojo_manager.load_project().unwrap();
        
        // Test source mapping functionality
        let server_output = rojo_manager.get_output_path(&server_file);
        let shared_output = rojo_manager.get_output_path(&shared_file);
        
        assert!(server_output.is_some(), "Server file mapping not found");
        assert!(shared_output.is_some(), "Shared file mapping not found");
        
        if let Some(path) = server_output {
            assert_eq!(path.extension().unwrap(), "lua", "Output should have .lua extension");
        }
        
        if let Some(path) = shared_output {
            assert_eq!(path.extension().unwrap(), "lua", "Output should have .lua extension");
        }
    }
}
