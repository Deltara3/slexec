use std::env;
use std::ffi::CString;
use std::process::exit;

/// Argument parsing struct.
pub struct Args {
    pub module: String,
    pub function: String,
    pub arguments: Option<Vec<CString>>,
    pub debug: bool
}

impl Args {
    /// Parses command line arguments.
    pub fn parse() -> Result<Self, String> {
        let mut args= env::args().skip(1);
        
        // Temporary variables.
        let should_print                   = args.len() == 0;
        let mut help                       = false;
        let mut debug                      = false;
        let mut module: Option<String>     = None;
        let mut function: Option<String>   = None;
        let mut pass: Option<Vec<CString>> = None;

        // Iterate over arguments.
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" => {
                    help = true;
                },
                "-m" | "--module" => {
                    match Args::parse_value("-m/--module", args.next()) {
                        Ok(value) => {
                            module = Some(value);
                        },
                        Err(error) => {
                            return Err(error);
                        }
                    }
                },
                "-f" | "--function" => {
                    match Args::parse_value("-f/--function", args.next()) {
                        Ok(value) => {
                            function = Some(value);
                        },
                        Err(error) => {
                            return Err(error);
                        }
                    }
                },
                "-p" | "--pass" => {
                    match Args::parse_value("-p/--pass", args.next()) {
                        Ok(value) => {
                            let data = value.split(',')
                                            .map(|s| CString::new(s).unwrap())
                                            .collect();

                            pass = Some(data);
                        },
                        Err(error) => {
                            return Err(error);
                        }
                    }
                },
                "-d" | "--debug" => {
                    debug = true;
                },
                _ => {
                    return Err(format!("argument \x1b[33m{}\x1b[0m unknown", arg));
                }

            }
        }

        if help == true || should_print {
            Args::print_help(include_str!("help.txt").to_string());
            exit(0);
        } else {
            // Check if required arguments are none.
            let required = [
                (&module, "-m/--module"),
                (&function, "-f/--function")
            ];

            for arg in required {
                if arg.0.is_none() {
                    return Err(format!("required argument \x1b[33m{}\x1b[0m is missing", arg.1))
                }
            }
        
            Ok(Self {
                module: module.unwrap(),
                function: function.unwrap(),
                arguments: pass,
                debug: debug
            })
        }
    }

    /// Parses a key-value argument pair.
    fn parse_value(arg: &str, value: Option<String>) -> Result<String, String> {
        if let Some(inner) = value {
            // Return error if another flag is found.
            if inner.starts_with('-') {
                return Err(format!("argument \x1b[33m{}\x1b[0m requires a value", arg));
            }

            // If not, return the value.
            Ok(inner)
        } else {
            // Return error if no value is provided.
            return Err(format!("argument \x1b[33m{}\x1b[0m requires a value", arg));
        }
    }

    /// Prints help information.
    fn print_help(text: String) {
        // Fill in placeholder text, not the best solution, but oh well.
        let new_text = text.replace("{name}", env!("CARGO_PKG_NAME"))
                           .replace("{version}", env!("CARGO_PKG_VERSION"))
                           .replace("{description}", env!("CARGO_PKG_DESCRIPTION"))
                           .replace("[yellow]", "\x1b[33m")
                           .replace("[magenta]", "\x1b[35m")
                           .replace("[green]", "\x1b[32m")
                           .replace("[cyan]", "\x1b[36m")
                           .replace("[blue]", "\x1b[34m")
                           .replace("[reset]", "\x1b[0m");

        println!("{}", new_text);
    }
}