---
name: debug-pattern
description: Debug pattern parsing issues with detailed analysis and troubleshooting.
argument-hint: <pattern or file path>
triggers: ["user"]
allowed-tools: Read, Grep, Write, Exec
---

## Analysis Steps

1. **Identify the issue:**
   - Read the pattern or file provided as argument
   - Determine which format the pattern should be (Cheat Engine, IDA, x64dbg, etc.)
   - Check if the pattern contains wildcards, brackets, or other format-specific syntax

2. **Examine the parser:**
   - Read `crates/sig-maker-core/src/formats/parser.rs`
   - Identify which parser should handle this pattern type
   - Check the parser logic for the specific format

3. **Test the parser:**
   - Run `cargo test --workspace` to see if any tests fail
   - If specific test fails, examine the test output
   - Run the CLI directly with the pattern to see the error

4. **Debug the issue:**
   - Add debug output if needed (temporarily)
   - Check if the pattern format is correctly detected
   - Verify the parser logic handles the pattern correctly
   - Check for edge cases in the pattern (e.g., malformed wildcards, invalid hex)

5. **Propose a fix:**
   - Identify the root cause
   - Propose a specific fix to the parser or detection logic
   - Explain why the fix will work
   - Ask for user confirmation before implementing

## Context

- Pattern to debug: $ARGUMENTS
- Current parser implementation: `crates/sig-maker-core/src/formats/parser.rs`
- Format detection logic: `crates/sig-maker-core/src/io.rs`

## Common Issues

- Pattern format not detected correctly (detected as MultipleAobs instead of SimplePattern)
- Wildcard syntax not supported (e.g., `?` vs `??` vs `?X`)
- Hex byte validation too strict or too lenient
- Brackets not parsed correctly for IDA/Ghidra formats

## Success Criteria

- Pattern is correctly parsed
- Tests pass
- CLI accepts the pattern without errors
