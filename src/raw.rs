#[cfg(windows)]
mod win32 {
    use std::ffi::c_void;
    use std::os::raw::{c_char, c_int};

    #[link(name = "kernel32")]
    unsafe extern "system" {
        /// Loads the specified module into the address space of the calling process.
        #[link_name = "LoadLibraryA"]
        fn load_library(library_filename: *const c_char) -> *mut c_void;

        /// Retrieves the address of an exported symbol from a dynamic-link library.
        #[link_name = "GetProcAddress"]
        fn get_proc_address(library_module: *mut c_void, symbol_name: *const c_char) -> *mut c_void;

        /// Frees a loaded dynamic-link library module.
        #[link_name = "FreeLibrary"]
        fn free_library(library_module: *mut c_void) -> *mut c_int;
    }
}