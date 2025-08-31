use std::ffi::{c_void, c_char, c_int};

/// Operation result type.
type OpRes = c_int;

/// Pointer to a raw C string.
type StrPtr = *const c_char;

/// Pointer to a library handle.
type LibPtr = *mut c_void;

/// Pointer to a function handle.
type FuncPtr = *mut c_void;

#[cfg(windows)]
#[link(name = "kernel32")]
unsafe extern "system" {
    /// Loads the specified module into the address space of the calling process.
    #[link_name = "LoadLibraryA"]
    fn load_library(lpLibFileName: StrPtr) -> LibPtr;

    /// Retrieves the address of an exported function from a dynamic-link library.
    #[link_name = "GetProcAddress"]
    fn get_proc_address(hLibModule: LibPtr, lpProcName: StrPtr) -> FuncPtr;

    /// Frees a loaded dynamic-link library module.
    #[link_name = "FreeLibrary"]
    fn free_library(hLibModule: LibPtr) -> OpRes;
}