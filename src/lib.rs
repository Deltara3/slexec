#[cfg(not(any(target_os = "windows")))]
compile_error!("You are using a currently unsupported operating system.");

/// Raw bindings to OS specific library loading functions.
pub mod raw;

/// Safe-ish bindings to the raw library loading bindings.
pub mod wrap;