//! Command-line argument parsing

use crate::formats::Format;

/// CLI configuration
pub struct Config {
    pub input_file: String,
    pub to_format: Option<Format>, // None = show all formats
}

impl Config {
    /// Parse command-line arguments
    pub fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();

        // Check for help or no arguments
        if args.len() <= 1 || args[1] == "-h" || args[1] == "--help" {
            print_usage();
            pause_if_no_terminal();
            std::process::exit(0);
        }

        let mut to_format: Option<Format> = None;
        let mut input_file: Option<String> = None;

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
                arg => {
                    if input_file.is_none() && !arg.starts_with("-") {
                        input_file = Some(arg.to_string());
                    }
                    i += 1;
                }
            }
        }

        Self {
            input_file: input_file.unwrap_or_else(|| {
                eprintln!("ERROR: No input file specified");
                print_usage();
                pause_if_no_terminal();
                std::process::exit(1);
            }),
            to_format,
        }
    }
}

fn print_usage() {
    println!("Sig-Maker - Multi-format signature/pattern converter and optimizer");
    println!();
    println!("Usage: sig-maker [OPTIONS] [INPUT_FILE]");
    println!();
    println!("Arguments:");
    println!("  [INPUT_FILE]  File with AOB pattern(s) (default: aobs.txt)");
    println!();
    println!("Options:");
    println!("  --to <FORMAT>     Output format (default: show all formats)");
    println!("  -o, --output      Write output to file");
    println!("  -v, --verbose     Verbose output with detailed stats");
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
}

/// Pause and wait for Enter if running without a proper terminal (e.g. double-click on Windows)
fn pause_if_no_terminal() {
    use std::io::{Read, stdin};

    // Check if stdin is a terminal; if not attached to a pipe, pause
    if atty_check() {
        println!();
        println!("Press Enter to exit...");
        let _ = stdin().read(&mut [0u8]);
    }
}

/// Returns true if we should pause (i.e. likely launched via double-click)
fn atty_check() -> bool {
    // On Windows, check if we're running in a console that was created for us
    #[cfg(windows)]
    {
        unsafe extern "system" {
            fn GetConsoleProcessList(list: *mut u32, count: u32) -> u32;
        }
        // If only 1 process is attached to the console, we own it (double-click)
        let mut pids = [0u32; 4];
        let count = unsafe { GetConsoleProcessList(pids.as_mut_ptr(), 4) };
        count <= 1
    }
    #[cfg(not(windows))]
    {
        false
    }
}
