//! A Bevy-inspired Entity Component System (ECS) framework for Roblox-rs.
//!
//! This crate provides a data-oriented approach to game development on Roblox
//! through a familiar ECS architecture.

mod app;
mod component;
mod entity;
mod event;
mod platform;
mod plugin;
mod query;
mod resource;
mod schedule;
mod system;
mod world;

pub mod prelude;
pub mod roblox;

// Re-export key types for convenient usage
pub use app::{App, AppBuilder};
pub use component::{Component, ComponentRegistry};
pub use entity::{Entity, EntityBuilder, EntityRef};
pub use event::{Event, EventReader, EventWriter};
pub use platform;
pub use plugin::{Plugin, PluginGroup, RobloxPlugin};
pub use query::{Query, QueryIter};
pub use resource::{Res, ResMut, Resource};
pub use system::{Commands, IntoSystem, System, SystemParam};
pub use world::World;

/// Current version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Errors that can occur in the ECS framework
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("entity not found: {0:?}")]
    EntityNotFound(Entity),
    
    #[error("component not found for entity: {0:?}")]
    ComponentNotFound(Entity),
    
    #[error("resource not found: {0}")]
    ResourceNotFound(String),
    
    #[error("system error: {0}")]
    SystemError(String),
    
    #[error("roblox instance error: {0}")]
    RobloxError(String),
    
    #[error("other error: {0}")]
    Other(String),
}

/// Result type for ECS operations
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let app = App::new();
        assert_eq!(app.world().entities().len(), 0);
    }
} 