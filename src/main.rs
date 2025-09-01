use std::fs;
use std::ptr;
use std::process::exit;
use std::ffi::{CString, c_char};
use slexec::cli::Args;
use slexec::wrap::Library;

fn main() {
    // Gather args.
    let args = match Args::parse() {
        Ok(inner) => inner,
        Err(error) => {
            eprintln!("\x1b[1;31merror\x1b[0m: {}", error);
            exit(1);
        }
    };

    // Find full path to passed dynamic-link library.
    let path = match fs::canonicalize(args.module) {
        Ok(inner) => inner,
        Err(error) => {
            let human_error = error.to_string().to_lowercase();
            eprintln!("\x1b[1;31merror\x1b[0m: failed to locate file - {}", human_error);
            exit(1);
        }
    };

    // Open dynamic-link library.
    let lib = match Library::open(path.to_str().unwrap()) {
        Ok(inner) => inner,
        Err(error) => {
            let human_error = error.to_string().to_lowercase();
            eprintln!("\x1b[1;31merror\x1b[0m: failed to load library - {}", human_error);
            exit(1);
        }
    };

    // Locate function.
    let func = match lib.get(&args.function) {
        Ok(inner) => inner,
        Err(error) => {
            let human_error = error.to_string().to_lowercase();
            eprintln!("\x1b[1;31merror\x1b[0m: failed to find symbol - {}", human_error);
            exit(1);
        }
    };

    // Convert arguments to C compatible ones.
    let func_arg: Vec<CString> = args.arguments.iter()
                                               .map(|s| CString::new(s.clone()).unwrap())
                                               .collect();

    let mut func_arg_ptrs: Vec<*const c_char> = func_arg.iter()
                                                        .map(|s| s.as_ptr())
                                                        .collect();

    func_arg_ptrs.push(ptr::null());

    // Execute function.
    func(func_arg_ptrs.as_ptr());
}
