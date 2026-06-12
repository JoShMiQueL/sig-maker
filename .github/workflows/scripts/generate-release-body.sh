#!/usr/bin/env bash

# Generate release body with Full Changelog link
# Usage: ./scripts/generate-release-body.sh <previous_tag> <current_tag> [output_file]

set -e

PREVIOUS_TAG="${1}"
CURRENT_TAG="${2}"
OUTPUT_FILE="${3:-RELEASE_BODY.md}"

if [ -z "$PREVIOUS_TAG" ] || [ -z "$CURRENT_TAG" ]; then
    echo "Usage: $0 <previous_tag> <current_tag> [output_file]"
    exit 1
fi

# Generate changelog using cliff.toml
TEMP_FILE=$(mktemp)
git-cliff --config cliff.toml "${PREVIOUS_TAG}..${CURRENT_TAG}" --output "$TEMP_FILE" --offline

# Read the generated changelog
CONTENT=$(cat "$TEMP_FILE")

# Remove the header (keep only body content)
CONTENT=$(echo "$CONTENT" | sed -n '/^## \[/,$p')

# Remove the footer (links at the end) - look for pattern [vx.x.x]: at start of line
CONTENT=$(echo "$CONTENT" | sed '/^\[v[0-9]/q')

# Add Full Changelog link at the end
FULL_CHANGELOG_LINK=$'\n\n'"**Full Changelog**: https://github.com/JoShMiQueL/sig-maker/compare/${PREVIOUS_TAG}...${CURRENT_TAG}"
CONTENT="${CONTENT}${FULL_CHANGELOG_LINK}"

# Write to output file
echo -n "$CONTENT" > "$OUTPUT_FILE"

# Clean up
rm "$TEMP_FILE"

echo "Release body generated: $OUTPUT_FILE"
