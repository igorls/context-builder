#!/bin/sh
# Context Builder Installer
#
# Recommended: download the script from a release tag, read it, then run it.
# Piping a remote script straight into a shell trusts whatever the URL serves
# at that moment, so avoid that, and never let an automated agent do it.
#
#   curl -fsSLO https://raw.githubusercontent.com/igorls/context-builder/v0.11.0/install.sh
#   less install.sh
#   CONTEXT_BUILDER_VERSION=0.11.0 sh install.sh
#
# Environment:
#   CONTEXT_BUILDER_VERSION      release to install, e.g. 0.11.0 (default: latest)
#   CONTEXT_BUILDER_INSTALL_DIR  target directory (default: ~/.local/bin)
set -e

REPO="igorls/context-builder"
INSTALL_DIR="${CONTEXT_BUILDER_INSTALL_DIR:-$HOME/.local/bin}"

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
VERSION_REQUESTED="${CONTEXT_BUILDER_VERSION:-}"
if [ -n "$VERSION_REQUESTED" ]; then
  # Accept both "0.11.0" and "v0.11.0".
  VERSION_REQUESTED="${VERSION_REQUESTED#v}"
  BASE_URL="https://github.com/${REPO}/releases/download/v${VERSION_REQUESTED}"
else
  BASE_URL="https://github.com/${REPO}/releases/latest/download"
fi

echo "Installing context-builder ${VERSION_REQUESTED:-(latest)} for ${TARGET}..."

# Ensure install directory exists (user-local, no sudo needed)
mkdir -p "$INSTALL_DIR"
SUDO=""

# Download binary and checksums
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
echo "Downloading ${ARCHIVE}..."
curl -fsSL "${BASE_URL}/${ARCHIVE}" -o "$TMP/$ARCHIVE"
curl -fsSL "${BASE_URL}/SHA256SUMS" -o "$TMP/SHA256SUMS"

# Verify SHA256 checksum (fail closed — never install unverified binaries)
echo "Verifying checksum..."
EXPECTED="$(grep "$ARCHIVE" "$TMP/SHA256SUMS" | awk '{print $1}')"
if [ -z "$EXPECTED" ]; then
  echo "Error: Could not find checksum for $ARCHIVE in SHA256SUMS"
  echo "Aborting installation. Download the binary manually from:"
  echo "  https://github.com/$REPO/releases/latest"
  exit 1
fi

if command -v sha256sum >/dev/null 2>&1; then
  ACTUAL="$(sha256sum "$TMP/$ARCHIVE" | awk '{print $1}')"
elif command -v shasum >/dev/null 2>&1; then
  ACTUAL="$(shasum -a 256 "$TMP/$ARCHIVE" | awk '{print $1}')"
else
  echo "Error: No SHA256 verification tool found (need sha256sum or shasum)"
  echo "Aborting installation. Install one of these tools or download manually:"
  echo "  https://github.com/$REPO/releases/latest"
  exit 1
fi

if [ "$ACTUAL" != "$EXPECTED" ]; then
  echo "Error: Checksum verification failed!"
  echo "  Expected: $EXPECTED"
  echo "  Got:      $ACTUAL"
  echo "The download may be corrupted or tampered with."
  exit 1
fi
echo "✓ Checksum verified"

# Extract and install
tar xzf "$TMP/$ARCHIVE" -C "$TMP"
$SUDO mv "$TMP/context-builder" "$INSTALL_DIR/context-builder"
$SUDO chmod +x "$INSTALL_DIR/context-builder"
# $TMP is cleaned up automatically by the EXIT trap

# Verify
VERSION="$("$INSTALL_DIR/context-builder" --version 2>/dev/null || true)"
if [ -n "$VERSION" ]; then
  echo "✓ Installed: $VERSION"
else
  echo "✓ Installed to $INSTALL_DIR/context-builder"
  echo "  Make sure $INSTALL_DIR is in your PATH"
fi
