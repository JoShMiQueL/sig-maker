#!/usr/bin/env sh
# sig-maker-cli installer for Linux and macOS
# Usage: curl -fsSL https://github.com/JoShMiQueL/sig-maker/releases/latest/download/install.sh | sh
# Or with a specific version:
# curl -fsSL https://github.com/JoShMiQueL/sig-maker/releases/latest/download/install.sh | sh -s -- --version v0.1.3

set -eu

REPO="JoShMiQueL/sig-maker"
BINARY_NAME="sig-maker-cli"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
VERSION="${1:-}"

# Parse --version flag
while [ $# -gt 0 ]; do
    case "$1" in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --install-dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        *)
            shift
            ;;
    esac
done

# Detect OS and arch
detect_target() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux)
            case "$ARCH" in
                x86_64)  echo "linux-x64" ;;
                aarch64) echo "linux-arm64" ;;
                *)       echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
            esac
            ;;
        Darwin)
            case "$ARCH" in
                x86_64)  echo "macos-x64" ;;
                arm64)   echo "macos-arm64" ;;
                *)       echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
            esac
            ;;
        *)
            echo "Unsupported OS: $OS" >&2
            exit 1
            ;;
    esac
}

# Get latest version from GitHub API
get_latest_version() {
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
            | grep '"tag_name"' \
            | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/'
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "https://api.github.com/repos/$REPO/releases/latest" \
            | grep '"tag_name"' \
            | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/'
    else
        echo "curl or wget is required" >&2
        exit 1
    fi
}

# Download file
download() {
    URL="$1"
    DEST="$2"
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$URL" -o "$DEST"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO "$DEST" "$URL"
    else
        echo "curl or wget is required" >&2
        exit 1
    fi
}

echo "sig-maker-cli installer"
echo "========================"

TARGET="$(detect_target)"
echo "Target: $TARGET"

if [ -z "$VERSION" ]; then
    echo "Fetching latest version..."
    VERSION="$(get_latest_version)"
fi
echo "Version: $VERSION"

ASSET="sig-maker-cli-$TARGET"
URL="https://github.com/$REPO/releases/download/$VERSION/$ASSET"
TMP="$(mktemp)"

echo "Downloading from $URL ..."
download "$URL" "$TMP"
chmod +x "$TMP"

mkdir -p "$INSTALL_DIR"
mv "$TMP" "$INSTALL_DIR/$BINARY_NAME"
echo "Installed to $INSTALL_DIR/$BINARY_NAME"

# Add to PATH hint if needed
case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
        echo ""
        echo "NOTE: $INSTALL_DIR is not in your PATH."
        echo "Add the following to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
        ;;
esac

echo ""
echo "Done! Run 'sig-maker-cli --help' to get started."
