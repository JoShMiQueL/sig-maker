#!/bin/sh
# Sets up the git hooks for this repository.
# Run once after cloning: sh .githooks/setup.sh

git config core.hooksPath .githooks
echo "Git hooks configured. Using .githooks/ directory."
