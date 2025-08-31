use std::ffi::c_void;
use std::os::raw::{c_char, c_int};

/// Pointer to a raw C string.
type StringPtr = *const c_char;

/// Pointer to a dynamic-link module.
type ModulePtr = *mut c_void;

/// Pointer to a symbol in a dynamic-link module.
type SymbolPtr = *mut c_void;

/// Result of an operation.
type OperationResult = c_int;

#[cfg(windows)]
mod win32 {
    use super::{StringPtr, ModulePtr, SymbolPtr, OperationResult};

    #[link(name = "kernel32")]
    unsafe extern "system" {
        /// Loads the specified module into the address space of the calling process.
        #[link_name = "LoadLibraryA"]
        fn load_library(library_filename: StringPtr) -> ModulePtr;

        /// Retrieves the address of an exported symbol from a dynamic-link library.
        #[link_name = "GetProcAddress"]
        fn get_proc_address(library_module: ModulePtr, symbol_name: StringPtr) -> SymbolPtr;

        /// Frees a loaded dynamic-link library module.
        #[link_name = "FreeLibrary"]
        fn free_library(library_module: ModulePtr) -> OperationResult;
    }
}