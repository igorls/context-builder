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
ARCHIVE="context-builder-${TARGET}.tar.gz"
BASE_URL="https://github.com/${REPO}/releases/latest/download"

echo "Installing context-builder for ${TARGET}..."

# Check write permissions
if [ ! -w "$INSTALL_DIR" ]; then
  echo "Note: $INSTALL_DIR requires elevated permissions."
  SUDO="sudo"
else
  SUDO=""
fi

# Download binary and checksums
TMP="$(mktemp -d)"
echo "Downloading ${ARCHIVE}..."
curl -sSL "${BASE_URL}/${ARCHIVE}" -o "$TMP/$ARCHIVE"
curl -sSL "${BASE_URL}/SHA256SUMS" -o "$TMP/SHA256SUMS"

# Verify SHA256 checksum
echo "Verifying checksum..."
EXPECTED="$(grep "$ARCHIVE" "$TMP/SHA256SUMS" | awk '{print $1}')"
if [ -z "$EXPECTED" ]; then
  echo "Warning: Could not find checksum for $ARCHIVE in SHA256SUMS"
  echo "Proceeding without verification..."
else
  if command -v sha256sum >/dev/null 2>&1; then
    ACTUAL="$(sha256sum "$TMP/$ARCHIVE" | awk '{print $1}')"
  elif command -v shasum >/dev/null 2>&1; then
    ACTUAL="$(shasum -a 256 "$TMP/$ARCHIVE" | awk '{print $1}')"
  else
    echo "Warning: No SHA256 tool found, skipping verification"
    ACTUAL="$EXPECTED"
  fi

  if [ "$ACTUAL" != "$EXPECTED" ]; then
    echo "Error: Checksum verification failed!"
    echo "  Expected: $EXPECTED"
    echo "  Got:      $ACTUAL"
    echo "The download may be corrupted or tampered with."
    rm -rf "$TMP"
    exit 1
  fi
  echo "✓ Checksum verified"
fi

# Extract and install
tar xzf "$TMP/$ARCHIVE" -C "$TMP"
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
