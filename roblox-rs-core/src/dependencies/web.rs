// Web interface for compiling and bundling Rust dependencies for Roblox
// Similar to how roblox-ts handles npm packages in the web

use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::{DependencyManager, CompiledPackage};

/// Web configuration for the dependency bundler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebBundlerConfig {
    /// Project name
    pub project_name: String,
    /// Output format (rbxm, lua)
    pub output_format: String,
    /// Whether to include documentation
    pub include_docs: bool,
    /// Whether to minify output
    pub minify: bool,
    /// Custom Roblox path mapping
    pub path_mapping: Option<HashMap<String, String>>,
}

/// Result of bundling dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleResult {
    /// Success status
    pub success: bool,
    /// Error message if any
    pub error: Option<String>,
    /// Output files
    pub files: Vec<OutputFile>,
    /// Metrics about the bundling process
    pub metrics: BundleMetrics,
}

/// Bundling metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleMetrics {
    /// Number of packages processed
    pub package_count: usize,
    /// Total lines of Rust code processed
    pub rust_lines: usize,
    /// Total lines of Luau code generated
    pub luau_lines: usize,
    /// Time taken in milliseconds
    pub time_ms: u64,
}

/// Output file from bundling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputFile {
    /// File name
    pub name: String,
    /// File content
    pub content: String,
    /// File type
    pub file_type: String,
}

/// Web bundler for Roblox-RS
#[wasm_bindgen]
pub struct RobloxRsWebBundler {
    config: WebBundlerConfig,
    dependency_manager: Option<DependencyManager>,
    output_dir: PathBuf,
    compiled_packages: Vec<CompiledPackage>,
}

#[wasm_bindgen]
impl RobloxRsWebBundler {
    /// Create a new web bundler
    #[wasm_bindgen(constructor)]
    pub fn new(config_json: &str) -> Result<RobloxRsWebBundler, JsValue> {
        let config: WebBundlerConfig = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse config: {}", e)))?;
        
        let output_dir = PathBuf::from("./output");
        
        Ok(RobloxRsWebBundler {
            config,
            dependency_manager: None,
            output_dir,
            compiled_packages: Vec::new(),
        })
    }
    
    /// Initialize the bundler
    #[wasm_bindgen]
    pub fn initialize(&mut self, root_dir: &str) -> Result<(), JsValue> {
        let root_path = PathBuf::from(root_dir);
        let output_path = root_path.join("output");
        
        // Create output directory
        fs::create_dir_all(&output_path)
            .map_err(|e| JsValue::from_str(&format!("Failed to create output directory: {}", e)))?;
        
        self.output_dir = output_path;
        self.dependency_manager = Some(DependencyManager::new(&root_path, &self.output_dir));
        
        Ok(())
    }
    
    /// Bundle all dependencies
    #[wasm_bindgen]
    pub fn bundle_dependencies(&mut self) -> Result<String, JsValue> {
        let start_time = js_sys::Date::now();
        
        let dependency_manager = self.dependency_manager.as_mut()
            .ok_or_else(|| JsValue::from_str("Dependency manager not initialized"))?;
        
        // Compile all dependencies
        let packages = dependency_manager.compile_all_dependencies()
            .map_err(|e| JsValue::from_str(&format!("Failed to compile dependencies: {}", e)))?;
        
        self.compiled_packages = packages.clone();
        
        // Generate output files
        let output_files = self.generate_output_files(&packages)
            .map_err(|e| JsValue::from_str(&format!("Failed to generate output files: {}", e)))?;
        
        // Calculate metrics
        let end_time = js_sys::Date::now();
        let rust_lines = packages.iter()
            .map(|p| p.code.lines().count())
            .sum();
        let luau_lines = output_files.iter()
            .filter(|f| f.file_type == "lua")
            .map(|f| f.content.lines().count())
            .sum();
        
        let metrics = BundleMetrics {
            package_count: packages.len(),
            rust_lines,
            luau_lines,
            time_ms: (end_time - start_time) as u64,
        };
        
        // Create result
        let result = BundleResult {
            success: true,
            error: None,
            files: output_files,
            metrics,
        };
        
        // Serialize result to JSON
        let result_json = serde_json::to_string(&result)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))?;
        
        Ok(result_json)
    }
    
    /// Generate output files
    fn generate_output_files(&self, packages: &[CompiledPackage]) -> Result<Vec<OutputFile>, String> {
        let mut output_files = Vec::new();
        
        match self.config.output_format.as_str() {
            "rbxm" => {
                // Generate RBXM model file
                // For now, we'll just create a placeholder
                output_files.push(OutputFile {
                    name: format!("{}.rbxm", self.config.project_name),
                    content: "RBXM content would go here".to_string(),
                    file_type: "rbxm".to_string(),
                });
            },
            "lua" => {
                // Generate individual Lua files
                for package in packages {
                    let file_name = format!("{}.lua", package.name);
                    let content = if self.config.minify {
                        self.minify_lua(&package.code)
                    } else {
                        package.code.clone()
                    };
                    
                    output_files.push(OutputFile {
                        name: file_name,
                        content,
                        file_type: "lua".to_string(),
                    });
                }
                
                // Generate index file
                let index_content = self.generate_index_file(packages)?;
                output_files.push(OutputFile {
                    name: "index.lua".to_string(),
                    content: index_content,
                    file_type: "lua".to_string(),
                });
            },
            _ => {
                return Err(format!("Unsupported output format: {}", self.config.output_format));
            }
        }
        
        // Generate types file if requested
        if self.config.include_docs {
            let types_content = self.generate_types_file(packages)?;
            output_files.push(OutputFile {
                name: "types.d.lua".to_string(),
                content: types_content,
                file_type: "lua".to_string(),
            });
        }
        
        Ok(output_files)
    }
    
    /// Generate an index file that exports all packages
    fn generate_index_file(&self, packages: &[CompiledPackage]) -> Result<String, String> {
        let mut content = String::new();
        
        content.push_str("-- Generated by Roblox-RS Web Bundler\n");
        content.push_str("-- https://github.com/your-username/roblox-rs\n\n");
        
        content.push_str("local RobloxRs = {}\n\n");
        
        for package in packages {
            content.push_str(&format!("RobloxRs.{} = require(script.{})\n", package.name, package.name));
        }
        
        content.push_str("\nreturn RobloxRs\n");
        
        Ok(content)
    }
    
    /// Generate a types file for all packages
    fn generate_types_file(&self, packages: &[CompiledPackage]) -> Result<String, String> {
        let mut content = String::new();
        
        content.push_str("-- Type definitions for Roblox-RS dependencies\n");
        content.push_str("-- Generated by Roblox-RS Web Bundler\n\n");
        
        content.push_str("export type RobloxRs = {\n");
        
        for package in packages {
            content.push_str(&format!("    {}: any,\n", package.name));
            
            // Include type definitions if available
            if let Some(types) = &package.types {
                content.push_str(&format!("    -- {} types\n", package.name));
                content.push_str(types);
                content.push_str("\n\n");
            }
        }
        
        content.push_str("}\n\n");
        content.push_str("return nil\n");
        
        Ok(content)
    }
    
    /// Minify Lua code
    fn minify_lua(&self, code: &str) -> String {
        // Simple minification: remove comments and extra whitespace
        // A real implementation would use a proper Lua minifier
        let mut minified = String::new();
        
        for line in code.lines() {
            let trimmed = match line.find("--") {
                Some(idx) => &line[..idx],
                None => line,
            }.trim();
            
            if !trimmed.is_empty() {
                minified.push_str(trimmed);
                minified.push(' ');
            }
        }
        
        minified
    }
    
    /// Get bundle status as JSON
    #[wasm_bindgen]
    pub fn get_status(&self) -> String {
        let status = BundleStatus {
            packages: self.compiled_packages.len(),
            config: self.config.clone(),
        };
        
        serde_json::to_string(&status).unwrap_or_else(|_| "{}".to_string())
    }
}

/// Bundle status
#[derive(Debug, Serialize, Deserialize)]
struct BundleStatus {
    packages: usize,
    config: WebBundlerConfig,
}

/// Map type for path mappings
type HashMap<K, V> = std::collections::HashMap<K, V>;
