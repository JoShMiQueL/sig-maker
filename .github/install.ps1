# One-line installer for sig-maker CLI (Windows)
# Usage: irm https://raw.githubusercontent.com/JoShMiQueL/sig-maker/main/scripts/install/install.ps1 | iex

param(
  [string]$Version = "latest",
  [string]$InstallDir = "$env:USERPROFILE\.local\bin"
)

$ErrorActionPreference = "Stop"

# Detect platform
$OS = [System.Environment]::OSVersion.Platform
$ARCH = [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture

if ($OS -ne "Win32NT") {
  Write-Error "This script is for Windows only"
  exit 1
}

switch ($ARCH) {
  "X64" { $Platform = "windows-x64" }
  "Arm64" { $Platform = "windows-arm64" }
  default {
    Write-Error "Unsupported architecture: $ARCH"
    exit 1
  }
}

# Download
if ($Version -eq "latest") {
  $DownloadUrl = "https://github.com/JoShMiQueL/sig-maker/releases/latest/download/sig-maker-cli-$Platform.exe"
} else {
  $DownloadUrl = "https://github.com/JoShMiQueL/sig-maker/releases/download/$Version/sig-maker-cli-$Platform.exe"
}

Write-Host "Downloading sig-maker CLI for $Platform..."
Invoke-WebRequest -Uri $DownloadUrl -OutFile sig-maker-cli.exe

# Create install directory
if (-not (Test-Path $InstallDir)) {
  New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Install
Write-Host "Installing to $InstallDir..."
Move-Item sig-maker-cli.exe "$InstallDir\sig-maker.exe"

# Add to PATH if not already
$PathEnv = [Environment]::GetEnvironmentVariable("Path", "User")
if ($PathEnv -notlike "*$InstallDir*") {
  [Environment]::SetEnvironmentVariable("Path", "$PathEnv;$InstallDir", "User")
  Write-Host "Added $InstallDir to user PATH. Please restart your terminal."
}

# Verify
Write-Host "Verifying installation..."
& "$InstallDir\sig-maker.exe" --version

Write-Host "✅ sig-maker CLI installed successfully!"
