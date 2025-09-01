#[cfg(target_os = "windows")]
use crate::raw::win32;
use std::ffi::{CString, c_void, c_int};
use std::mem;

/// Wrapper around a dynamic-link library.
pub struct Library {
    handle: *mut c_void,
    debug: bool
}

impl Library {
    /// Opens a dynamic-link library.
    pub fn open(path: &str) -> Result<Self, String> {
        unsafe {
            // Convert Rust string to C string.
            let c_path = CString::new(path).unwrap();

            // Open dynamic-link library.
            #[cfg(target_os = "windows")]
            let handle = win32::load_library(c_path.as_ptr());

            // If handle is null, exit.
            if handle.is_null() {
                #[cfg(target_os = "windows")]
                return Err(win32::get_last_human_error());
            }

            Ok(Library { handle, debug: false })
        }
    }

    pub fn get(&self, name: &str) -> Result<extern "C" fn(), String> {
        unsafe {
            // Convert Rust string to C string.
            let c_name = CString::new(name).unwrap();

            // Locate symbol.
            #[cfg(target_os = "windows")]
            let ptr = win32::get_proc_address(self.handle, c_name.as_ptr());

            // If symbol is null, exit.
            if ptr.is_null() {
                #[cfg(target_os = "windows")]
                return Err(win32::get_last_human_error());
            }

            // Transmute to function, this is really unsafe as we assume everything matches.
            let func: extern "C" fn() = mem::transmute(ptr);

            Ok(func)
        }
    }

    /// Enables debug writing this instance.
    pub fn enable_debug(&mut self) {
        self.debug = true;
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            if !self.handle.is_null() {
                #[cfg(target_os = "windows")]
                win32::free_library(self.handle);
            }
        }
    }
}