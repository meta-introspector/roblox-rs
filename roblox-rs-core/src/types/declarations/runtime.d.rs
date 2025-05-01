/// Type declarations for the Roblox-RS runtime

/// Main Roblox-RS runtime namespace
pub mod RobloxRS {
    /// Object pooling system
    pub mod Pool {
        /// Create a new object pool
        pub fn new(objectType: String, initialSize: Option<i32>, factory: Option<fn() -> Instance>) -> ObjectPool;
    }

    /// Object pool for efficient reuse of instances
    pub struct ObjectPool {
        /// Get an object from the pool or create a new one
        pub fn get(&self) -> Instance,
        /// Return an object to the pool
        pub fn release(&self, object: Instance),
        /// Get pool statistics
        pub fn stats(&self) -> PoolStats,
    }

    /// Statistics for an object pool
    pub struct PoolStats {
        /// Number of available objects in the pool
        pub available: i32,
        /// Total number of allocated objects
        pub allocated: i32,
        /// Number of active objects (allocated - available)
        pub active: i32,
    }

    /// Parallel execution utilities
    pub mod Parallel {
        /// Iterate over array elements in parallel
        pub fn forEach<T, R>(array: Vec<T>, callback: fn(item: T, index: i32) -> R) -> Vec<R>;
        
        /// Transform array elements in parallel
        pub fn map<T, R>(array: Vec<T>, transformer: fn(item: T, index: i32) -> R) -> Vec<R>;
        
        /// Filter array elements in parallel
        pub fn filter<T>(array: Vec<T>, predicate: fn(item: T, index: i32) -> bool) -> Vec<T>;
    }

    /// Rust-like Result type
    pub mod Result {
        /// Create a success result
        pub fn ok<T>(value: T) -> ResultType<T>;
        
        /// Create an error result
        pub fn err<T>(error: String) -> ResultType<T>;
    }

    /// Result type wrapper
    pub struct ResultType<T> {
        /// Whether the result is successful
        pub success: bool,
        /// Result value if successful
        pub value: Option<T>,
        /// Error message if failed
        pub error: Option<String>,
        
        /// Check if result is successful
        pub fn isOk(&self) -> bool,
        
        /// Check if result is an error
        pub fn isErr(&self) -> bool,
        
        /// Get value or panic with error message
        pub fn unwrap(&self) -> T,
        
        /// Get value or return default
        pub fn unwrapOr(&self, default: T) -> T,
    }

    /// Debug utilities
    pub mod Debug {
        /// Trace a function call
        pub fn traceCall(funcName: String, args: dynamic),
        
        /// Get the current call stack
        pub fn getCallStack() -> Vec<CallStackEntry>,
        
        /// Clear the call stack
        pub fn clearCallStack(),
        
        /// Set a breakpoint
        pub fn setBreakpoint(line: i32, condition: Option<fn(context: dynamic) -> bool>),
        
        /// Check if hitting a breakpoint
        pub fn checkBreakpoint(line: i32, context: dynamic) -> bool,
        
        /// Watch a variable
        pub fn watch(name: String, getValue: fn() -> dynamic),
        
        /// Get all watch values
        pub fn getWatchValues() -> Map<String, dynamic>,
    }

    /// Call stack entry information
    pub struct CallStackEntry {
        /// Function name
        pub name: String,
        /// Function arguments
        pub args: dynamic,
        /// Source line number
        pub line: i32,
        /// Source file
        pub source: String,
        /// Call timestamp
        pub time: f64,
    }

    /// Profiling utilities
    pub mod Profiler {
        /// Enable or disable the profiler
        pub fn enable(enabled: bool) -> bool,
        
        /// Start profiling a function
        pub fn start(funcName: String) -> fn(),
        
        /// Reset profiler data
        pub fn reset(),
        
        /// Get profiling results
        pub fn getResults() -> Vec<ProfileResult>,
    }

    /// Profiling result for a function
    pub struct ProfileResult {
        /// Function name
        pub name: String,
        /// Number of calls
        pub calls: i32,
        /// Total execution time
        pub totalTime: f64,
        /// Average execution time per call
        pub avgTime: f64,
        /// Total memory usage
        pub totalMemory: i32,
        /// Average memory usage per call
        pub avgMemory: f64,
    }

    /// Table utilities
    pub mod Table {
        /// Deep copy a table
        pub fn deepCopy<T>(original: T) -> T,
        
        /// Shallow copy a table
        pub fn shallowCopy<T>(original: T) -> T,
    }

    /// Vector utilities
    pub mod Vector {
        /// Convert a table to Vector3
        pub fn toVector3(tbl: dynamic) -> Vector3,
        
        /// Convert Vector3 to a table
        pub fn toTable(vec: Vector3) -> { x: f32, y: f32, z: f32 },
    }

    /// Color utilities
    pub mod Color {
        /// Convert a table to Color3
        pub fn toColor3(tbl: dynamic) -> Color3,
        
        /// Convert Color3 to a table
        pub fn toTable(color: Color3) -> { r: f32, g: f32, b: f32 },
    }
}
