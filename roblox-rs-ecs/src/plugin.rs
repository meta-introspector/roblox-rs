//! Plugin module for the ECS framework
//!
//! Plugins provide a way to extend the framework with additional functionality.

use std::any::TypeId;
use std::collections::HashMap;

use crate::app::App;
use crate::roblox::{
    Workspace, Players, ReplicatedStorage, ServerStorage,
    ServerScriptService, StarterPlayer, StarterGui, StarterPack,
};

/// A plugin for the ECS framework
pub trait Plugin: Send + Sync + 'static {
    /// Build the plugin into the app
    fn build(&self, app: &mut App);
    
    /// Get the name of the plugin
    fn name(&self) -> &str;
}

/// A group of plugins that can be added to an app
pub trait PluginGroup: Send + Sync + 'static {
    /// Configure the plugin group
    fn configure(&mut self);
    
    /// Build the plugin group into the app
    fn build(&mut self, app: &mut App);
}

/// Default plugins for a basic app
#[derive(Default)]
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn configure(&mut self) {
        // No configuration needed
    }
    
    fn build(&mut self, app: &mut App) {
        // Add core plugins
        app.add_plugin(CorePlugin);
        
        // Add default roblox integration if the feature is enabled
        #[cfg(feature = "roblox-instances")]
        app.add_plugin(RobloxPlugin);
    }
}

/// Core plugin for basic ECS functionality
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Set up basic world resources
        app.init_resource::<HashMap<TypeId, String>>();
    }
    
    fn name(&self) -> &str {
        "CorePlugin"
    }
}

/// Plugin for Roblox integration
pub struct RobloxPlugin;

impl Plugin for RobloxPlugin {
    fn build(&self, app: &mut App) {
        // Add Roblox services as resources
        app.insert_resource(Workspace::new());
        app.insert_resource(Players::new());
        app.insert_resource(ReplicatedStorage::new());
        app.insert_resource(ServerStorage::new());
        app.insert_resource(ServerScriptService::new());
        app.insert_resource(StarterPlayer::new());
        app.insert_resource(StarterGui::new());
        app.insert_resource(StarterPack::new());
    }
    
    fn name(&self) -> &str {
        "RobloxPlugin"
    }
}

/// Minimal plugins for a lightweight app
#[derive(Default)]
pub struct MinimalPlugins;

impl PluginGroup for MinimalPlugins {
    fn configure(&mut self) {
        // No configuration needed
    }
    
    fn build(&mut self, app: &mut App) {
        // Add only the core plugin
        app.add_plugin(CorePlugin);
    }
} 