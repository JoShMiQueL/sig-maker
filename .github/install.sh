#!/bin/bash
# One-line installer for sig-maker CLI
# Usage: curl -fsSL https://raw.githubusercontent.com/JoShMiQueL/sig-maker/main/scripts/install/install.sh | bash

set -e

VERSION="${1:-latest}"
INSTALL_DIR="${2:-/usr/local/bin}"

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)
    case "$ARCH" in
      x86_64) PLATFORM="linux-x64" ;;
      aarch64) PLATFORM="linux-arm64" ;;
      armv7l) PLATFORM="linux-armv7" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  Darwin)
    case "$ARCH" in
      x86_64) PLATFORM="macos-intel" ;;
      arm64) PLATFORM="macos-arm64" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

# Download
if [ "$VERSION" = "latest" ]; then
  DOWNLOAD_URL="https://github.com/JoShMiQueL/sig-maker/releases/latest/download/sig-maker-cli-$PLATFORM"
else
  DOWNLOAD_URL="https://github.com/JoShMiQueL/sig-maker/releases/download/$VERSION/sig-maker-cli-$PLATFORM"
fi

echo "Downloading sig-maker CLI for $PLATFORM..."
curl -fsSL "$DOWNLOAD_URL" -o sig-maker-cli

# Install
echo "Installing to $INSTALL_DIR..."
chmod +x sig-maker-cli
sudo mv sig-maker-cli "$INSTALL_DIR/sig-maker"

# Verify
echo "Verifying installation..."
sig-maker --version

echo "✅ sig-maker CLI installed successfully!"
