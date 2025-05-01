use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use crate::RobloxResult;

/// Represents an event that can be fired and connected to
#[derive(Clone)]
pub struct Event<T: Clone + Send + 'static> {
    sender: broadcast::Sender<T>,
}

impl<T: Clone + Send + 'static> Event<T> {
    /// Create a new event
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(32);
        Self { sender }
    }
    
    /// Connect a callback to this event
    pub fn connect<F>(&self, callback: F) -> EventConnection<T>
    where
        F: FnMut(T) + Send + 'static,
    {
        let mut receiver = self.sender.subscribe();
        let callback = Arc::new(Mutex::new(callback));
        
        let handle = tokio::spawn({
            let callback = callback.clone();
            async move {
                while let Ok(value) = receiver.recv().await {
                    if let Ok(mut cb) = callback.lock() {
                        cb(value);
                    }
                }
            }
        });
        
        EventConnection {
            _handle: handle,
            _callback: callback,
        }
    }
    
    /// Fire the event with a value
    pub fn fire(&self, value: T) -> RobloxResult<()> {
        let _ = self.sender.send(value);
        Ok(())
    }
}

/// Represents a connection to an event
pub struct EventConnection<T: Clone + Send + 'static> {
    _handle: tokio::task::JoinHandle<()>,
    _callback: Arc<Mutex<dyn FnMut(T) + Send>>,
}

impl<T: Clone + Send + 'static> Drop for EventConnection<T> {
    fn drop(&mut self) {
        self._handle.abort();
    }
}

/// A simple event that can be fired without any value
pub type SignalEvent = Event<()>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    
    #[tokio::test]
    async fn test_event_connection() {
        let event = Event::new();
        let called = Arc::new(AtomicBool::new(false));
        
        let called_clone = called.clone();
        let _connection = event.connect(move |value: i32| {
            assert_eq!(value, 42);
            called_clone.store(true, Ordering::SeqCst);
        });
        
        event.fire(42).unwrap();
        
        // Give the event some time to propagate
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        assert!(called.load(Ordering::SeqCst));
    }
    
    #[tokio::test]
    async fn test_multiple_connections() {
        let event = Event::new();
        let counter = Arc::new(Mutex::new(0));
        
        let counter_clone = counter.clone();
        let _conn1 = event.connect(move |_| {
            *counter_clone.lock().unwrap() += 1;
        });
        
        let counter_clone = counter.clone();
        let _conn2 = event.connect(move |_| {
            *counter_clone.lock().unwrap() += 1;
        });
        
        event.fire(()).unwrap();
        
        // Give the event some time to propagate
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        assert_eq!(*counter.lock().unwrap(), 2);
    }
    
    #[tokio::test]
    async fn test_connection_cleanup() {
        let event = Event::new();
        let counter = Arc::new(Mutex::new(0));
        
        let counter_clone = counter.clone();
        let conn = event.connect(move |_| {
            *counter_clone.lock().unwrap() += 1;
        });
        
        event.fire(()).unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        assert_eq!(*counter.lock().unwrap(), 1);
        
        // Drop the connection
        drop(conn);
        
        event.fire(()).unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        assert_eq!(*counter.lock().unwrap(), 1); // Counter should not have increased
    }
} 