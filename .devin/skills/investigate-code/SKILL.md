---
name: investigate-code
description: Read-only code exploration and analysis of specific topics or areas in the codebase.
argument-hint: <topic or area to investigate>
triggers: ["user"]
allowed-tools: Read, Grep, FindFileByName
---

## Research Steps

1. **Search for relevant files:**
   - Use grep to search for keywords related to: $ARGUMENTS
   - Use find-file-by-name to locate relevant source files
   - Focus on core library and CLI source files

2. **Read relevant files:**
   - Read the most relevant files thoroughly
   - Understand the implementation details
   - Trace the call chain and data flow

3. **Analyze the code:**
   - Identify key functions and their responsibilities
   - Understand the data structures used
   - Note any patterns or abstractions

4. **Document findings:**
   - Write a comprehensive summary of how $ARGUMENTS works
   - Include specific file paths and line numbers for every claim
   - Note any concerns, edge cases, or areas that need attention
   - Suggest improvements if applicable

## Context

- Project structure: Cargo workspace with sig-maker-core and sig-maker-cli
- Core library: `crates/sig-maker-core/` (zero external dependencies)
- CLI binary: `crates/sig-maker-cli/`
- Format handling: `crates/sig-maker-core/src/formats/`

## Output Format

Provide a structured report with:
1. **Overview** - High-level description of the component
2. **Key Files** - List of relevant files with purposes
3. **Implementation Details** - How it works
4. **Data Flow** - How data moves through the system
5. **Concerns/Notes** - Any issues or areas for improvement
6. **References** - Specific file paths and line numbers

## Important Notes

- This skill is read-only - no code changes allowed
- Focus on understanding and documentation
- Use specific file references (e.g., `crates/sig-maker-core/src/io.rs:50-105`)
- Ask for clarification if the topic is ambiguous
