//! Common imports for working with the ECS framework
//!
//! Import this module with `use roblox_rs_ecs::prelude::*;` to get access to the most
//! commonly used types and traits.

pub use crate::{
    App,
    AppBuilder,
    Component,
    Entity,
    EntityBuilder,
    Error,
    Event,
    EventReader,
    EventWriter,
    Plugin,
    PluginGroup,
    Query,
    Res,
    ResMut,
    Resource,
    Result,
    RobloxPlugin,
    System,
    SystemParam,
    World,
    Commands,
    IntoSystem,
};

// Re-export from roblox module
pub use crate::roblox::{
    RobloxInstance,
    Workspace,
    Players,
    ReplicatedStorage,
    ServerStorage,
    ServerScriptService,
    StarterPlayer,
    StarterGui,
    StarterPack,
}; 