#[cfg(any(target_os = "macos", target_os = "linux"))]
use crate::raw::unix::{self, RTLD_LAZY};
#[cfg(target_os = "windows")]
use crate::raw::win32;
use std::ffi::{CString, c_char, c_void};
use std::mem;

/// Wrapper around a dynamic-link library.
pub struct Library {
    handle: *mut c_void,
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

            #[cfg(any(target_os = "macos", target_os = "linux"))]
            let handle = unix::dlopen(c_path.as_ptr(), RTLD_LAZY);

            // If handle is null, exit.
            if handle.is_null() {
                #[cfg(target_os = "windows")]
                return Err(win32::get_last_human_error());

                #[cfg(any(target_os = "macos", target_os = "linux"))]
                return Err(unix::dlerror());
            }

            Ok(Library { handle })
        }
    }

    pub fn get(&self, name: &str) -> Result<extern "C" fn(*const *const c_char), String> {
        unsafe {
            // Convert Rust string to C string.
            let c_name = CString::new(name).unwrap();

            // Locate symbol.
            #[cfg(target_os = "windows")]
            let ptr = win32::get_proc_address(self.handle, c_name.as_ptr());

            #[cfg(any(target_os = "macos", target_os = "linux"))]
            let ptr = unix::dlsym(self.handle, c_name.as_ptr());

            // If symbol is null, exit.
            if ptr.is_null() {
                #[cfg(target_os = "windows")]
                return Err(win32::get_last_human_error());

                #[cfg(any(target_os = "macos", target_os = "linux"))]
                return Err(unix::dlerror());
            }

            // Transmute to function, this is really unsafe as we assume everything matches.
            let func: extern "C" fn(*const *const c_char) = mem::transmute(ptr);

            Ok(func)
        }
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            if !self.handle.is_null() {
                #[cfg(target_os = "windows")]
                win32::free_library(self.handle);

                #[cfg(any(target_os = "macos", target_os = "linux"))]
                unix::dlclose(self.handle);
            }
        }
    }
}
