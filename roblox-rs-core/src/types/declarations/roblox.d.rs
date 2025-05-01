/// Roblox Global APIs type declarations

/// DataModel is the root of the Roblox game hierarchy
pub struct DataModel {
    /// Workspace contains the 3D environment objects
    pub Workspace: Workspace,
    /// Players service manages player objects
    pub Players: PlayerService,
    /// ReplicatedStorage is shared between server and client
    pub ReplicatedStorage: ReplicatedStorage,
    /// ServerStorage is only accessible on the server
    pub ServerStorage: ServerStorage,
    /// Lighting manages the game's lighting and atmosphere
    pub Lighting: LightingService,
}

/// Workspace contains 3D objects in the game world
pub struct Workspace {
    /// Current gravity in the workspace
    pub Gravity: f32,
    /// Get all parts in the workspace
    pub GetParts: fn() -> Vec<BasePart>,
    /// Get a child by name
    pub FindFirstChild: fn(name: String) -> Option<Instance>,
}

/// Player service manages players
pub struct PlayerService {
    /// Get a player by user ID
    pub GetPlayerByUserId: fn(userId: i64) -> Option<Player>,
    /// Array of all players in the game
    pub GetPlayers: fn() -> Vec<Player>,
    /// Event fired when a player joins
    pub PlayerAdded: RbxScriptSignal<fn(player: Player)>,
    /// Event fired when a player leaves
    pub PlayerRemoving: RbxScriptSignal<fn(player: Player)>,
}

/// Player represents a user in the game
pub struct Player {
    /// The player's user ID
    pub UserId: i64,
    /// The player's display name
    pub DisplayName: String,
    /// The player's character in the 3D world
    pub Character: Option<Model>,
    /// The player's backpack containing tools
    pub Backpack: Backpack,
    /// Return whether the player has a specific permission
    pub HasPermission: fn(permission: String) -> bool,
}

/// Model is a container for other objects
pub struct Model {
    /// Position of the model's primary part
    pub PrimaryPart: Option<BasePart>,
    /// Set the primary part of the model
    pub SetPrimaryPartCFrame: fn(cf: CFrame),
    /// Get all parts in the model
    pub GetParts: fn() -> Vec<BasePart>,
}

/// BasePart is a physical object in the 3D world
pub struct BasePart {
    /// Position and orientation
    pub CFrame: CFrame,
    /// Size of the part
    pub Size: Vector3,
    /// Part color
    pub Color: Color3,
    /// Part transparency (0-1)
    pub Transparency: f32,
    /// Whether the part can collide
    pub CanCollide: bool,
    /// Part anchored state
    pub Anchored: bool,
}

/// CFrame (Coordinate Frame) represents position and orientation
pub struct CFrame {
    /// X position
    pub X: f32,
    /// Y position
    pub Y: f32,
    /// Z position
    pub Z: f32,
    /// Look at a position
    pub LookAt: fn(target: Vector3) -> CFrame,
    /// Get position as Vector3
    pub GetPosition: fn() -> Vector3,
}

/// Vector3 represents a 3D vector
pub struct Vector3 {
    /// X component
    pub X: f32,
    /// Y component
    pub Y: f32,
    /// Z component
    pub Z: f32,
    /// Constructor
    pub new: fn(x: f32, y: f32, z: f32) -> Vector3,
    /// Magnitude of the vector
    pub Magnitude: f32,
    /// Unit vector in the same direction
    pub Unit: Vector3,
    /// Dot product with another vector
    pub Dot: fn(other: Vector3) -> f32,
    /// Cross product with another vector
    pub Cross: fn(other: Vector3) -> Vector3,
}

/// Color3 represents an RGB color
pub struct Color3 {
    /// Red component (0-1)
    pub R: f32,
    /// Green component (0-1)
    pub G: f32,
    /// Blue component (0-1)
    pub B: f32,
    /// Constructor from RGB
    pub new: fn(r: f32, g: f32, b: f32) -> Color3,
    /// Constructor from RGB (0-255)
    pub fromRGB: fn(r: i32, g: i32, b: i32) -> Color3,
    /// Constructor from HSV
    pub fromHSV: fn(h: f32, s: f32, v: f32) -> Color3,
}

/// Roblox script signal for events
pub struct RbxScriptSignal<T> {
    /// Connect a function to this event
    pub Connect: fn(callback: T) -> RbxScriptConnection,
    /// Connect a function to this event, but only trigger once
    pub Once: fn(callback: T) -> RbxScriptConnection,
    /// Wait for the event to fire
    pub Wait: fn() -> Vec<dynamic>,
}

/// Connection to an event
pub struct RbxScriptConnection {
    /// Whether the connection is still active
    pub Connected: bool,
    /// Disconnect from the event
    pub Disconnect: fn(),
}

/// Storage accessible by both client and server
pub struct ReplicatedStorage {
    /// Get a child by name
    pub FindFirstChild: fn(name: String) -> Option<Instance>,
    /// Wait for a child to be added
    pub WaitForChild: fn(name: String) -> Instance,
}

/// Storage only accessible on the server
pub struct ServerStorage {
    /// Get a child by name
    pub FindFirstChild: fn(name: String) -> Option<Instance>,
    /// Wait for a child to be added
    pub WaitForChild: fn(name: String) -> Instance,
}

/// Service that controls lighting
pub struct LightingService {
    /// Time of day in 24-hour format
    pub TimeOfDay: String,
    /// Whether shadows are enabled
    pub GlobalShadows: bool,
    /// Ambient color in shadow areas
    pub Ambient: Color3,
    /// Brightness multiplier for direct light
    pub Brightness: f32,
    /// Exposure compensation
    pub ExposureCompensation: f32,
}

/// Base class for all Roblox objects
pub struct Instance {
    /// Unique identifier
    pub Name: String,
    /// Class name of the instance
    pub ClassName: String,
    /// Parent instance
    pub Parent: Option<Instance>,
    /// Clone this instance
    pub Clone: fn() -> Instance,
    /// Destroy this instance
    pub Destroy: fn(),
    /// Get a child by name
    pub FindFirstChild: fn(name: String) -> Option<Instance>,
    /// Wait for a child to be added
    pub WaitForChild: fn(name: String) -> Instance,
}

/// Player's backpack containing tools
pub struct Backpack {
    /// Get all tools in the backpack
    pub GetTools: fn() -> Vec<Tool>,
    /// Get a tool by name
    pub FindFirstTool: fn(name: String) -> Option<Tool>,
}

/// Tool that can be equipped by a player
pub struct Tool {
    /// Tool name
    pub Name: String,
    /// Whether the tool is currently equipped
    pub Equipped: bool,
    /// Event when tool is activated (e.g. clicked)
    pub Activated: RbxScriptSignal<fn()>,
    /// Event when tool is equipped
    pub Equipped: RbxScriptSignal<fn()>,
    /// Event when tool is unequipped
    pub Unequipped: RbxScriptSignal<fn()>,
}
