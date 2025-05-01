//! Platform abstraction module
//!
//! This module provides platform-specific implementations for various features,
//! allowing the codebase to work on both native platforms and Roblox.

// Import the appropriate platform module based on features
#[cfg(feature = "roblox")]
mod roblox;
#[cfg(not(feature = "roblox"))]
mod native;

// Re-export platform-specific functionality from the selected module
#[cfg(feature = "roblox")]
pub use self::roblox::*;
#[cfg(not(feature = "roblox"))]
pub use self::native::*;

/// Common traits and types for all platforms
pub mod common {
    /// Trait for platform-specific time functionality
    pub trait Time {
        /// Get the current time in seconds
        fn now() -> f64;
        
        /// Sleep for a specified number of seconds (may be a no-op on some platforms)
        fn sleep(seconds: f64);
    }
    
    /// Trait for platform-specific logging
    pub trait Logger {
        /// Log a message at the info level
        fn info(message: &str);
        
        /// Log a message at the warning level
        fn warn(message: &str);
        
        /// Log a message at the error level
        fn error(message: &str);
    }
    
    /// Trait for platform-specific threading
    pub trait Threading {
        /// Create a new thread executing the given function
        /// 
        /// This may be a real thread on native platforms or a coroutine on Roblox
        fn spawn<F>(f: F) where F: FnOnce() + Send + 'static;
        
        /// Yield to other threads/coroutines
        fn yield_now();
    }
} 