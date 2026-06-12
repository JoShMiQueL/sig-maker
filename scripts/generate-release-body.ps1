#!/usr/bin/env pwsh

# Generate release body with Full Changelog link
# Usage: .\scripts\generate-release-body.ps1 <previous_tag> <current_tag> [output_file]

param(
    [Parameter(Mandatory=$true)]
    [string]$PreviousTag,
    
    [Parameter(Mandatory=$true)]
    [string]$CurrentTag,
    
    [Parameter(Mandatory=$false)]
    [string]$OutputFile = "RELEASE_BODY.md"
)

# Generate changelog using cliff.toml
$tempFile = New-TemporaryFile
git cliff --config cliff.toml "$PreviousTag..$CurrentTag" --output $tempFile --offline

# Read the generated changelog
$content = Get-Content $tempFile -Raw

# Remove the header (keep only body content)
$headerEnd = $content.IndexOf("## [")
if ($headerEnd -gt 0) {
    $content = $content.Substring($headerEnd)
}

# Remove the footer (links at the end) - look for pattern [vx.x.x]: at start of line
$lines = $content -split "`n"
$footerLineIndex = -1

for ($i = 0; $i -lt $lines.Count; $i++) {
    if ($lines[$i] -match "^\[v\d+\.\d+\.\d+.*\]:") {
        $footerLineIndex = $i
        break
    }
}

if ($footerLineIndex -gt 0) {
    $content = ($lines[0..($footerLineIndex-1)] -join "`n").TrimEnd()
}

# Add Full Changelog link at the end
$fullChangelogLink = "`n`n**Full Changelog**: https://github.com/JoShMiQueL/sig-maker/compare/$PreviousTag...$CurrentTag"
$content = $content + $fullChangelogLink

# Write to output file
Set-Content -Path $OutputFile -Value $content -NoNewline

# Clean up
Remove-Item $tempFile

Write-Host "Release body generated: $OutputFile"
