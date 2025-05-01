// Roblox-RS 1.0 Simple Game Example
// Demonstrates the core features of our Roblox-RS 1.0 implementation

// Player struct for game logic
struct Player {
    name: String,
    health: i32,
    score: i32,
    position: (f32, f32, f32),
}

impl Player {
    // Create a new player
    fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            health: 100,
            score: 0,
            position: (0.0, 10.0, 0.0),
        }
    }
    
    // Deal damage to player
    fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
        if self.health < 0 {
            self.health = 0;
        }
    }
    
    // Add score to player
    fn add_score(&mut self, points: i32) {
        self.score += points;
    }
    
    // Check if player is alive
    fn is_alive(&self) -> bool {
        self.health > 0
    }
    
    // Move player to a new position
    fn move_to(&mut self, x: f32, y: f32, z: f32) {
        self.position = (x, y, z);
    }
}

// Game controller
fn start_game() {
    // Create player actor
    let player_actor = Actors::spawn(|mailbox| {
        let mut player = Player::new("RobloxPlayer");
        
        // Actor loop - process messages as they arrive
        while let Some(msg) = mailbox.receive() {
            match msg {
                "damage:10" => {
                    player.take_damage(10);
                    println!("Player took 10 damage. Health: {}", player.health);
                },
                "score:50" => {
                    player.add_score(50);
                    println!("Player scored 50 points. Score: {}", player.score);
                },
                "move" => {
                    player.move_to(10.0, 20.0, 30.0);
                    println!("Player moved to position: {:?}", player.position);
                },
                "status" => {
                    println!("Player status: Health={}, Score={}", player.health, player.score);
                },
                _ => println!("Unknown message: {}", msg),
            }
        }
    });
    
    // Set up UI
    create_ui();
    
    // Set up networking
    setup_networking();
    
    // Simulate game events
    player_actor.send("damage:10");
    player_actor.send("score:50");
    player_actor.send("move");
    player_actor.send("status");
}

// UI creation
fn create_ui() {
    // Create a ScreenGui
    let screen_gui = Instance::new("ScreenGui");
    let frame = Instance::new("Frame");
    frame.Size = UDim2::new(0, 300, 0, 200);
    frame.Position = UDim2::new(0.5, -150, 0.5, -100);
    frame.BackgroundColor3 = Color3::new(0.2, 0.2, 0.2);
    frame.Parent = screen_gui;
    
    // Add title
    let title = Instance::new("TextLabel");
    title.Text = "Roblox-RS 1.0 Demo";
    title.Size = UDim2::new(1, 0, 0, 30);
    title.TextColor3 = Color3::new(1, 1, 1);
    title.Parent = frame;
    
    // Add health display
    let health_label = Instance::new("TextLabel");
    health_label.Text = "Health: 100";
    health_label.Size = UDim2::new(1, 0, 0, 30);
    health_label.Position = UDim2::new(0, 0, 0, 40);
    health_label.TextColor3 = Color3::new(0, 1, 0);
    health_label.Parent = frame;
    
    // Add score display
    let score_label = Instance::new("TextLabel");
    score_label.Text = "Score: 0";
    score_label.Size = UDim2::new(1, 0, 0, 30);
    score_label.Position = UDim2::new(0, 0, 0, 80);
    score_label.TextColor3 = Color3::new(1, 1, 0);
    score_label.Parent = frame;
    
    // Add to player
    let players = Instance::getService("Players");
    let local_player = players.LocalPlayer;
    screen_gui.Parent = local_player.PlayerGui;
}

// Networking setup
fn setup_networking() {
    // Define custom event for player damage
    let damage_event = Net::defineEvent("PlayerDamaged", {
        player_id: "string",
        damage: "number",
        position: "Vector3",
    });
    
    // Server listens for the event
    damage_event.listen(|data| {
        println!("Player {} took {} damage at position {}", 
                data.player_id, data.damage, data.position);
    });
    
    // Define RPC for healing
    let heal_function = Net::defineRPC("HealPlayer", {
        player_id: "string",
        amount: "number",
    }, {
        success: "boolean",
        new_health: "number",
    });
    
    // Server implementation of the heal function
    heal_function.implement(|req| {
        println!("Healing player {} for {}", req.player_id, req.amount);
        return {
            success: true,
            new_health: 100,
        };
    });
}

fn main() {
    // Start the game
    start_game();
}
