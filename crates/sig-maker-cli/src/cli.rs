//! Command-line argument parsing

use sig_maker_core::formats::Format;
use std::io::IsTerminal;

/// CLI configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub input_file: String,
    pub to_format: Option<Format>, // None = show all formats
    pub output_file: Option<String>,
    pub verbose: bool,
    pub quiet: bool,
    pub check_only: bool,
}

/// Print error and exit
pub fn error_exit(msg: &str) -> ! {
    eprintln!("ERROR: {}", msg);
    pause_if_no_terminal();
    std::process::exit(1);
}

/// Pause and wait for Enter if not in a terminal (e.g., double-click on Windows)
pub fn pause_if_no_terminal() {
    // Skip pause in tests or CI
    if std::env::var("SIG_MAKER_NO_PAUSE").is_ok() {
        return;
    }

    // On Windows, check if we're running in a console that was created for us (double-click)
    #[cfg(windows)]
    {
        unsafe extern "system" {
            fn GetConsoleProcessList(list: *mut u32, count: u32) -> u32;
        }
        // If only 1 process is attached to the console, we own it (double-click)
        let mut pids = [0u32; 4];
        let count = unsafe { GetConsoleProcessList(pids.as_mut_ptr(), 4) };
        if count <= 1 {
            println!();
            println!("Press Enter to exit...");
            let _ = std::io::stdin().read_line(&mut String::new());
        }
    }
    #[cfg(not(windows))]
    {
        // On Unix, check if stdout is a TTY
        if !std::io::stdout().is_terminal() {
            println!();
            println!("Press Enter to exit...");
            let _ = std::io::stdin().read_line(&mut String::new());
        }
    }
}

impl Config {
    /// Parse command-line arguments
    pub fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();

        // Check for help (no input file needed)
        if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
            print_usage();
            pause_if_no_terminal();
            std::process::exit(0);
        }

        // Check for version (no input file needed)
        if args.len() > 1 && (args[1] == "-v" || args[1] == "--version") {
            println!("sig-maker {}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }

        // Check if we're in a pipe (no TTY)
        let is_pipe = !std::io::stdout().is_terminal();

        // If no arguments and in a pipe, use stdin
        let input_file = if args.len() <= 1 {
            if is_pipe {
                "-".to_string()
            } else {
                eprintln!("ERROR: No input file specified");
                print_usage();
                pause_if_no_terminal();
                std::process::exit(1);
            }
        } else {
            // Find the first non-flag argument
            let mut input = None;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--to" | "-t" => i += 2,
                    "--output" | "-o" => i += 2,
                    "--verbose" => i += 1,
                    "--quiet" | "-q" => i += 1,
                    "--check" | "-c" => i += 1,
                    _ => {
                        if input.is_none() && !args[i].starts_with('-') {
                            input = Some(args[i].clone());
                        }
                        i += 1;
                    }
                }
            }
            input.unwrap_or_else(|| {
                eprintln!("ERROR: No input file specified");
                print_usage();
                pause_if_no_terminal();
                std::process::exit(1);
            })
        };

        let mut to_format: Option<Format> = None;
        let mut output_file: Option<String> = None;
        let mut verbose = false;
        let mut quiet = false;
        let mut check_only = false;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--to" | "-t" => {
                    if i + 1 < args.len() {
                        match Format::from_string(&args[i + 1]) {
                            Some(f) => to_format = Some(f),
                            None => {
                                eprintln!("ERROR: Unknown format '{}'", args[i + 1]);
                                eprintln!(
                                    "Valid: ce, cpp, rust, ghidra, ida, x64dbg, python, json"
                                );
                                std::process::exit(1);
                            }
                        }
                        i += 2;
                    } else {
                        eprintln!("ERROR: --to requires an argument");
                        std::process::exit(1);
                    }
                }
                "--output" | "-o" => {
                    if i + 1 < args.len() {
                        output_file = Some(args[i + 1].to_string());
                        i += 2;
                    } else {
                        eprintln!("ERROR: --output requires an argument");
                        std::process::exit(1);
                    }
                }
                "--verbose" => {
                    verbose = true;
                    i += 1;
                }
                "--quiet" | "-q" => {
                    quiet = true;
                    i += 1;
                }
                "--check" | "-c" => {
                    check_only = true;
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }

        // Validate conflicting options
        if verbose && quiet {
            eprintln!("ERROR: --verbose and --quiet are mutually exclusive");
            std::process::exit(1);
        }

        if check_only && output_file.is_some() {
            eprintln!("ERROR: --check cannot be used with --output");
            std::process::exit(1);
        }

        Self {
            input_file,
            to_format,
            output_file,
            verbose,
            quiet,
            check_only,
        }
    }
}

fn print_usage() {
    println!("Sig-Maker - Multi-format signature/pattern converter and optimizer");
    println!();
    println!("Usage: sig-maker [OPTIONS] <INPUT_FILE>");
    println!("       echo 'pattern' | sig-maker [OPTIONS]");
    println!();
    println!("Arguments:");
    println!("  <INPUT_FILE>     File with AOB pattern(s) to analyze or convert");
    println!("                   Use '-' to read from stdin");
    println!();
    println!("Options:");
    println!("  -h, --help        Show this help message");
    println!("  --to <FORMAT>     Output format (default: show all formats)");
    println!("  -o, --output      Write output to file");
    println!("  -v, --version     Show version information");
    println!("  --verbose        Verbose output with detailed stats");
    println!("  -q, --quiet       Quiet mode (only output, no headers)");
    println!("  -c, --check       Validate pattern without converting");
    println!();
    println!("Formats:");
    println!("  ce, cheatengine   Cheat Engine (0? 00 00 ...)");
    println!("  cpp, c++          C++ with mask arrays");
    println!("  rust, rs          Rust with mask arrays");
    println!("  ghidra            Ghidra regex format");
    println!("  ida, idapro       IDA Pro format");
    println!("  x64dbg            x64dbg format (uses .)");
    println!("  python            Python regex pattern");
    println!("  json              JSON with pattern and mask");
    println!();
    println!("Examples:");
    println!("  sig-maker aobs.txt                  # Analyze AOBs, show all formats");
    println!("  sig-maker pattern.txt               # Convert pattern, show all formats");
    println!("  sig-maker --to rust aobs.txt        # Output only Rust format");
    println!("  sig-maker pattern.txt -o out.txt    # Save to file");
    println!("  sig-maker pattern.txt -v            # Verbose with stats");
    println!("  echo '0? 00 00' | sig-maker         # Read from stdin");
    println!("  sig-maker -                         # Read from stdin (pipe)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_string_simple() {
        // Test with custom args (we'll need to refactor Config::parse to accept args)
        // For now, just test the struct fields
        let config = Config {
            input_file: "test.txt".to_string(),
            to_format: Some(Format::CheatEngine),
            output_file: Some("output.txt".to_string()),
            verbose: true,
            quiet: false,
            check_only: false,
        };

        assert_eq!(config.input_file, "test.txt");
        assert_eq!(config.to_format, Some(Format::CheatEngine));
        assert_eq!(config.output_file, Some("output.txt".to_string()));
        assert!(config.verbose);
        assert!(!config.quiet);
        assert!(!config.check_only);
    }

    #[test]
    fn config_from_string_all_options() {
        let config = Config {
            input_file: "-".to_string(),
            to_format: Some(Format::Rust),
            output_file: None,
            verbose: false,
            quiet: true,
            check_only: true,
        };

        assert_eq!(config.input_file, "-");
        assert_eq!(config.to_format, Some(Format::Rust));
        assert!(config.output_file.is_none());
        assert!(!config.verbose);
        assert!(config.quiet);
        assert!(config.check_only);
    }

    #[test]
    fn pause_if_no_terminal_with_env_var() {
        // Set environment variable to skip pause
        unsafe {
            std::env::set_var("SIG_MAKER_NO_PAUSE", "1");
        }
        pause_if_no_terminal(); // Should not panic or hang
        unsafe {
            std::env::remove_var("SIG_MAKER_NO_PAUSE");
        }
    }

    #[test]
    fn format_variants() {
        assert_eq!(Format::from_string("ce"), Some(Format::CheatEngine));
        assert_eq!(Format::from_string("cpp"), Some(Format::Cpp));
        assert_eq!(Format::from_string("rust"), Some(Format::Rust));
        assert_eq!(Format::from_string("ghidra"), Some(Format::Ghidra));
        assert_eq!(Format::from_string("ida"), Some(Format::IdaPro));
        assert_eq!(Format::from_string("x64dbg"), Some(Format::X64dbg));
        assert_eq!(Format::from_string("python"), Some(Format::Python));
        assert_eq!(Format::from_string("json"), Some(Format::Json));
        assert_eq!(Format::from_string("invalid"), None);
    }

    #[test]
    fn format_name() {
        assert_eq!(Format::CheatEngine.name(), "Cheat Engine");
        assert_eq!(Format::Cpp.name(), "C++");
        assert_eq!(Format::Rust.name(), "Rust");
        assert_eq!(Format::Ghidra.name(), "Ghidra");
        assert_eq!(Format::IdaPro.name(), "IDA Pro");
        assert_eq!(Format::X64dbg.name(), "x64dbg");
        assert_eq!(Format::Python.name(), "Python");
        assert_eq!(Format::Json.name(), "JSON");
    }
}
