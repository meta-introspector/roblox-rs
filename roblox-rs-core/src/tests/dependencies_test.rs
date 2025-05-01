use crate::dependencies::{DependencyManager, DependencyConfig, CompiledPackage};
use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a test environment
    fn setup_test_env() -> (PathBuf, PathBuf) {
        let temp_dir = std::env::temp_dir().join("roblox-rs-test");
        let project_dir = temp_dir.join("project");
        let output_dir = temp_dir.join("output");

        // Clean up any previous test data
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap_or_default();
        }

        // Create test directories
        fs::create_dir_all(&project_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();

        // Create a minimal Cargo.toml
        let cargo_toml = r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0.0"
rand = "0.8.0"
tokio = { version = "1.0", features = ["full"] }
"#;
        fs::write(project_dir.join("Cargo.toml"), cargo_toml).unwrap();

        // Create a test Rust file
        let lib_rs = r#"
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub struct Vector3 {
    pub x: f32,
    pub y: f32, 
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
"#;
        fs::create_dir_all(project_dir.join("src")).unwrap();
        fs::write(project_dir.join("src/lib.rs"), lib_rs).unwrap();

        (project_dir, output_dir)
    }

    #[test]
    fn test_dependency_scan() {
        let (project_dir, output_dir) = setup_test_env();
        
        let mut manager = DependencyManager::new(&project_dir, &output_dir);
        let configs = manager.scan_dependencies().expect("Failed to scan dependencies");
        
        // Verify we found the expected dependencies
        assert_eq!(configs.len(), 3);
        
        // Check for specific dependencies
        let found_serde = configs.iter().any(|config| config.name == "serde");
        let found_rand = configs.iter().any(|config| config.name == "rand");
        let found_tokio = configs.iter().any(|config| config.name == "tokio");
        
        assert!(found_serde, "serde dependency not found");
        assert!(found_rand, "rand dependency not found");
        assert!(found_tokio, "tokio dependency not found");
        
        // Check tokio has features
        let tokio = configs.iter().find(|config| config.name == "tokio").unwrap();
        assert!(!tokio.features.is_empty(), "tokio should have features");
    }

    #[test]
    fn test_mock_compilation() {
        let (project_dir, output_dir) = setup_test_env();
        
        // Create a mock compiled package
        let package = create_mock_package();
        
        // Create a manager and manually insert the package
        let mut manager = DependencyManager::new(&project_dir, &output_dir);
        manager.compiled_packages.insert(
            format!("{}@{}", package.name, package.version),
            package.clone()
        );
        
        // Write the package output
        manager.write_package_output(&package).expect("Failed to write package output");
        
        // Verify the output files were created
        let package_dir = output_dir.join("packages").join(&package.name);
        assert!(package_dir.exists(), "Package directory not created");
        
        let module_path = package_dir.join("init.lua");
        assert!(module_path.exists(), "Module file not created");
        
        let metadata_path = package_dir.join("package.json");
        assert!(metadata_path.exists(), "Metadata file not created");
    }

    // Helper function to create a mock compiled package
    fn create_mock_package() -> CompiledPackage {
        CompiledPackage {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            code: "-- Test Luau code\nlocal module = {}\nfunction module.add(a, b) return a + b end\nreturn module".to_string(),
            types: Some("-- Type definitions\nexport type TestModule = { add: (number, number) -> number }".to_string()),
            dependencies: vec!["dep1".to_string(), "dep2".to_string()],
        }
    }

    #[test]
    fn test_mock_dependency_config() {
        // Create a test dependency config
        let config = DependencyConfig {
            name: "test-crate".to_string(),
            version: "0.1.0".to_string(),
            main: PathBuf::from("src/lib.rs"),
            types: Some(PathBuf::from("types/index.d.ts")),
            repository: Some("https://github.com/test/test-crate".to_string()),
            metadata: {
                let mut map = HashMap::new();
                map.insert("author".to_string(), "Test Author".to_string());
                map
            },
            features: vec!["feature1".to_string(), "feature2".to_string()],
        };
        
        // Verify config fields
        assert_eq!(config.name, "test-crate");
        assert_eq!(config.version, "0.1.0");
        assert_eq!(config.main, PathBuf::from("src/lib.rs"));
        assert_eq!(config.types, Some(PathBuf::from("types/index.d.ts")));
        assert_eq!(config.repository, Some("https://github.com/test/test-crate".to_string()));
        assert_eq!(config.features, vec!["feature1".to_string(), "feature2".to_string()]);
        assert_eq!(config.metadata.get("author"), Some(&"Test Author".to_string()));
    }
}

// Integration test for the web bundler
#[cfg(test)]
mod web_tests {
    use crate::dependencies::web::{RobloxRsWebBundler, WebBundlerConfig};
    use std::collections::HashMap;
    
    // This test requires wasm support, so we'll mock it for now
    #[test]
    fn test_web_bundler_config() {
        // Create a web bundler config
        let config = WebBundlerConfig {
            project_name: "test-project".to_string(),
            output_format: "lua".to_string(),
            include_docs: true,
            minify: false,
            path_mapping: {
                let mut map = HashMap::new();
                map.insert("src".to_string(), "game.ReplicatedStorage.SourceModules".to_string());
                Some(map)
            },
        };
        
        // Verify config fields
        assert_eq!(config.project_name, "test-project");
        assert_eq!(config.output_format, "lua");
        assert!(config.include_docs);
        assert!(!config.minify);
        assert!(config.path_mapping.is_some());
        
        if let Some(mapping) = &config.path_mapping {
            assert_eq!(
                mapping.get("src"),
                Some(&"game.ReplicatedStorage.SourceModules".to_string())
            );
        }
    }
}
