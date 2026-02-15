#!/usr/bin/env bash
# Demo script for context-builder v0.8.0 — records a clean asciinema demo
# Usage: asciinema rec --cols 100 --rows 32 --command="bash scripts/demo.sh" docs/demo.cast

set -e

# Simulate typing effect
type_cmd() {
    local cmd="$1"
    local delay="${2:-0.04}"
    printf '\033[1;32m❯\033[0m '
    for ((i=0; i<${#cmd}; i++)); do
        printf '%s' "${cmd:$i:1}"
        sleep "$delay"
    done
    sleep 0.4
    echo ""
}

# Section header
section() {
    echo ""
    printf '\033[1;35m━━━ %s ━━━\033[0m\n' "$1"
    sleep 0.8
}

# Copy our own source to a clean temp dir (avoids config overrides)
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO="$SCRIPT_DIR/.."
DEMO_DIR=$(mktemp -d)
PROJECT="$DEMO_DIR/context-builder"
mkdir -p "$PROJECT"

# Copy the real source code
cp -r "$REPO/src" "$PROJECT/src"
cp -r "$REPO/tests" "$PROJECT/tests"
cp "$REPO/Cargo.toml" "$PROJECT/"
cp "$REPO/Cargo.lock" "$PROJECT/" 2>/dev/null || true

cd "$PROJECT"

clear
echo ""
printf '\033[1;33m  ╔══════════════════════════════════════════════════════╗\033[0m\n'
printf '\033[1;33m  ║  ⚡ \033[1;37mcontext-builder\033[1;33m v0.8.0  — \033[0;36mTree-Sitter Edition\033[1;33m   ║\033[0m\n'
printf '\033[1;33m  ╚══════════════════════════════════════════════════════╝\033[0m\n'
printf '\033[2m    LLM context from your codebase, with AST superpowers\033[0m\n'
echo ""
sleep 1.5

# --- Demo 1: Preview ---
section "1. See what files will be included"
type_cmd "context-builder -f rs --preview -y"
context-builder -f rs --preview -y 2>/dev/null
sleep 1.5

# --- Demo 2: Full context ---
section "2. Generate full LLM context"
type_cmd "context-builder -f rs -o full.md -y"
context-builder -f rs -o full.md -y 2>/dev/null
sleep 1

type_cmd "head -45 full.md"
head -45 full.md
sleep 2.5

# --- Demo 3: Signatures ---
section "3. NEW: Extract signatures only (tree-sitter AST)"
type_cmd "context-builder -f rs --signatures -o sigs.md -y"
context-builder -f rs --signatures -o sigs.md -y 2>/dev/null
sleep 1

type_cmd "head -70 sigs.md"
head -70 sigs.md
sleep 3

# --- Demo 4: Size comparison ---
section "4. Compare: full context vs signatures"
type_cmd "wc -l full.md sigs.md"
wc -l full.md sigs.md
sleep 2.5

# --- Demo 5: Structure ---
section "5. NEW: Structural summary per file"
type_cmd "context-builder -f rs --structure --signatures -o overview.md -y"
context-builder -f rs --structure --signatures -o overview.md -y 2>/dev/null
sleep 1

type_cmd "head -80 overview.md"
head -80 overview.md
sleep 3

echo ""
printf '\033[1;32m✨ Your codebase is now LLM-ready.\033[0m\n'
printf '\033[2m   cargo install context-builder --features tree-sitter-all\033[0m\n'
sleep 3

# Cleanup
rm -rf "$DEMO_DIR"
