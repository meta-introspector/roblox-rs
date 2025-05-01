// Client UI Controller
// Demonstrates Roblox Instance interaction from Rust

use roblox_rs::instance::*;

pub struct UIController {
    main_frame: Instance,
    start_button: Instance,
    quit_button: Instance,
}

impl UIController {
    pub fn new() -> Self {
        // Create the main UI frame
        let main_frame = Instance::new("Frame");
        main_frame.set_property("Name", "MainMenu");
        main_frame.set_property("Size", Vector2::new(400, 300));
        main_frame.set_property("Position", UDim2::new(0.5, -200, 0.5, -150));
        main_frame.set_property("BackgroundColor3", Color3::new(0.1, 0.1, 0.1));
        main_frame.set_property("BackgroundTransparency", 0.2);
        
        // Create the start button
        let start_button = Instance::new("TextButton");
        start_button.set_property("Name", "StartButton");
        start_button.set_property("Size", UDim2::new(0.8, 0, 0.2, 0));
        start_button.set_property("Position", UDim2::new(0.5, -160, 0.4, -30));
        start_button.set_property("BackgroundColor3", Color3::new(0, 0.6, 0.1));
        start_button.set_property("Text", "Start Game");
        start_button.set_property("TextSize", 24);
        start_button.set_property("Parent", &main_frame);
        
        // Create the quit button
        let quit_button = Instance::new("TextButton");
        quit_button.set_property("Name", "QuitButton");
        quit_button.set_property("Size", UDim2::new(0.8, 0, 0.2, 0));
        quit_button.set_property("Position", UDim2::new(0.5, -160, 0.7, -30));
        quit_button.set_property("BackgroundColor3", Color3::new(0.8, 0.1, 0.1));
        quit_button.set_property("Text", "Quit");
        quit_button.set_property("TextSize", 24);
        quit_button.set_property("Parent", &main_frame);
        
        // Return the controller
        UIController {
            main_frame,
            start_button,
            quit_button,
        }
    }
    
    pub fn show(&self, parent: &Instance) {
        self.main_frame.set_property("Parent", parent);
    }
    
    pub fn hide(&self) {
        self.main_frame.set_property("Parent", "nil");
    }
    
    pub fn on_start_clicked<F>(&self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.start_button.connect("MouseButton1Click", callback);
    }
    
    pub fn on_quit_clicked<F>(&self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.quit_button.connect("MouseButton1Click", callback);
    }
}

// Entry point for the client code
pub fn init() {
    // Get the player GUI
    let player_gui = get_service("Players")
        .find_first_child("LocalPlayer")
        .wait_for_child("PlayerGui");
    
    // Create the UI controller
    let ui = UIController::new();
    ui.show(&player_gui);
    
    // Set up button click handlers
    ui.on_start_clicked(|| {
        println!("Starting game...");
        // You would put game start logic here
    });
    
    ui.on_quit_clicked(|| {
        println!("Quitting game...");
        ui.hide();
    });
}

// Helper functions to make the code more Rust-like
fn get_service(name: &str) -> Instance {
    Instance::get_service(name)
}

struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }
}

struct UDim2 {
    scale_x: f32,
    offset_x: f32,
    scale_y: f32,
    offset_y: f32,
}

impl UDim2 {
    fn new(scale_x: f32, offset_x: f32, scale_y: f32, offset_y: f32) -> Self {
        UDim2 {
            scale_x,
            offset_x,
            scale_y,
            offset_y,
        }
    }
}

struct Color3 {
    r: f32,
    g: f32,
    b: f32,
}

impl Color3 {
    fn new(r: f32, g: f32, b: f32) -> Self {
        Color3 { r, g, b }
    }
}
