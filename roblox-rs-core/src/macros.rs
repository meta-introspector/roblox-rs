//! Macros to help with Luau transpilation
//!
//! This module provides macros that assist with writing code that can be
//! compiled to both Rust and Luau.

/// Marks code that has a special Luau implementation
///
/// When transpiling to Luau, the code inside this macro will be replaced
/// with the Luau-specific implementation.
///
/// # Examples
///
/// ```
/// # use roblox_rs_core::luau_impl;
/// fn get_time() -> f64 {
///     luau_impl! {
///         // This code runs in Rust
///         std::time::SystemTime::now()
///             .duration_since(std::time::UNIX_EPOCH)
///             .unwrap_or_else(|_| std::time::Duration::from_secs(0))
///             .as_secs_f64()
///         
///         // This comment is a directive for the Luau transpiler:
///         // @luau: return os.clock()
///     }
/// }
/// ```
#[macro_export]
macro_rules! luau_impl {
    ($($code:tt)*) => {
        // In Rust, we just execute the code directly
        $($code)*
    };
}

/// Marks code that should only be included in Luau output
///
/// The code within this macro will be completely ignored in Rust,
/// but will be included in the Luau output.
///
/// # Examples
///
/// ```
/// # use roblox_rs_core::luau_only;
/// fn do_something() {
///     // This code runs in both Rust and Luau
///     let x = 42;
///     
///     luau_only! {
///         // This code only runs in Luau
///         -- This is Luau code, not Rust
///         print("Hello from Luau!")
///     }
/// }
/// ```
#[macro_export]
macro_rules! luau_only {
    ($($code:tt)*) => {
        // In Rust, this is a no-op
        #[cfg(feature = "luau_codegen")]
        {
            let _ = stringify!($($code)*);
        }
    };
}

/// Marks code that should only be included in Rust output
///
/// The code within this macro will be completely removed from Luau output.
///
/// # Examples
///
/// ```
/// # use roblox_rs_core::rust_only;
/// fn do_something() {
///     // This code runs in both Rust and Luau
///     let x = 42;
///     
///     rust_only! {
///         // This code only runs in Rust
///         println!("Hello from Rust!");
///     }
/// }
/// ```
#[macro_export]
macro_rules! rust_only {
    ($($code:tt)*) => {
        // In Rust, execute the code
        $($code)*
        // In Luau, this will be removed completely
    };
}

/// Tag a function or struct as requiring special handling during transpilation
///
/// This attribute doesn't do anything at runtime, but it's used by the
/// transpiler to identify items that need special handling.
///
/// # Examples
///
/// ```
/// # use roblox_rs_core::luau_transpile;
/// #[luau_transpile(direct_translation = false)]
/// fn complex_function() {
///     // This function will be handled specially by the transpiler
/// }
/// ```
#[cfg(feature = "proc_macros")]
pub use roblox_rs_macros::luau_transpile;

/// Define a function that will be transpiled directly to Luau
///
/// This allows embedding raw Luau code within Rust code.
///
/// # Examples
///
/// ```
/// # use roblox_rs_core::luau_function;
/// #[luau_function]
/// fn wait_for_seconds(seconds: f64) -> () {
///     r#"
///     -- This is raw Luau code
///     task.wait(seconds)
///     "#
/// }
/// ```
#[cfg(feature = "proc_macros")]
pub use roblox_rs_macros::luau_function; 