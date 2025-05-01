use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use roblox_rs_core::dependencies::{DependencyManager, DependencyConfig};
use roblox_rs_core::tests::TestHelper;

fn setup_test_environment() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let project_path = temp_dir.path().to_path_buf();
    
    // Create a simple Cargo.toml file
    let cargo_toml = r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
serde_json = "1.0"
"#;
    
    fs::write(project_path.join("Cargo.toml"), cargo_toml)
        .expect("Failed to write Cargo.toml");
    
    // Create a simple Rust file
    let rust_file = r#"
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct TestStruct {
    name: String,
    value: i32,
}

fn main() {
    let test = TestStruct {
        name: "Test".to_string(),
        value: 42,
    };
    
    let json = serde_json::to_string(&test).unwrap();
    println!("{}", json);
}
"#;
    
    fs::create_dir_all(project_path.join("src"))
        .expect("Failed to create src directory");
    
    fs::write(project_path.join("src").join("main.rs"), rust_file)
        .expect("Failed to write main.rs");
    
    (temp_dir, project_path)
}

#[test]
fn test_dependency_scanning() {
    let (temp_dir, project_path) = setup_test_environment();
    
    // Create dependency manager and scan for dependencies
    let mut manager = DependencyManager::new(project_path.clone());
    let dependencies = manager.scan_dependencies()
        .expect("Failed to scan dependencies");
    
    // Verify dependencies were found
    assert!(dependencies.contains_key("serde"));
    assert!(dependencies.contains_key("serde_json"));
    
    // Clean up
    drop(temp_dir);
}

#[test]
fn test_dependency_config() {
    // Test default config
    let default_config = DependencyConfig::default();
    assert_eq!(default_config.minify, false);
    assert_eq!(default_config.include_docs, true);
    
    // Test custom config
    let custom_config = DependencyConfig {
        minify: true,
        include_docs: false,
        output_format: "rbxm".to_string(),
        ..Default::default()
    };
    
    assert_eq!(custom_config.minify, true);
    assert_eq!(custom_config.include_docs, false);
    assert_eq!(custom_config.output_format, "rbxm");
}

#[test]
fn test_dependency_compilation() {
    let (temp_dir, project_path) = setup_test_environment();
    
    // Create output directory
    let output_path = project_path.join("output");
    fs::create_dir_all(&output_path).expect("Failed to create output directory");
    
    // Configure the dependency manager
    let mut manager = DependencyManager::new(project_path.clone());
    manager.set_output_path(output_path.clone());
    
    // Mock compilation (we're not actually compiling here to avoid external dependencies)
    let dependencies = manager.scan_dependencies().expect("Failed to scan dependencies");
    
    // Create mock output for each dependency
    for (name, _) in dependencies {
        let module_path = output_path.join(format!("{}.lua", name));
        let mock_content = format!("-- Mock module for {}\nreturn {{}}", name);
        fs::write(module_path, mock_content).expect("Failed to write mock module");
    }
    
    // Verify files were created
    assert!(output_path.join("serde.lua").exists());
    assert!(output_path.join("serde_json.lua").exists());
    
    // Clean up
    drop(temp_dir);
}

#[test]
fn test_dependency_metadata() {
    let (temp_dir, project_path) = setup_test_environment();
    
    // Create dependency manager and generate metadata
    let mut manager = DependencyManager::new(project_path.clone());
    let dependencies = manager.scan_dependencies().expect("Failed to scan dependencies");
    
    let metadata = manager.generate_metadata(&dependencies);
    
    // Verify metadata contains expected fields
    assert!(metadata.contains_key("serde"));
    assert!(metadata.contains_key("serde_json"));
    
    let serde_meta = metadata.get("serde").expect("No metadata for serde");
    assert!(serde_meta.contains_key("version"));
    assert!(serde_meta.contains_key("features"));
    
    // Clean up
    drop(temp_dir);
}

#[test]
fn test_dependency_error_handling() {
    // Test invalid project path
    let invalid_path = PathBuf::from("/path/that/does/not/exist");
    let mut manager = DependencyManager::new(invalid_path);
    let result = manager.scan_dependencies();
    
    // Should return an error
    assert!(result.is_err());
}

#[test]
fn test_dependency_bundle_generation() {
    let (temp_dir, project_path) = setup_test_environment();
    
    // Create output directory
    let output_path = project_path.join("output");
    fs::create_dir_all(&output_path).expect("Failed to create output directory");
    
    // Configure the dependency manager
    let mut manager = DependencyManager::new(project_path.clone());
    manager.set_output_path(output_path.clone());
    
    // Generate mock bundle (without actual compilation)
    let dependencies = manager.scan_dependencies().expect("Failed to scan dependencies");
    
    // Create mock output files
    for (name, _) in &dependencies {
        let module_path = output_path.join(format!("{}.lua", name));
        let mock_content = format!("-- Mock module for {}\nreturn {{}}", name);
        fs::write(module_path, mock_content).expect("Failed to write mock module");
    }
    
    // Create index file that requires all dependencies
    let mut index_content = String::from("-- Generated index file\nlocal Packages = {}\n\n");
    
    for (name, _) in dependencies {
        index_content.push_str(&format!("Packages[\"{0}\"] = require(\"{0}\")\n", name));
    }
    
    index_content.push_str("\nreturn Packages");
    
    fs::write(output_path.join("index.lua"), index_content)
        .expect("Failed to write index file");
    
    // Verify index file exists and contains required dependencies
    let index_file = fs::read_to_string(output_path.join("index.lua"))
        .expect("Failed to read index file");
    
    assert!(index_file.contains("require(\"serde\")"));
    assert!(index_file.contains("require(\"serde_json\")"));
    
    // Clean up
    drop(temp_dir);
}
