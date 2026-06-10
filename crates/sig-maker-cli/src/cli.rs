//! Command-line argument parsing

use sig_maker_core::formats::Format;
use std::io::IsTerminal;

/// CLI configuration
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

        // Check for help
        if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
            print_usage();
            pause_if_no_terminal();
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
            args[1].clone()
        };

        let mut to_format: Option<Format> = None;
        let mut output_file: Option<String> = None;
        let mut verbose = false;
        let mut quiet = false;
        let mut show_version = false;
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
                "--version" | "-v" => {
                    show_version = true;
                    i += 1;
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

        // Show version and exit
        if show_version {
            println!("sig-maker {}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
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
