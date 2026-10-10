#!/usr/bin/env bash
# Demo script for context-builder v0.11.0 — records a clean asciinema demo.
#
# Usage (needs `context-builder` built with --features tree-sitter-all on PATH):
#   asciinema rec --cols 100 --rows 32 --command="bash scripts/demo.sh" docs/demo.cast
#   agg --font-size 16 --speed 1.5 docs/demo.cast docs/demo.gif
#
# The script builds a throwaway fixture from this repo (src, tests, manifests,
# README) plus a lockfile, two fake images, a big data dump and a fake private
# key, so the v0.11.0 default skips have something to skip.

set -e

# Simulate typing effect
type_cmd() {
    local cmd="$1"
    local delay="${2:-0.03}"
    printf '\033[1;32m❯\033[0m '
    for ((i=0; i<${#cmd}; i++)); do
        printf '%s' "${cmd:$i:1}"
        sleep "$delay"
    done
    sleep 0.4
    echo ""
}

# Section header on a clean screen
section() {
    clear
    printf '\033[1;35m━━━ %s ━━━\033[0m\n' "$1"
    sleep 0.8
}

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO="$SCRIPT_DIR/.."
DEMO_DIR=$(mktemp -d)
PROJECT="$DEMO_DIR/context-builder"
mkdir -p "$PROJECT/assets"

# Real source and manifests
cp -r "$REPO/src" "$REPO/tests" "$PROJECT/"
cp "$REPO/Cargo.toml" "$REPO/Cargo.lock" "$REPO/README.md" "$PROJECT/"

# Things v0.11.0 skips by default
printf '\x89PNG\r\n\x1a\n\0\0\0\rIHDR' > "$PROJECT/logo.png"
printf '\x89PNG\r\n\x1a\n\0\0\0\rIHDR' > "$PROJECT/assets/banner.png"
head -c 400000 /dev/urandom | base64 > "$PROJECT/data_dump.json"
printf -- '-----BEGIN OPENSSH PRIVATE KEY-----\nnot-a-real-key\n' > "$PROJECT/id_rsa"

cd "$PROJECT"

clear
echo ""
printf '\033[1;33m  ╔══════════════════════════════════════════════════════╗\033[0m\n'
printf '\033[1;33m  ║  ⚡ \033[1;37mcontext-builder\033[1;33m v0.11.0  — \033[0;36mQuiet Defaults\033[1;33m        ║\033[0m\n'
printf '\033[1;33m  ╚══════════════════════════════════════════════════════╝\033[0m\n'
printf '\033[2m    LLM context from your codebase, minus the noise\033[0m\n'
echo ""
sleep 1.5

# --- 1: Preview shows what is skipped ---
section "1. Preview: lockfiles, assets, big files and secrets stay out"
type_cmd "context-builder --preview 2>&1 | sed -n 1,18p"
context-builder --preview 2>&1 | sed -n 1,18p || true
sleep 3

# --- 2: Full context, README first, Skipped section ---
section "2. Full context: root README first, skips are listed"
type_cmd "context-builder -o full.md 2>&1 | sed -n 1,3p"
context-builder -o full.md 2>&1 | sed -n 1,3p || true
sleep 1.5
type_cmd "grep -A5 '^## Skipped' full.md"
grep -A5 '^## Skipped' full.md
sleep 1.5
type_cmd "grep '^### File' full.md | head -4"
grep '^### File' full.md | head -4
sleep 3

# --- 3: Opt back in ---
section "3. Opt back in: --include-lockfiles, --file-metadata"
type_cmd "context-builder -f toml --include-lockfiles --file-metadata -o meta.md >/dev/null 2>&1"
context-builder -f toml --include-lockfiles --file-metadata -o meta.md >/dev/null 2>&1
sleep 0.5
type_cmd "grep -A3 '### File: \`Cargo.lock\`' meta.md"
grep -A3 '### File: `Cargo.lock`' meta.md
sleep 3

# --- 4: Signatures ---
section "4. Extract signatures only (tree-sitter AST)"
type_cmd "context-builder -f rs --signatures -o sigs.md >/dev/null 2>&1"
context-builder -f rs --signatures -o sigs.md >/dev/null 2>&1
type_cmd "grep -A14 '### File: \`src/lib.rs\`' sigs.md"
grep -A14 '### File: `src/lib.rs`' sigs.md
sleep 3

# --- 5: Size comparison ---
section "5. Compare: full context vs signatures"
type_cmd "wc -c full.md sigs.md"
wc -c full.md sigs.md
sleep 3

# --- 6: Structure ---
section "6. Structural summary per file"
type_cmd "context-builder -f rs --structure --signatures -o overview.md >/dev/null 2>&1"
context-builder -f rs --structure --signatures -o overview.md >/dev/null 2>&1
type_cmd "grep -m1 -A12 '^\\*\\*Structure' overview.md"
grep -m1 -A12 '^\*\*Structure' overview.md || true
sleep 3

echo ""
printf '\033[1;32m✨ Your codebase is now LLM-ready.\033[0m\n'
printf '\033[2m   cargo install context-builder --features tree-sitter-all\033[0m\n'
sleep 3

# Cleanup
rm -rf "$DEMO_DIR"
