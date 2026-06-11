# sig-maker-cli Documentation Guidelines

This crate provides the command-line interface for sig-maker-core.

## Documentation Standards

When documenting sig-maker-cli:
- Focus on the CLI interface: arguments, options, flags
- Document how the CLI integrates with sig-maker-core
- Include usage examples for common operations
- Explain file I/O and format detection
- Document TTY/pipe detection behavior

## Key Files to Document

- `src/main.rs` - CLI entry point and argument parsing
- `src/cli.rs` - CLI argument definitions (if separate)
- `src/converter.rs` - Pattern conversion logic (if separate)
- `src/output.rs` - Output formatting and TTY detection

## CLI Interface

Document:
- Command-line arguments and their purposes
- Options (e.g., `--to <format>`)
- Flags (e.g., any boolean switches)
- How to use stdin vs file input
- How to use pipe output

## Usage Examples

Include examples for:
- Converting a file to a specific format
- Reading from stdin
- Piping output to another command
- Using format auto-detection
- Handling multiple patterns in a file

## File I/O

Document:
- How the CLI detects input format from file extension
- How it handles different file types
- Error handling for missing files or invalid formats

## TTY Detection

Document:
- How the CLI detects if output is to a TTY
- How output formatting changes when piped
- The "Press Enter to exit..." behavior on Windows
