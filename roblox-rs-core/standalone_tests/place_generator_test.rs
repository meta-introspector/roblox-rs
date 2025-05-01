// Roblox-RS Place Generator Test
// Tests the functionality of our place file generator with the instance library

use std::fs;
use std::path::PathBuf;
use std::env;

// Import modules from roblox-rs-core
use roblox_rs_core::packaging::{PlaceGenerator, PlaceGeneratorConfig, Workspace};
use roblox_rs_core::runtime;

// Mock project directory structure for testing
fn setup_mock_project() -> PathBuf {
    // Create a temporary directory for our test project
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let project_dir = temp_dir.path().to_path_buf();
    
    // Create the workspace structure
    fs::create_dir_all(project_dir.join("client")).expect("Failed to create client dir");
    fs::create_dir_all(project_dir.join("shared")).expect("Failed to create shared dir");
    fs::create_dir_all(project_dir.join("server")).expect("Failed to create server dir");
    fs::create_dir_all(project_dir.join("assets")).expect("Failed to create assets dir");
    
    // Create some rust files in each workspace
    let client_code = r#"
    // Simple Rust file for testing client code
    
    pub fn init_client() {
        println!("Client initialized");
    }
    
    pub fn interact_with_ui() {
        // This will be transpiled to use RobloxRS.Instance API
        let button = create_button("Start Game", 100, 50);
        button.on_click(|| {
            println!("Game started!");
        });
    }
    
    fn create_button(text: &str, x: i32, y: i32) -> Button {
        let button = Button::new();
        button.set_text(text);
        button.set_position(x, y);
        button
    }
    
    struct Button {
        text: String,
        x: i32,
        y: i32,
    }
    
    impl Button {
        fn new() -> Self {
            Button {
                text: "".to_string(),
                x: 0,
                y: 0,
            }
        }
        
        fn set_text(&mut self, text: &str) {
            self.text = text.to_string();
        }
        
        fn set_position(&mut self, x: i32, y: i32) {
            self.x = x;
            self.y = y;
        }
        
        fn on_click<F>(&self, callback: F) where F: Fn() {
            // Mock implementation
        }
    }
    "#;
    
    let shared_code = r#"
    // Simple Rust file for testing shared code
    
    pub struct GameData {
        pub level: i32,
        pub score: i32,
    }
    
    impl GameData {
        pub fn new() -> Self {
            GameData {
                level: 1,
                score: 0,
            }
        }
        
        pub fn increment_score(&mut self, amount: i32) {
            self.score += amount;
            if self.score >= 100 * self.level {
                self.level += 1;
                println!("Level up! Now at level {}", self.level);
            }
        }
    }
    "#;
    
    let server_code = r#"
    // Simple Rust file for testing server code
    use crate::actors::ActorSystem;
    
    pub fn init_server() {
        println!("Server initialized");
        
        // Create a folder in workspace using our Instance API
        let folder = create_workspace_folder("GameObjects");
        
        // Start the actor system
        let system = ActorSystem::new();
        let game_manager = system.spawn(GameManager::new());
        
        // Send a message to the game manager
        game_manager.send("start_game");
    }
    
    fn create_workspace_folder(name: &str) -> Folder {
        let folder = Folder::new();
        folder.set_name(name);
        folder.set_parent(Workspace);
        folder
    }
    
    struct Folder {
        name: String,
        parent: Option<String>,
    }
    
    impl Folder {
        fn new() -> Self {
            Folder {
                name: "".to_string(),
                parent: None,
            }
        }
        
        fn set_name(&mut self, name: &str) {
            self.name = name.to_string();
        }
        
        fn set_parent(&mut self, parent: impl Into<String>) {
            self.parent = Some(parent.into());
        }
    }
    
    struct Workspace;
    
    impl Into<String> for Workspace {
        fn into(self) -> String {
            "Workspace".to_string()
        }
    }
    
    struct GameManager {
        players: Vec<String>,
    }
    
    impl GameManager {
        fn new() -> Self {
            GameManager {
                players: Vec::new(),
            }
        }
    }
    "#;
    
    // Write the Rust files to the directories
    fs::write(project_dir.join("client/client_main.rs"), client_code).expect("Failed to write client code");
    fs::write(project_dir.join("shared/game_data.rs"), shared_code).expect("Failed to write shared code");
    fs::write(project_dir.join("server/server_main.rs"), server_code).expect("Failed to write server code");
    
    // Create a simple asset file (mock .rbxm)
    fs::write(project_dir.join("assets/test_model.rbxm"), "Mock RBXM content").expect("Failed to write asset file");
    
    // Return the project directory path (will be cleaned up automatically when temp_dir is dropped)
    println!("Mock project created at: {}", project_dir.display());
    project_dir
}

/// Main test function for the place generator
pub fn test_place_generator() -> bool {
    println!("Testing place file generator...");
    
    // Set up a mock project
    let project_dir = setup_mock_project();
    
    // Create output directory
    let output_dir = project_dir.join("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    
    // Configure the place generator
    let config = PlaceGeneratorConfig {
        project_path: project_dir.clone(),
        output_path: output_dir.join("test_place"),
        include_runtime: true,
        include_test_code: false,
        auto_start_server_scripts: true,
        place_version: "0.1.0".to_string(),
        place_name: "RobloxRS Test Project".to_string(),
    };
    
    // Create the place generator
    let mut generator = PlaceGenerator::new(config);
    
    // Scan the project
    match generator.scan_project() {
        Ok(_) => println!("Project scanned successfully"),
        Err(e) => {
            println!("Failed to scan project: {}", e);
            return false;
        }
    }
    
    // Transpile the code
    match generator.transpile_code() {
        Ok(_) => println!("Code transpiled successfully"),
        Err(e) => {
            println!("Failed to transpile code: {}", e);
            return false;
        }
    }
    
    // Generate the place file
    match generator.generate_place_file() {
        Ok(place_file) => {
            println!("Place file generated successfully at: {}", place_file.display());
            
            // Check if the place file exists
            if !place_file.exists() {
                println!("Place file was not created");
                return false;
            }
            
            // Check if the XML version exists
            let xml_file = place_file.with_extension("rbxlx");
            if !xml_file.exists() {
                println!("XML version of place file was not created");
                return false;
            }
            
            // Read the XML to make sure it contains expected elements
            let xml_content = fs::read_to_string(&xml_file).expect("Failed to read XML file");
            
            // Check for client scripts
            if !xml_content.contains("client_main") {
                println!("XML does not contain client scripts");
                return false;
            }
            
            // Check for shared scripts
            if !xml_content.contains("game_data") {
                println!("XML does not contain shared scripts");
                return false;
            }
            
            // Check for server scripts
            if !xml_content.contains("server_main") {
                println!("XML does not contain server scripts");
                return false;
            }
            
            // Check for runtime library
            if !xml_content.contains("RobloxRS_Runtime") {
                println!("XML does not contain runtime library");
                return false;
            }
            
            // Check for workspace structure
            if !xml_content.contains("ReplicatedStorage") || 
               !xml_content.contains("ServerScriptService") || 
               !xml_content.contains("StarterPlayerScripts") {
                println!("XML does not contain required Roblox services");
                return false;
            }
            
            println!("Place file content verified successfully");
            true
        },
        Err(e) => {
            println!("Failed to generate place file: {}", e);
            false
        }
    }
}

fn main() {
    // Run the test
    let result = test_place_generator();
    
    println!("Test result: {}", if result { "PASSED" } else { "FAILED" });
    
    // Set the exit code based on the test result
    std::process::exit(if result { 0 } else { 1 });
}
