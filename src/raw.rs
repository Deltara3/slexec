#[cfg(windows)]
pub mod win32 {
    use std::ptr;
    use std::ffi::{CStr, c_void, c_char, c_int, c_ulong};

    const FM_ALLOCATE_BUFFER: c_ulong = 0x00000100;
    const FM_IGNORE_INSERTS: c_ulong  = 0x00000200;
    const FM_FROM_SYSTEM: c_ulong     = 0x00001000;
    const LANG_NEUTRAL: c_ulong       = 0;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        /// Loads the specified module into the address space of the calling process.
        #[link_name = "LoadLibraryA"]
        pub unsafe fn load_library(library_filename: *const c_char) -> *mut c_void;

        /// Retrieves the address of an exported symbol from a dynamic-link library.
        #[link_name = "GetProcAddress"]
        pub unsafe fn get_proc_address(library_module: *mut c_void, 
                                       symbol_name: *const c_char) -> *mut c_void;

        /// Frees a loaded dynamic-link library module.
        #[link_name = "FreeLibrary"]
        pub unsafe fn free_library(library_module: *mut c_void) -> *mut c_int;

        /// Retrieves the calling thread's last-error code value.
        #[link_name = "GetLastError"]
        unsafe fn get_last_error() -> c_ulong;

        /// Formats a message string.
        #[link_name = "FormatMessageA"]
        unsafe fn format_message(flags: c_ulong,
                                     source: *const c_void,
                                     message_id: c_ulong,
                                     language_id: c_ulong,
                                     buffer: *mut *mut c_char,
                                     size: c_ulong,
                                     arugments: *mut c_void) -> c_ulong;

        /// Frees the specified local memory object.
        #[link_name = "LocalFree"]
        unsafe fn local_free(mem: *mut c_void) -> *mut c_void;
    }

    /// Retrieves the calling thread's last-error code in a human readable format.
    pub unsafe fn get_last_human_error() -> String {
        unsafe {
            let mut buf: *mut c_char = ptr::null_mut();
            let flags = FM_ALLOCATE_BUFFER | FM_IGNORE_INSERTS | FM_FROM_SYSTEM;

            // Attempt to get the human message.
            let len = format_message(
                flags,
                ptr::null(),
                get_last_error(),
                LANG_NEUTRAL,
                &mut buf,
                0,
                ptr::null_mut()
            );

            // If there is none, provide a default.
            if len == 0 || buf.is_null() {
                return format!("An unknown error occured");
            }

            // Convert buffer to a native Rust string.
            let msg = CStr::from_ptr(buf).to_string_lossy()
                                         .into_owned();

            // Free the buffer that was allocated by FormatMessage, assume success.
            local_free(buf as *mut c_void);

            return msg.trim().to_owned();
        }
    }
}

#[cfg(target_os = "linux")]
pub mod unix {
    use std::ffi::{CStr, c_void, c_char, c_int};

    pub const RTLD_LAZY: c_int = 1;

    #[link(name = "dl")]
    unsafe extern "C" {
        /// Loads a dynamic shared object.
        pub unsafe fn dlopen(library_filename: *const c_char,
                             flags: c_int) -> *mut c_void;
        
        /// Retrieves the address of an exported symbol in a library. 
        pub unsafe fn dlsym(library_module: *mut c_void,
                            symbol_name: *const c_char) -> *mut c_void;

        /// Frees a loaded dynamic shared object.
        pub unsafe fn dlclose(library_module: *mut c_void) -> c_int;

        /// Retrieves the last error.
        #[link_name = "dlerror"]
        unsafe fn dlerror_internal() -> *const c_char;
    }

    /// Retrieves the last error and converts to a Rust string.
    pub unsafe fn dlerror() -> String {
        unsafe {
            let error = dlerror_internal();

            // Convert to native Rust string.
            let msg = CStr::from_ptr(error).to_string_lossy()
                                           .into_owned();

            return msg;
        }
    }
}