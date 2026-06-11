---
name: test-integration
description: Run integration tests with detailed output and analysis of failures.
triggers: ["user"]
allowed-tools: Exec, Read, Write
---

## Testing Steps

1. **Run all tests:**
   - Execute `cargo test --workspace --verbose`
   - Capture all output including test names and results

2. **Analyze results:**
   - Check if all tests passed
   - If any tests failed, identify which ones
   - Examine the failure messages and stack traces

3. **Debug failures:**
   - Read the failing test file
   - Understand what the test is checking
   - Identify why it failed
   - Propose a fix if the issue is in the code

4. **Report findings:**
   - Summarize test results
   - List any failures with detailed analysis
   - Suggest next steps

## Context

- Integration tests location: `crates/sig-maker-cli/tests/integration.rs`
- Unit tests location: `crates/sig-maker-core/tests/` and inline in source files
- Test patterns: `"0? 00 00 00 01 00 00 00 E? FF FF FF 00"` (Cheat Engine format)

## Common Test Failures

- Pattern parsing errors (invalid format, unsupported wildcards)
- Exit code mismatches in integration tests
- File I/O issues in tests
- TTY detection issues (pause functionality)

## Success Criteria

- All tests pass
- No panics or errors
- Exit codes match expected values
- Output contains expected content

## Notes

- Integration tests require the debug binary to be built
- Tests use `SIG_MAKER_NO_PAUSE=1` to disable pause functionality
- Always run tests after making changes to parser or CLI logic
