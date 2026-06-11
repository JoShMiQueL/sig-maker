#!/bin/sh
# Hook script to warn before git commit
# Checks if the command is a git commit and reminds user to run verification

# Read JSON input from stdin
input=$(cat)

# Extract command using more robust JSON parsing
command=$(echo "$input" | sed -n 's/.*"command":"\([^"]*\)".*/\1/p')

# Check if this is a git commit command
if echo "$command" | grep -q "git commit"; then
  echo "⚠️  Remember to run @skills:verify-before-commit before committing!" >&2
  echo "This ensures fmt, clippy, build, and tests pass." >&2
  # Exit 0 to allow (just a warning), exit 2 to block
  exit 0
fi

exit 0
