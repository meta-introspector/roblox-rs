//! Native platform implementation
//!
//! This module provides implementations for platform-specific functionality
//! when running on native platforms (not Roblox).

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;

use super::common::{Time, Logger, Threading};

/// Native implementation of time functionality
pub struct NativeTime;

impl Time for NativeTime {
    fn now() -> f64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs_f64()
    }
    
    fn sleep(seconds: f64) {
        let millis = (seconds * 1000.0) as u64;
        thread::sleep(Duration::from_millis(millis));
    }
}

/// Native implementation of logging
pub struct NativeLogger;

impl Logger for NativeLogger {
    fn info(message: &str) {
        log::info!("{}", message);
    }
    
    fn warn(message: &str) {
        log::warn!("{}", message);
    }
    
    fn error(message: &str) {
        log::error!("{}", message);
    }
}

/// Native implementation of threading
pub struct NativeThreading;

impl Threading for NativeThreading {
    fn spawn<F>(f: F) where F: FnOnce() + Send + 'static {
        thread::spawn(f);
    }
    
    fn yield_now() {
        thread::yield_now();
    }
}

/// Thread-safe mutex implementation
#[cfg(feature = "parking_lot")]
pub use parking_lot::Mutex;

/// Thread-safe mutex implementation fallback
#[cfg(not(feature = "parking_lot"))]
pub use std::sync::Mutex;

/// Current time in seconds
pub fn time_now() -> f64 {
    NativeTime::now()
}

/// Sleep for specified seconds
pub fn sleep(seconds: f64) {
    NativeTime::sleep(seconds);
}

/// Log an info message
pub fn log_info(message: &str) {
    NativeLogger::info(message);
}

/// Log a warning message
pub fn log_warn(message: &str) {
    NativeLogger::warn(message);
}

/// Log an error message
pub fn log_error(message: &str) {
    NativeLogger::error(message);
}

/// Spawn a new thread
pub fn spawn_thread<F>(f: F) where F: FnOnce() + Send + 'static {
    NativeThreading::spawn(f);
}

/// Yield to other threads
pub fn thread_yield() {
    NativeThreading::yield_now();
} 