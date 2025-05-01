//! Roblox API simulation and Luau integration for roblox-rs
//! 
//! This crate provides simulated Roblox APIs and Luau integration for the roblox-rs compiler.
//! It allows testing Rust code that will be compiled to Luau without running it on Roblox.

use std::sync::Arc;
use thiserror::Error;
use mlua::prelude::*;

mod instance;
mod datamodel;
mod services;
mod events;
mod types;

pub use instance::Instance;
pub use datamodel::DataModel;
pub use services::*;
pub use events::*;
pub use types::*;

/// Error types for Roblox API operations
#[derive(Error, Debug)]
pub enum RobloxError {
    #[error("Instance not found: {0}")]
    InstanceNotFound(String),
    
    #[error("Invalid property: {0}")]
    InvalidProperty(String),
    
    #[error("Type mismatch: expected {expected}, got {got}")]
    TypeMismatch {
        expected: String,
        got: String,
    },
    
    #[error("Luau error: {0}")]
    LuauError(#[from] mlua::Error),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for Roblox API operations
pub type RobloxResult<T> = Result<T, RobloxError>;

/// The main entry point for the Roblox API simulation
#[derive(Clone)]
pub struct RobloxRuntime {
    datamodel: Arc<DataModel>,
    lua: Arc<Lua>,
}

impl RobloxRuntime {
    /// Create a new Roblox runtime instance
    pub fn new() -> RobloxResult<Self> {
        let lua = Lua::new();
        let datamodel = DataModel::new();
        
        Ok(Self {
            datamodel: Arc::new(datamodel),
            lua: Arc::new(lua),
        })
    }
    
    /// Get the DataModel instance
    pub fn get_datamodel(&self) -> Arc<DataModel> {
        self.datamodel.clone()
    }
    
    /// Get the Lua state
    pub fn get_lua(&self) -> Arc<Lua> {
        self.lua.clone()
    }
    
    /// Execute Luau code in the runtime
    pub fn execute_luau(&self, code: &str) -> RobloxResult<mlua::Value> {
        self.lua.load(code)
            .eval()
            .map_err(RobloxError::LuauError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_runtime_creation() {
        let runtime = RobloxRuntime::new().unwrap();
        assert!(runtime.get_datamodel().get_service::<services::Workspace>().is_ok());
    }
    
    #[test]
    fn test_luau_execution() {
        let runtime = RobloxRuntime::new().unwrap();
        let result = runtime.execute_luau("return 1 + 1").unwrap();
        assert_eq!(result.as_integer(), Some(2));
    }
} 