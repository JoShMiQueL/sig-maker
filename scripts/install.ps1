# sig-maker-cli installer for Windows
# Usage: irm https://github.com/JoShMiQueL/sig-maker/releases/latest/download/install.ps1 | iex

param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:USERPROFILE\.local\bin"
)

$ErrorActionPreference = "Stop"
$Repo = "JoShMiQueL/sig-maker"
$BinaryName = "sig-maker-cli.exe"
$AssetName = "sig-maker-cli-x86_64-pc-windows-msvc.zip"

function Get-LatestVersion {
    $release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
    return $release.tag_name
}

function Get-DownloadUrl {
    param([string]$Tag)
    return "https://github.com/$Repo/releases/download/$Tag/$AssetName"
}

function Add-ToPath {
    param([string]$Dir)
    $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($userPath -notlike "*$Dir*") {
        [Environment]::SetEnvironmentVariable("PATH", "$userPath;$Dir", "User")
        $env:PATH += ";$Dir"
        Write-Host "  Added $Dir to PATH (restart shell to take effect)"
    }
}

Write-Host "sig-maker-cli installer"
Write-Host "========================"

# Resolve version
if ($Version -eq "latest") {
    Write-Host "Fetching latest version..."
    $Version = Get-LatestVersion
}
Write-Host "Version: $Version"

# Download
$Url = Get-DownloadUrl $Version
$TmpFile = Join-Path $env:TEMP $AssetName
Write-Host "Downloading from $Url ..."
Invoke-WebRequest -Uri $Url -OutFile $TmpFile -UseBasicParsing

# Extract
$TmpDir = Join-Path $env:TEMP "sig-maker-install-$(Get-Random)"
New-Item -ItemType Directory -Path $TmpDir | Out-Null
Write-Host "Extracting..."
Expand-Archive -Path $TmpFile -DestinationPath $TmpDir -Force

# Install
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}
$Dest = Join-Path $InstallDir $BinaryName
Move-Item -Path (Join-Path $TmpDir "sig-maker-cli.exe") -Destination $Dest -Force
Write-Host "Installed to $Dest"

# Cleanup
Remove-Item -Path $TmpFile -Force
Remove-Item -Path $TmpDir -Recurse -Force

# PATH
Add-ToPath $InstallDir

Write-Host ""
Write-Host "Done! Run 'sig-maker-cli --help' to get started."
Write-Host "You may need to restart your terminal for PATH changes to take effect."
