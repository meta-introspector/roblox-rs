// Dependency management for Roblox-RS
// Similar to how roblox-ts handles npm packages, this module handles Rust crates

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use serde::{Deserialize, Serialize};

use crate::codegen::LuauCodeGenerator;
use crate::ast::luau::Program;
use crate::transform::RustToLuauTransformer;

/// Configuration for the dependency manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyConfig {
    /// The name of the dependency package
    pub name: String,
    /// The version of the dependency package
    pub version: String,
    /// The entry point for the dependency
    pub main: PathBuf,
    /// Type declarations for the dependency
    pub types: Option<PathBuf>,
    /// Repository URL for the dependency
    pub repository: Option<String>,
    /// Additional metadata for the dependency
    pub metadata: HashMap<String, String>,
    /// Package features to enable
    pub features: Vec<String>,
}

/// A compiled dependency package
#[derive(Debug, Clone)]
pub struct CompiledPackage {
    /// The name of the package
    pub name: String,
    /// The version of the package
    pub version: String,
    /// The compiled Luau code
    pub code: String,
    /// Type definitions
    pub types: Option<String>,
    /// Dependency packages this package depends on
    pub dependencies: Vec<String>,
}

/// Manager for handling Rust dependencies
pub struct DependencyManager {
    /// Root directory for the project
    pub root_dir: PathBuf,
    /// Output directory for compiled dependencies
    pub output_dir: PathBuf,
    /// Cache of compiled packages
    pub compiled_packages: HashMap<String, CompiledPackage>,
    /// Code generator for Luau
    pub code_generator: LuauCodeGenerator,
    /// Transformer for Rust to Luau
    pub transformer: RustToLuauTransformer,
}

impl DependencyManager {
    /// Create a new dependency manager
    pub fn new<P: AsRef<Path>>(root_dir: P, output_dir: P) -> Self {
        Self {
            root_dir: root_dir.as_ref().to_path_buf(),
            output_dir: output_dir.as_ref().to_path_buf(),
            compiled_packages: HashMap::new(),
            code_generator: LuauCodeGenerator::new(),
            transformer: RustToLuauTransformer::new(),
        }
    }

    /// Scan for dependencies in Cargo.toml
    pub fn scan_dependencies(&mut self) -> Result<Vec<DependencyConfig>, String> {
        let cargo_path = self.root_dir.join("Cargo.toml");
        if !cargo_path.exists() {
            return Err("Cargo.toml not found".to_string());
        }

        // Read Cargo.toml
        let cargo_content = fs::read_to_string(cargo_path)
            .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;

        // Parse dependencies
        let cargo_toml: CargoToml = toml::from_str(&cargo_content)
            .map_err(|e| format!("Failed to parse Cargo.toml: {}", e))?;

        // Convert to dependency configs
        let mut configs = Vec::new();
        for (name, dep) in cargo_toml.dependencies.iter() {
            let version = match dep {
                DependencyDetail::Simple(version) => version.clone(),
                DependencyDetail::Complex(detail) => detail.version.clone().unwrap_or_else(|| "*".to_string()),
            };

            let features = match dep {
                DependencyDetail::Simple(_) => Vec::new(),
                DependencyDetail::Complex(detail) => detail.features.clone().unwrap_or_else(Vec::new),
            };

            configs.push(DependencyConfig {
                name: name.clone(),
                version,
                main: PathBuf::from(format!("src/lib.rs")), // Default main file
                types: None,
                repository: None,
                metadata: HashMap::new(),
                features,
            });
        }

        Ok(configs)
    }

    /// Compile a single dependency
    pub fn compile_dependency(&mut self, config: &DependencyConfig) -> Result<CompiledPackage, String> {
        // Check if already compiled
        let package_key = format!("{}@{}", config.name, config.version);
        if let Some(package) = self.compiled_packages.get(&package_key) {
            return Ok(package.clone());
        }

        // Create a temporary directory for the crate
        let temp_dir = self.output_dir.join("temp").join(&config.name);
        fs::create_dir_all(&temp_dir).map_err(|e| format!("Failed to create temp directory: {}", e))?;

        // Download the crate using cargo
        self.download_crate(&config.name, &config.version, &temp_dir)?;

        // Parse the crate's source
        let source_dir = temp_dir.join("src");
        let main_file = source_dir.join(&config.main);
        let source = fs::read_to_string(&main_file)
            .map_err(|e| format!("Failed to read main file: {}", e))?;

        // Transform Rust to Luau
        let program = self.transformer.transform_program(&source)?;

        // Generate Luau code
        let luau_code = self.generate_luau_code(&program)?;

        // Create package
        let package = CompiledPackage {
            name: config.name.clone(),
            version: config.version.clone(),
            code: luau_code,
            types: None, // TODO: Generate type definitions
            dependencies: Vec::new(), // TODO: Extract dependencies
        };

        // Cache the compiled package
        self.compiled_packages.insert(package_key, package.clone());

        // Write output
        self.write_package_output(&package)?;

        Ok(package)
    }

    /// Compile all dependencies
    pub fn compile_all_dependencies(&mut self) -> Result<Vec<CompiledPackage>, String> {
        let configs = self.scan_dependencies()?;
        let mut packages = Vec::new();

        for config in configs {
            let package = self.compile_dependency(&config)?;
            packages.push(package);
        }

        Ok(packages)
    }

    /// Generate a Roblox model file containing all dependencies
    pub fn generate_roblox_model(&self) -> Result<PathBuf, String> {
        let model_path = self.output_dir.join("dependencies.rbxm");
        
        // TODO: Implement model generation logic
        // This would create a Roblox model file (.rbxm) containing all compiled dependencies
        // Each dependency would be a ModuleScript inside the model

        Ok(model_path)
    }

    /// Download a crate using cargo
    fn download_crate(&self, name: &str, version: &str, target_dir: &Path) -> Result<(), String> {
        // In a real implementation, you might use a Rust dependency resolver
        // For now, we'll simulate the download process
        
        // Create a temporary Cargo.toml
        let cargo_content = format!(
            r#"[package]
name = "temp_package"
version = "0.1.0"
edition = "2021"

[dependencies]
{} = "{}""#,
            name, version
        );

        let cargo_path = target_dir.join("Cargo.toml");
        fs::write(&cargo_path, cargo_content)
            .map_err(|e| format!("Failed to write temporary Cargo.toml: {}", e))?;

        // Run cargo fetch to download the dependency
        let status = Command::new("cargo")
            .args(&["fetch", "--manifest-path", cargo_path.to_str().unwrap()])
            .status()
            .map_err(|e| format!("Failed to run cargo fetch: {}", e))?;

        if !status.success() {
            return Err(format!("cargo fetch failed with status: {}", status));
        }

        Ok(())
    }

    /// Generate Luau code from a program
    fn generate_luau_code(&mut self, program: &Program) -> Result<String, String> {
        let result = self.code_generator.generate(&LuauNode::Program(program.clone()))
            .map_err(|e| format!("Failed to generate Luau code: {}", e))?;
        
        Ok(result)
    }

    /// Write a compiled package to the output directory
    fn write_package_output(&self, package: &CompiledPackage) -> Result<(), String> {
        let output_dir = self.output_dir.join("packages").join(&package.name);
        fs::create_dir_all(&output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;

        // Write the Luau module
        let module_path = output_dir.join("init.lua");
        fs::write(&module_path, &package.code)
            .map_err(|e| format!("Failed to write module: {}", e))?;

        // Write module metadata
        let metadata = PackageMetadata {
            name: package.name.clone(),
            version: package.version.clone(),
            dependencies: package.dependencies.clone(),
        };

        let metadata_json = serde_json::to_string_pretty(&metadata)
            .map_err(|e| format!("Failed to serialize metadata: {}", e))?;

        let metadata_path = output_dir.join("package.json");
        fs::write(&metadata_path, metadata_json)
            .map_err(|e| format!("Failed to write metadata: {}", e))?;

        Ok(())
    }
}

/// Cargo.toml structure (simplified)
#[derive(Debug, Deserialize)]
struct CargoToml {
    package: CargoPackage,
    dependencies: HashMap<String, DependencyDetail>,
}

/// Cargo package metadata
#[derive(Debug, Deserialize)]
struct CargoPackage {
    name: String,
    version: String,
}

/// Dependency detail, can be simple (just a version string) or complex (a table)
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum DependencyDetail {
    Simple(String),
    Complex(ComplexDependency),
}

/// Complex dependency with details
#[derive(Debug, Deserialize)]
struct ComplexDependency {
    version: Option<String>,
    path: Option<String>,
    git: Option<String>,
    branch: Option<String>,
    features: Option<Vec<String>>,
}

/// Metadata for a compiled package
#[derive(Debug, Serialize)]
struct PackageMetadata {
    name: String,
    version: String,
    dependencies: Vec<String>,
}
