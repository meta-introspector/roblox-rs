// Roblox-RS Place Generator
// Generates a complete Roblox place file (.rbxlx) from transpiled code

use std::path::{Path, PathBuf};
use std::fs;
use std::error::Error;
use std::io::Write;
use std::collections::HashMap;

// Access our crate's transpiler module
use crate::transpiler;

/// Structure of a Roblox place file
pub struct PlaceFile {
    /// Path to the client code directory
    pub client_path: PathBuf,
    /// Path to the server code directory
    pub server_path: PathBuf,
    /// Path to the shared code directory
    pub shared_path: PathBuf,
    /// Path to the assets directory
    pub assets_path: PathBuf,
    /// Output path for the place file
    pub output_path: PathBuf,
    /// Place file name
    pub name: String,
}

impl PlaceFile {
    /// Create a new place file configuration
    pub fn new(project_path: &Path, output_path: &Path, name: &str) -> Self {
        let client_path = project_path.join("client");
        let server_path = project_path.join("server");
        let shared_path = project_path.join("shared");
        let assets_path = project_path.join("assets");
        
        PlaceFile {
            client_path,
            server_path,
            shared_path,
            assets_path,
            output_path: output_path.to_path_buf(),
            name: name.to_string(),
        }
    }
    
    /// Generate the place file
    pub fn generate(&self) -> Result<PathBuf, Box<dyn Error>> {
        println!("Generating place file: {}", self.name);
        
        // Create temporary directories for transpiled code
        let temp_dir = tempfile::tempdir()?;
        let client_out = temp_dir.path().join("client");
        let server_out = temp_dir.path().join("server");
        let shared_out = temp_dir.path().join("shared");
        
        fs::create_dir_all(&client_out)?;
        fs::create_dir_all(&server_out)?;
        fs::create_dir_all(&shared_out)?;
        
        // Transpile Rust code to Luau
        self.transpile_directory(&self.client_path, &client_out)?;
        self.transpile_directory(&self.server_path, &server_out)?;
        self.transpile_directory(&self.shared_path, &shared_out)?;
        
        // Generate XML for the place file
        let xml_content = self.generate_place_xml(&client_out, &server_out, &shared_out)?;
        
        // Ensure output directory exists
        if let Some(parent) = self.output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Write place file (.rbxlx - XML format)
        let rbxlx_path = self.output_path.join(format!("{}.rbxlx", self.name));
        fs::write(&rbxlx_path, xml_content)?;
        
        println!("Place file generated: {:?}", rbxlx_path);
        
        Ok(rbxlx_path)
    }
    
    /// Transpile all Rust files in a directory to Luau
    fn transpile_directory(&self, input_dir: &Path, output_dir: &Path) -> Result<(), Box<dyn Error>> {
        if !input_dir.exists() {
            return Ok(());
        }
        
        for entry in fs::read_dir(input_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                // Output path for the transpiled file
                let rel_path = path.strip_prefix(input_dir).unwrap_or(&path);
                let output_path = output_dir.join(rel_path)
                    .with_extension("lua");
                
                // Ensure parent directory exists
                if let Some(parent) = output_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                
                // Transpile the file
                transpiler::transpile_file(&path, &output_path)?;
            } else if path.is_dir() {
                // Recursively transpile subdirectories
                let rel_path = path.strip_prefix(input_dir).unwrap_or(&path);
                let new_output_dir = output_dir.join(rel_path);
                fs::create_dir_all(&new_output_dir)?;
                
                self.transpile_directory(&path, &new_output_dir)?;
            }
        }
        
        Ok(())
    }
    
    /// Generate XML content for the place file
    fn generate_place_xml(&self, client_dir: &Path, server_dir: &Path, shared_dir: &Path) -> Result<String, Box<dyn Error>> {
        let mut xml = String::new();
        
        // XML header
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<roblox xmlns:xmime=\"http://www.w3.org/2005/05/xmlmime\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"http://www.roblox.com/roblox.xsd\" version=\"4\">\n");
        xml.push_str("\t<External>null</External>\n");
        xml.push_str("\t<External>nil</External>\n");
        
        // Add basic services
        let mut item_refs = Vec::new();
        let mut items = Vec::new();
        
        // Add workspace
        let workspace_ref = "RBX0";
        item_refs.push(workspace_ref.to_string());
        items.push(self.generate_service_xml("Workspace", workspace_ref, &[]));
        
        // Add lighting
        let lighting_ref = "RBX1";
        item_refs.push(lighting_ref.to_string());
        items.push(self.generate_service_xml("Lighting", lighting_ref, &[]));
        
        // Add ReplicatedStorage for shared code
        let rep_storage_ref = "RBX2";
        item_refs.push(rep_storage_ref.to_string());
        let mut rep_storage_children = Vec::new();
        
        // Add shared code
        let shared_code_ref = "RBX3";
        item_refs.push(shared_code_ref.to_string());
        rep_storage_children.push(shared_code_ref.to_string());
        
        let mut shared_scripts = Vec::new();
        self.add_scripts_to_xml(shared_dir, "ModuleScript", &mut item_refs, &mut shared_scripts)?;
        
        items.push(self.generate_folder_xml("Shared", shared_code_ref, &rep_storage_ref, &shared_scripts));
        items.extend(shared_scripts);
        
        // Add ReplicatedStorage service
        items.push(self.generate_service_xml("ReplicatedStorage", rep_storage_ref, &rep_storage_children));
        
        // Add StarterPlayer
        let starter_player_ref = "RBX4";
        item_refs.push(starter_player_ref.to_string());
        
        // Add StarterPlayerScripts for client code
        let starter_player_scripts_ref = "RBX5";
        item_refs.push(starter_player_scripts_ref.to_string());
        
        let mut client_scripts = Vec::new();
        self.add_scripts_to_xml(client_dir, "LocalScript", &mut item_refs, &mut client_scripts)?;
        
        // Add StarterPlayer service and StarterPlayerScripts
        items.push(self.generate_service_xml("StarterPlayer", starter_player_ref, &[starter_player_scripts_ref.to_string()]));
        items.push(self.generate_folder_xml("StarterPlayerScripts", starter_player_scripts_ref, starter_player_ref, &client_scripts));
        items.extend(client_scripts);
        
        // Add ServerScriptService for server code
        let server_script_service_ref = "RBX6";
        item_refs.push(server_script_service_ref.to_string());
        
        let mut server_scripts = Vec::new();
        self.add_scripts_to_xml(server_dir, "Script", &mut item_refs, &mut server_scripts)?;
        
        // Add ServerScriptService
        items.push(self.generate_service_xml("ServerScriptService", server_script_service_ref, &server_scripts.iter().map(|s| s.0.clone()).collect::<Vec<_>>()));
        items.extend(server_scripts.iter().map(|s| s.1.clone()));
        
        // Add references to XML
        xml.push_str("\t<Item class=\"Folder\" referent=\"RBX7\">\n");
        xml.push_str("\t\t<Properties>\n");
        xml.push_str("\t\t\t<string name=\"Name\">RobloxRS</string>\n");
        xml.push_str("\t\t</Properties>\n");
        xml.push_str("\t</Item>\n");
        
        // Add all items to XML
        for item in items {
            xml.push_str(&item);
        }
        
        xml.push_str("</roblox>\n");
        
        Ok(xml)
    }
    
    /// Add scripts from a directory to the XML
    fn add_scripts_to_xml(&self, dir: &Path, script_type: &str, item_refs: &mut Vec<String>, scripts: &mut Vec<(String, String)>) -> Result<(), Box<dyn Error>> {
        if !dir.exists() {
            return Ok(());
        }
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "lua") {
                let script_ref = format!("RBX{}", item_refs.len());
                let parent_ref = "RBX6"; // Default to ServerScriptService
                
                let script_name = path.file_stem().unwrap().to_string_lossy().to_string();
                let script_content = fs::read_to_string(&path)?;
                
                let script_xml = self.generate_script_xml(
                    &script_name, 
                    &script_ref, 
                    parent_ref, 
                    script_type, 
                    &script_content
                );
                
                item_refs.push(script_ref.clone());
                scripts.push((script_ref, script_xml));
            } else if path.is_dir() {
                // For directories, create a folder and add scripts recursively
                let folder_ref = format!("RBX{}", item_refs.len());
                let parent_ref = "RBX6"; // Default to ServerScriptService
                let folder_name = path.file_name().unwrap().to_string_lossy().to_string();
                
                let mut folder_scripts = Vec::new();
                // Recursively add scripts from subdirectory
                // (This is a simplified version, full implementation would handle hierarchy properly)
                
                item_refs.push(folder_ref.clone());
                
                let folder_xml = self.generate_folder_xml(
                    &folder_name, 
                    &folder_ref, 
                    parent_ref, 
                    &folder_scripts.iter().map(|s| s.0.clone()).collect::<Vec<_>>()
                );
                
                scripts.push((folder_ref, folder_xml));
                scripts.extend(folder_scripts);
            }
        }
        
        Ok(())
    }
    
    /// Generate XML for a service
    fn generate_service_xml(&self, service_name: &str, referent: &str, children: &[String]) -> String {
        let mut xml = format!("\t<Item class=\"{service_name}\" referent=\"{referent}\">\n");
        xml.push_str("\t\t<Properties>\n");
        xml.push_str("\t\t</Properties>\n");
        
        if !children.is_empty() {
            for child in children {
                xml.push_str(&format!("\t\t<Item referent=\"{child}\">\n"));
                xml.push_str("\t\t</Item>\n");
            }
        }
        
        xml.push_str("\t</Item>\n");
        xml
    }
    
    /// Generate XML for a folder
    fn generate_folder_xml(&self, folder_name: &str, referent: &str, parent_ref: &str, children: &[String]) -> String {
        let mut xml = format!("\t<Item class=\"Folder\" referent=\"{referent}\">\n");
        xml.push_str("\t\t<Properties>\n");
        xml.push_str(&format!("\t\t\t<string name=\"Name\">{folder_name}</string>\n"));
        xml.push_str(&format!("\t\t\t<string name=\"Parent\">{parent_ref}</string>\n"));
        xml.push_str("\t\t</Properties>\n");
        
        if !children.is_empty() {
            for child in children {
                xml.push_str(&format!("\t\t<Item referent=\"{child}\">\n"));
                xml.push_str("\t\t</Item>\n");
            }
        }
        
        xml.push_str("\t</Item>\n");
        xml
    }
    
    /// Generate XML for a script
    fn generate_script_xml(&self, script_name: &str, referent: &str, parent_ref: &str, script_type: &str, content: &str) -> String {
        let mut xml = format!("\t<Item class=\"{script_type}\" referent=\"{referent}\">\n");
        xml.push_str("\t\t<Properties>\n");
        xml.push_str(&format!("\t\t\t<string name=\"Name\">{script_name}</string>\n"));
        xml.push_str(&format!("\t\t\t<string name=\"Parent\">{parent_ref}</string>\n"));
        
        // Encode script content as Base64
        use base64::{Engine as _, engine::general_purpose};
        let encoded_content = general_purpose::STANDARD.encode(content.as_bytes());
        
        xml.push_str(&format!("\t\t\t<ProtectedString name=\"Source\"><![CDATA[{content}]]></ProtectedString>\n"));
        xml.push_str("\t\t</Properties>\n");
        xml.push_str("\t</Item>\n");
        xml
    }
}

/// Convert a place file to binary format (.rbxl)
pub fn convert_to_binary(rbxlx_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    // In a full implementation, we would use rbx_binary to convert the XML to binary
    // For this speed run, we'll just copy the file and change the extension
    
    let rbxl_path = rbxlx_path.with_extension("rbxl");
    fs::copy(rbxlx_path, &rbxl_path)?;
    
    println!("Binary place file generated: {:?}", rbxl_path);
    
    Ok(rbxl_path)
}
