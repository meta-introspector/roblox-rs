// Place file generator for Roblox-rs
// This module converts Rust code and assets into a Roblox place file (.rbxl)

use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::process::Command;
use std::collections::HashMap;

// Workspace structure for Roblox-rs projects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Workspace {
    Client,
    Shared,
    Server,
}

// Asset types supported in Roblox-rs projects
#[derive(Debug, Clone)]
pub enum AssetType {
    Model,
    Image,
    Audio,
    Animation,
    Mesh,
    Other(String),
}

// Configuration for place file generation
pub struct PlaceGeneratorConfig {
    pub project_path: PathBuf,
    pub output_path: PathBuf,
    pub include_runtime: bool,
    pub include_test_code: bool,
    pub auto_start_server_scripts: bool,
    pub place_version: String,
    pub place_name: String,
}

impl Default for PlaceGeneratorConfig {
    fn default() -> Self {
        PlaceGeneratorConfig {
            project_path: PathBuf::new(),
            output_path: PathBuf::new(),
            include_runtime: true,
            include_test_code: false,
            auto_start_server_scripts: true,
            place_version: "0.1.0".to_string(),
            place_name: "RobloxRS Project".to_string(),
        }
    }
}

// Main place generator struct
pub struct PlaceGenerator {
    config: PlaceGeneratorConfig,
    rust_files: HashMap<Workspace, Vec<PathBuf>>,
    asset_files: Vec<(PathBuf, AssetType)>,
    transpiled_code: HashMap<String, String>,
}

impl PlaceGenerator {
    // Create a new place generator with the given configuration
    pub fn new(config: PlaceGeneratorConfig) -> Self {
        PlaceGenerator {
            config,
            rust_files: HashMap::new(),
            asset_files: Vec::new(),
            transpiled_code: HashMap::new(),
        }
    }

    // Scan project directory to find Rust files and assets
    pub fn scan_project(&mut self) -> io::Result<()> {
        // Initialize workspace maps
        self.rust_files.insert(Workspace::Client, Vec::new());
        self.rust_files.insert(Workspace::Shared, Vec::new());
        self.rust_files.insert(Workspace::Server, Vec::new());
        
        // Scan for Rust files in the client, shared, and server directories
        self.scan_workspace_directory("client", Workspace::Client)?;
        self.scan_workspace_directory("shared", Workspace::Shared)?;
        self.scan_workspace_directory("server", Workspace::Server)?;

        // Scan for assets in the assets directory
        self.scan_assets_directory()?;

        Ok(())
    }

    // Scan a workspace directory for Rust files
    fn scan_workspace_directory(&mut self, dir_name: &str, workspace: Workspace) -> io::Result<()> {
        let workspace_path = self.config.project_path.join(dir_name);
        
        if !workspace_path.exists() {
            // Create the directory if it doesn't exist
            fs::create_dir_all(&workspace_path)?;
            return Ok(());
        }

        // Walk the directory and find all .rs files
        self.find_rust_files(&workspace_path, &workspace)
    }

    // Find all Rust files in a directory and its subdirectories
    fn find_rust_files(&mut self, dir: &Path, workspace: &Workspace) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    self.find_rust_files(&path, workspace)?;
                } else if let Some(extension) = path.extension() {
                    if extension == "rs" {
                        match workspace {
                            Workspace::Client => self.rust_files.get_mut(&Workspace::Client).unwrap().push(path),
                            Workspace::Shared => self.rust_files.get_mut(&Workspace::Shared).unwrap().push(path),
                            Workspace::Server => self.rust_files.get_mut(&Workspace::Server).unwrap().push(path),
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    // Scan the assets directory for asset files
    fn scan_assets_directory(&mut self) -> io::Result<()> {
        let assets_path = self.config.project_path.join("assets");
        
        if !assets_path.exists() {
            // Create the directory if it doesn't exist
            fs::create_dir_all(&assets_path)?;
            return Ok(());
        }

        // Walk the directory and find all asset files
        self.find_asset_files(&assets_path)
    }

    // Find all asset files in a directory and its subdirectories
    fn find_asset_files(&mut self, dir: &Path) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    self.find_asset_files(&path)?;
                } else if let Some(extension) = path.extension() {
                    let asset_type = match extension.to_str().unwrap_or("") {
                        "rbxm" | "rbxmx" => AssetType::Model,
                        "png" | "jpg" | "jpeg" => AssetType::Image,
                        "mp3" | "ogg" => AssetType::Audio,
                        "fbx" => AssetType::Animation,
                        "obj" | "fbx" => AssetType::Mesh,
                        other => AssetType::Other(other.to_string()),
                    };
                    
                    self.asset_files.push((path, asset_type));
                }
            }
        }
        
        Ok(())
    }

    // Transpile all Rust files into Luau code
    pub fn transpile_code(&mut self) -> Result<(), String> {
        // Placeholder for actual transpilation logic
        // In a real implementation, this would call the Rust-to-Luau transpiler
        
        // Mock transpilation for now
        for (workspace, files) in &self.rust_files {
            for file_path in files {
                let file_content = match fs::read_to_string(file_path) {
                    Ok(content) => content,
                    Err(e) => return Err(format!("Failed to read file {}: {}", file_path.display(), e)),
                };
                
                let file_name = file_path.file_name().unwrap().to_str().unwrap();
                let workspace_name = match workspace {
                    Workspace::Client => "client",
                    Workspace::Shared => "shared",
                    Workspace::Server => "server",
                };

                // Mock transpilation - in reality, this would be the actual transpiler
                let transpiled = format!("-- Transpiled from {}\n-- Workspace: {}\n\nlocal module = {{}}\n\n-- Mock transpiled code\nfunction module.init()\n    print(\"Initialized {}\")\nend\n\nreturn module", file_path.display(), workspace_name, file_name);
                
                let output_key = format!("{}/{}", workspace_name, file_name.replace(".rs", ".lua"));
                self.transpiled_code.insert(output_key, transpiled);
            }
        }
        
        Ok(())
    }

    // Generate the runtime library code
    fn generate_runtime(&self) -> String {
        use crate::runtime::generate_runtime_lib;
        
        // Generate the complete runtime library
        format!("-- RobloxRS Runtime Library\n\nRobloxRS = {{}}\n\n{}", generate_runtime_lib())
    }

    // Generate the place file XML structure
    fn generate_place_xml(&self, temp_dir: &Path) -> Result<String, String> {
        // Create XML structure for Roblox place file
        let mut xml = String::new();
        
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<roblox xmlns:xmime=\"http://www.w3.org/2005/05/xmlmime\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"http://www.roblox.com/roblox.xsd\" version=\"4\">\n");
        xml.push_str("\t<External>null</External>\n");
        xml.push_str("\t<External>nil</External>\n");
        
        // Add game services
        xml.push_str("\t<Item class=\"Workspace\" referent=\"RBX0\">\n");
        xml.push_str("\t\t<Properties>\n");
        xml.push_str("\t\t\t<string name=\"Name\">Workspace</string>\n");
        xml.push_str("\t\t</Properties>\n");
        xml.push_str("\t</Item>\n");
        
        // Add ReplicatedStorage
        xml.push_str("\t<Item class=\"ReplicatedStorage\" referent=\"RBX1\">\n");
        xml.push_str("\t\t<Properties>\n");
        xml.push_str("\t\t\t<string name=\"Name\">ReplicatedStorage</string>\n");
        xml.push_str("\t\t</Properties>\n");
        
        // Add Shared Code
        xml.push_str("\t\t<Item class=\"Folder\" referent=\"RBXSHARED\">\n");
        xml.push_str("\t\t\t<Properties>\n");
        xml.push_str("\t\t\t\t<string name=\"Name\">Shared</string>\n");
        xml.push_str("\t\t\t</Properties>\n");
        
        // Add shared scripts
        for (path, content) in &self.transpiled_code {
            if path.starts_with("shared/") {
                let script_name = Path::new(path).file_stem().unwrap().to_str().unwrap();
                
                xml.push_str("\t\t\t<Item class=\"ModuleScript\" referent=\"RBXS\">\n");
                xml.push_str("\t\t\t\t<Properties>\n");
                xml.push_str(&format!("\t\t\t\t\t<string name=\"Name\">{}</string>\n", script_name));
                xml.push_str("\t\t\t\t\t<ProtectedString name=\"Source\"><![CDATA[");
                xml.push_str(content);
                xml.push_str("]]></ProtectedString>\n");
                xml.push_str("\t\t\t\t</Properties>\n");
                xml.push_str("\t\t\t</Item>\n");
            }
        }
        
        // Add Runtime library if needed
        if self.config.include_runtime {
            xml.push_str("\t\t\t<Item class=\"ModuleScript\" referent=\"RBXRUNTIME\">\n");
            xml.push_str("\t\t\t\t<Properties>\n");
            xml.push_str("\t\t\t\t\t<string name=\"Name\">RobloxRS_Runtime</string>\n");
            xml.push_str("\t\t\t\t\t<ProtectedString name=\"Source\"><![CDATA[");
            xml.push_str(&self.generate_runtime());
            xml.push_str("]]></ProtectedString>\n");
            xml.push_str("\t\t\t\t</Properties>\n");
            xml.push_str("\t\t\t</Item>\n");
        }
        
        xml.push_str("\t\t</Item>\n");
        xml.push_str("\t</Item>\n");
        
        // Add ServerScriptService
        xml.push_str("\t<Item class=\"ServerScriptService\" referent=\"RBX2\">\n");
        xml.push_str("\t\t<Properties>\n");
        xml.push_str("\t\t\t<string name=\"Name\">ServerScriptService</string>\n");
        xml.push_str("\t\t</Properties>\n");
        
        // Add Server Code
        xml.push_str("\t\t<Item class=\"Folder\" referent=\"RBXSERVER\">\n");
        xml.push_str("\t\t\t<Properties>\n");
        xml.push_str("\t\t\t\t<string name=\"Name\">Server</string>\n");
        xml.push_str("\t\t\t</Properties>\n");
        
        // Add server scripts
        for (path, content) in &self.transpiled_code {
            if path.starts_with("server/") {
                let script_name = Path::new(path).file_stem().unwrap().to_str().unwrap();
                let script_type = if path.ends_with("_init.lua") { "Script" } else { "ModuleScript" };
                
                xml.push_str(&format!("\t\t\t<Item class=\"{}\" referent=\"RBXS\">\n", script_type));
                xml.push_str("\t\t\t\t<Properties>\n");
                xml.push_str(&format!("\t\t\t\t\t<string name=\"Name\">{}</string>\n", script_name));
                xml.push_str("\t\t\t\t\t<ProtectedString name=\"Source\"><![CDATA[");
                xml.push_str(content);
                xml.push_str("]]></ProtectedString>\n");
                xml.push_str("\t\t\t\t</Properties>\n");
                xml.push_str("\t\t\t</Item>\n");
            }
        }
        
        xml.push_str("\t\t</Item>\n");
        xml.push_str("\t</Item>\n");
        
        // Add StarterPlayer and StarterPlayerScripts
        xml.push_str("\t<Item class=\"StarterPlayer\" referent=\"RBX3\">\n");
        xml.push_str("\t\t<Properties>\n");
        xml.push_str("\t\t\t<string name=\"Name\">StarterPlayer</string>\n");
        xml.push_str("\t\t</Properties>\n");
        
        xml.push_str("\t\t<Item class=\"StarterPlayerScripts\" referent=\"RBX4\">\n");
        xml.push_str("\t\t\t<Properties>\n");
        xml.push_str("\t\t\t\t<string name=\"Name\">StarterPlayerScripts</string>\n");
        xml.push_str("\t\t\t</Properties>\n");
        
        // Add Client Code
        xml.push_str("\t\t\t<Item class=\"Folder\" referent=\"RBXCLIENT\">\n");
        xml.push_str("\t\t\t\t<Properties>\n");
        xml.push_str("\t\t\t\t\t<string name=\"Name\">Client</string>\n");
        xml.push_str("\t\t\t\t</Properties>\n");
        
        // Add client scripts
        for (path, content) in &self.transpiled_code {
            if path.starts_with("client/") {
                let script_name = Path::new(path).file_stem().unwrap().to_str().unwrap();
                let script_type = if path.ends_with("_init.lua") { "LocalScript" } else { "ModuleScript" };
                
                xml.push_str(&format!("\t\t\t\t<Item class=\"{}\" referent=\"RBXS\">\n", script_type));
                xml.push_str("\t\t\t\t\t<Properties>\n");
                xml.push_str(&format!("\t\t\t\t\t\t<string name=\"Name\">{}</string>\n", script_name));
                xml.push_str("\t\t\t\t\t\t<ProtectedString name=\"Source\"><![CDATA[");
                xml.push_str(content);
                xml.push_str("]]></ProtectedString>\n");
                xml.push_str("\t\t\t\t\t</Properties>\n");
                xml.push_str("\t\t\t\t</Item>\n");
            }
        }
        
        xml.push_str("\t\t\t</Item>\n");
        xml.push_str("\t\t</Item>\n");
        xml.push_str("\t</Item>\n");
        
        // Close the file
        xml.push_str("</roblox>");
        
        Ok(xml)
    }

    // Generate the .rbxl file from XML
    fn generate_rbxl_file(&self, xml_path: &Path, output_path: &Path) -> Result<(), String> {
        // In a real implementation, we would use a Roblox XML to binary converter
        // For now, let's just copy the XML file to the output path with a .rbxl extension
        
        // Placeholder - in reality, we'd convert XML to binary RBXL format
        fs::copy(xml_path, output_path).map_err(|e| format!("Failed to create place file: {}", e))?;
        
        Ok(())
    }

    // Generate the Roblox place file
    pub fn generate_place_file(&self) -> Result<PathBuf, String> {
        // Create a temporary directory to store the Lua scripts and XML
        let temp_dir = tempfile::tempdir().map_err(|e| format!("Failed to create temporary directory: {}", e))?;
        
        // Create the directory structure
        let client_dir = temp_dir.path().join("client");
        let shared_dir = temp_dir.path().join("shared");
        let server_dir = temp_dir.path().join("server");
        
        fs::create_dir_all(&client_dir).map_err(|e| format!("Failed to create client directory: {}", e))?;
        fs::create_dir_all(&shared_dir).map_err(|e| format!("Failed to create shared directory: {}", e))?;
        fs::create_dir_all(&server_dir).map_err(|e| format!("Failed to create server directory: {}", e))?;
        
        // Write the transpiled Lua scripts to the temp directory
        for (path, content) in &self.transpiled_code {
            let full_path = temp_dir.path().join(path);
            
            // Create parent directories if needed
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
            }
            
            fs::write(&full_path, content).map_err(|e| format!("Failed to write file {}: {}", full_path.display(), e))?;
        }
        
        // Write the runtime library to the shared directory
        if self.config.include_runtime {
            let runtime_path = shared_dir.join("RobloxRS_Runtime.lua");
            fs::write(&runtime_path, self.generate_runtime()).map_err(|e| format!("Failed to write runtime: {}", e))?;
        }
        
        // Generate the place file XML
        let xml_content = self.generate_place_xml(temp_dir.path())?;
        let xml_path = temp_dir.path().join("place.rbxlx");
        fs::write(&xml_path, xml_content).map_err(|e| format!("Failed to write XML: {}", e))?;
        
        // Determine output path
        let output_path = if self.config.output_path.is_absolute() {
            self.config.output_path.clone()
        } else {
            self.config.project_path.join(&self.config.output_path)
        };

        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create output directory: {}", e))?;
        }
        
        // Generate the binary .rbxl file
        let rbxl_path = output_path.with_extension("rbxl");
        self.generate_rbxl_file(&xml_path, &rbxl_path)?;
        
        // Also save the XML version for debugging/viewing
        let rbxlx_path = output_path.with_extension("rbxlx");
        fs::copy(xml_path, &rbxlx_path).map_err(|e| format!("Failed to save XML version: {}", e))?;
        
        // Return the path to the generated place file
        Ok(rbxl_path)
    }

    // Get workspace statistics
    pub fn get_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        
        stats.insert("client_files".to_string(), self.rust_files.get(&Workspace::Client).map_or(0, |files| files.len()));
        stats.insert("shared_files".to_string(), self.rust_files.get(&Workspace::Shared).map_or(0, |files| files.len()));
        stats.insert("server_files".to_string(), self.rust_files.get(&Workspace::Server).map_or(0, |files| files.len()));
        stats.insert("asset_files".to_string(), self.asset_files.len());
        stats.insert("transpiled_modules".to_string(), self.transpiled_code.len());
        
        stats
    }
}

// Helper function to copy a directory recursively
fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}
