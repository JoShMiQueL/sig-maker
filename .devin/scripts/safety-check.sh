#!/bin/bash
# Safety hook for Devin/Claude - blocks truly dangerous commands
# Receives JSON event data on stdin, exits with code 2 to block action

# Read JSON input
input=$(cat)

# Extract command if this is an exec tool call
# Try multiple methods for JSON parsing (cross-platform)
if command -v python3 &> /dev/null; then
    command=$(echo "$input" | python3 -c "import sys, json; data = json.load(sys.stdin); print(data.get('tool_input', {}).get('command', ''))" 2>/dev/null || echo "")
elif command -v python &> /dev/null; then
    command=$(echo "$input" | python -c "import sys, json; data = json.load(sys.stdin); print(data.get('tool_input', {}).get('command', ''))" 2>/dev/null || echo "")
else
    # Fallback: simple grep for command field
    command=$(echo "$input" | grep -o '"command":"[^"]*"' | cut -d'"' -f4 || echo "")
fi

# Dangerous git commands (affect history irreversibly)
dangerous_git_commands=(
    "git reset --hard"
    "git clean -fd"
    "git clean -f"
    "git branch -D"
    "git push --force"
    "git push -f"
    "git rebase"
    "git checkout --"
    "git restore --worktree"
    "git stash drop"
    "git stash clear"
    "git reflog expire"
    "git commit --no-verify"
    "git commit -n"
)

# Check for dangerous git commands
for cmd in "${dangerous_git_commands[@]}"; do
    if [[ "$command" == *"$cmd"* ]]; then
        echo "BLOCKED: Dangerous git command '$cmd'" >&2
        echo "This command can irreversibly modify git history or lose work." >&2
        echo "Safer alternatives:" >&2
        case "$cmd" in
            "git reset --hard")
                echo "  - Use 'git stash' to save changes first" >&2
                echo "  - Use 'git reset --soft' to keep changes in staging" >&2
                ;;
            "git clean -fd"|"git clean -f")
                echo "  - Use 'git clean -n' for dry run first" >&2
                ;;
            "git branch -D")
                echo "  - Use 'git branch -d' (only for merged branches)" >&2
                ;;
            "git push --force"|"git push -f")
                echo "  - Use 'git push --force-with-lease' instead" >&2
                ;;
            "git rebase")
                echo "  - Consider using merge instead" >&2
                ;;
            "git checkout --"|"git restore --worktree")
                echo "  - Use 'git stash' to save changes first" >&2
                ;;
            "git stash drop"|"git stash clear")
                echo "  - This permanently deletes stashed work" >&2
                ;;
            "git reflog expire")
                echo "  - This destroys your recovery safety net" >&2
                ;;
            "git commit --no-verify"|"git commit -n")
                echo "  - Remove the flag and fix what hooks report" >&2
                ;;
        esac
        exit 2
    fi
done

# Block commands that affect outside the project or destroy recovery
if [[ "$command" == *"rm -rf /"* ]] || \
   [[ "$command" == *"rm -rf ~"* ]] || \
   [[ "$command" == *"rm -rf .."* ]] || \
   [[ "$command" == *"rm -rf .git/"* ]]; then
    echo "BLOCKED: Dangerous deletion outside project or destroys git history" >&2
    echo "Command: $command" >&2
    echo "Files within the project can be recovered with git." >&2
    echo "This command affects things outside the project or destroys recovery capability." >&2
    exit 2
fi

# Windows equivalents
if [[ "$command" == *"rmdir /s /q C:\\"* ]] || \
   [[ "$command" == *"rmdir /s /q .."* ]] || \
   [[ "$command" == *"rmdir /s /q .git"* ]]; then
    echo "BLOCKED: Dangerous Windows deletion outside project or destroys git history" >&2
    echo "Command: $command" >&2
    exit 2
fi

# Allow the action
exit 0
