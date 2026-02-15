#!/bin/sh
# Context Builder Installer
# Usage: curl -sSL https://raw.githubusercontent.com/igorls/context-builder/master/install.sh | sh
set -e

REPO="igorls/context-builder"
INSTALL_DIR="/usr/local/bin"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)   TARGET_OS="unknown-linux-gnu" ;;
  Darwin)  TARGET_OS="apple-darwin" ;;
  *)       echo "Error: Unsupported OS '$OS'. Please install manually from:"; echo "  https://github.com/$REPO/releases/latest"; exit 1 ;;
esac

case "$ARCH" in
  x86_64|amd64)  TARGET_ARCH="x86_64" ;;
  arm64|aarch64) TARGET_ARCH="aarch64" ;;
  *)             echo "Error: Unsupported architecture '$ARCH'. Please install manually from:"; echo "  https://github.com/$REPO/releases/latest"; exit 1 ;;
esac

TARGET="${TARGET_ARCH}-${TARGET_OS}"
URL="https://github.com/${REPO}/releases/latest/download/context-builder-${TARGET}.tar.gz"

echo "Installing context-builder for ${TARGET}..."

# Check write permissions
if [ ! -w "$INSTALL_DIR" ]; then
  echo "Note: $INSTALL_DIR requires elevated permissions."
  SUDO="sudo"
else
  SUDO=""
fi

# Download and install
TMP="$(mktemp -d)"
curl -sSL "$URL" -o "$TMP/context-builder.tar.gz"
tar xzf "$TMP/context-builder.tar.gz" -C "$TMP"
$SUDO mv "$TMP/context-builder" "$INSTALL_DIR/context-builder"
$SUDO chmod +x "$INSTALL_DIR/context-builder"
rm -rf "$TMP"

# Verify
VERSION="$(context-builder --version 2>/dev/null || true)"
if [ -n "$VERSION" ]; then
  echo "✓ Installed: $VERSION"
else
  echo "✓ Installed to $INSTALL_DIR/context-builder"
  echo "  Make sure $INSTALL_DIR is in your PATH"
fi
