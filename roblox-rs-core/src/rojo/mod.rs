use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use serde::{Deserialize, Serialize};

/// Represents a Rojo project file
#[derive(Debug, Serialize, Deserialize)]
pub struct RojoProject {
    /// Project name
    pub name: Option<String>,
    /// Tree of files and directories
    pub tree: HashMap<String, RojoNode>,
    /// Serve settings
    #[serde(rename = "servePort")]
    pub serve_port: Option<u16>,
    /// Rojo file format version
    #[serde(default = "default_rojo_version")]
    pub rojo_version: String,
}

/// Default Rojo file format version
fn default_rojo_version() -> String {
    "7.0.0".to_string()
}

/// Represents a single node in the Rojo project tree
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$className")]
pub enum RojoNode {
    /// DataModel and children
    #[serde(rename = "DataModel")]
    DataModel {
        #[serde(rename = "$path")]
        path: Option<String>,
        #[serde(rename = "$properties")]
        properties: Option<HashMap<String, serde_json::Value>>,
        #[serde(flatten)]
        children: HashMap<String, RojoNode>,
    },
    /// Folder and children
    #[serde(rename = "Folder")]
    Folder {
        #[serde(rename = "$path")]
        path: Option<String>,
        #[serde(rename = "$properties")]
        properties: Option<HashMap<String, serde_json::Value>>,
        #[serde(flatten)]
        children: HashMap<String, RojoNode>,
    },
    /// Script (server code)
    #[serde(rename = "Script")]
    Script {
        #[serde(rename = "$path")]
        path: Option<String>,
        #[serde(rename = "$properties")]
        properties: Option<HashMap<String, serde_json::Value>>,
    },
    /// LocalScript (client code)
    #[serde(rename = "LocalScript")]
    LocalScript {
        #[serde(rename = "$path")]
        path: Option<String>,
        #[serde(rename = "$properties")]
        properties: Option<HashMap<String, serde_json::Value>>,
    },
    /// ModuleScript (module code)
    #[serde(rename = "ModuleScript")]
    ModuleScript {
        #[serde(rename = "$path")]
        path: Option<String>,
        #[serde(rename = "$properties")]
        properties: Option<HashMap<String, serde_json::Value>>,
        #[serde(flatten)]
        children: Option<HashMap<String, RojoNode>>,
    },
    /// Other Roblox Instance types represented as generic instance
    #[serde(rename = "Instance")]
    Instance {
        #[serde(rename = "$path")]
        path: Option<String>,
        #[serde(rename = "$properties")]
        properties: Option<HashMap<String, serde_json::Value>>,
        #[serde(flatten)]
        children: Option<HashMap<String, RojoNode>>,
    },
    
    /// Fallback for any unrecognized instance type
    #[serde(other)]
    Other,
}

/// Manager for Rojo project files and serving
pub struct RojoManager {
    /// Path to the project file
    project_path: PathBuf,
    /// Parsed project data
    project: Option<RojoProject>,
    /// Map of Rust source files to Luau output destinations
    source_mapping: HashMap<PathBuf, PathBuf>,
    /// Currently running Rojo serve process
    serve_process: Option<Child>,
}

impl RojoManager {
    /// Create a new Rojo manager for a project file
    pub fn new(project_path: &Path) -> Self {
        Self {
            project_path: project_path.to_path_buf(),
            project: None,
            source_mapping: HashMap::new(),
            serve_process: None,
        }
    }

    /// Load and parse a Rojo project file
    pub fn load_project(&mut self) -> Result<&RojoProject, String> {
        let content = fs::read_to_string(&self.project_path)
            .map_err(|e| format!("Failed to read Rojo project file: {}", e))?;

        let project: RojoProject = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse Rojo project file: {}", e))?;

        self.project = Some(project);
        self.build_source_mapping();

        Ok(self.project.as_ref().unwrap())
    }

    /// Create a new default Rojo project file
    pub fn create_default_project(&mut self, project_name: &str, src_dir: &Path) -> Result<(), String> {
        let mut project = RojoProject {
            name: Some(project_name.to_string()),
            tree: HashMap::new(),
            serve_port: Some(34872),
            rojo_version: "7.0.0".to_string(),
        };

        // Create source directory structure
        let server_scripts_path = src_dir.join("server");
        let client_scripts_path = src_dir.join("client");
        let shared_modules_path = src_dir.join("shared");

        // Create basic structure - can be customized
        let mut tree = HashMap::new();
        
        // Add server scripts
        tree.insert("ServerScriptService".to_string(), RojoNode::Folder {
            path: Some(server_scripts_path.to_string_lossy().to_string()),
            properties: None,
            children: HashMap::new(),
        });

        // Add client scripts
        tree.insert("StarterPlayerScripts".to_string(), RojoNode::Folder {
            path: Some(client_scripts_path.to_string_lossy().to_string()),
            properties: None,
            children: HashMap::new(),
        });

        // Add shared modules
        tree.insert("ReplicatedStorage".to_string(), RojoNode::Folder {
            path: Some(shared_modules_path.to_string_lossy().to_string()),
            properties: None,
            children: HashMap::new(),
        });

        project.tree = tree;

        // Write the project file
        let json = serde_json::to_string_pretty(&project)
            .map_err(|e| format!("Failed to serialize project: {}", e))?;

        fs::write(&self.project_path, json)
            .map_err(|e| format!("Failed to write project file: {}", e))?;

        self.project = Some(project);
        Ok(())
    }

    /// Build mapping of Rust source files to their compiled Luau destinations
    fn build_source_mapping(&mut self) {
        self.source_mapping.clear();
        
        if let Some(project) = &self.project {
            // Create a separate collection of nodes to process to avoid borrowing conflicts
            let mut nodes_to_process = Vec::new();
            for (name, node) in &project.tree {
                nodes_to_process.push((node.clone(), PathBuf::new()));
            }
            
            // Process each node without borrowing self.project
            for (node, base_path) in nodes_to_process {
                self.process_node_mapping(&node, base_path);
            }
        }
    }

    /// Process a node in the Rojo tree to build source mapping
    fn process_node_mapping(&mut self, node: &RojoNode, base_path: PathBuf) {
        match node {
            RojoNode::Folder { path, children, .. } | 
            RojoNode::DataModel { path, children, .. } => {
                // Process all children
                let path_prefix = path.as_ref().map_or(base_path.clone(), |p| base_path.clone());
                
                // Collect children to avoid borrowing issues
                let children_to_process: Vec<_> = children.iter()
                    .map(|(name, child)| (name.clone(), child.clone()))
                    .collect();
                
                for (name, child_node) in children_to_process {
                    let new_base = path_prefix.join(&name);
                    self.process_node_mapping(&child_node, new_base);
                }
            },
            RojoNode::Script { path, .. } | 
            RojoNode::LocalScript { path, .. } | 
            RojoNode::ModuleScript { path, .. } => {
                if let Some(path_str) = path {
                    let source_path = PathBuf::from(path_str);
                    
                    // If this is a .rs file, map it to the expected .lua output
                    if source_path.extension().map_or(false, |ext| ext == "rs") {
                        let mut lua_path = source_path.clone();
                        lua_path.set_extension("lua");
                        
                        // Store mapping from source to destination
                        self.source_mapping.insert(source_path, lua_path);
                    }
                }
            },
            _ => {}
        }
    }

    /// Get the output path for a given source file
    pub fn get_output_path(&self, source_path: &Path) -> Option<&PathBuf> {
        self.source_mapping.get(source_path)
    }

    /// Serve the project using rojo
    pub fn serve(&mut self) -> Result<u16, String> {
        if self.serve_process.is_some() {
            return Err("Rojo is already serving this project".to_string());
        }

        // Find serve port
        let port = if let Some(project) = &self.project {
            project.serve_port.unwrap_or(34872)
        } else {
            34872 // Default port
        };

        // Start rojo serve process
        let project_dir = self.project_path.parent().unwrap_or(Path::new("."));
        
        match Command::new("rojo")
            .arg("serve")
            .arg(&self.project_path)
            .current_dir(project_dir)
            .spawn() {
                Ok(child) => {
                    self.serve_process = Some(child);
                    Ok(port)
                },
                Err(e) => Err(format!("Failed to start rojo serve: {}", e))
            }
    }

    /// Stop the currently running rojo serve process
    pub fn stop_serve(&mut self) -> Result<(), String> {
        if let Some(mut child) = self.serve_process.take() {
            match child.kill() {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to stop rojo serve: {}", e))
            }
        } else {
            Ok(()) // No process running
        }
    }
}

impl Drop for RojoManager {
    fn drop(&mut self) {
        // Ensure we kill the serve process when the manager is dropped
        if let Some(mut child) = self.serve_process.take() {
            let _ = child.kill();
        }
    }
}
