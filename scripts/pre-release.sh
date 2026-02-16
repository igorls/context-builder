#!/usr/bin/env bash
# Pre-release gate — run before tagging a new version.
# Usage: ./scripts/pre-release.sh [--fix]
#
# With --fix: auto-fix formatting and regenerate demo gif
# Without:   dry-run, reports issues and exits non-zero if any fail
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO="$SCRIPT_DIR/.."
cd "$REPO"

FIX=false
[[ "${1:-}" == "--fix" ]] && FIX=true

ERRORS=0
WARNINGS=0

pass()  { printf '\033[1;32m  ✓ %s\033[0m\n' "$1"; }
fail()  { printf '\033[1;31m  ✗ %s\033[0m\n' "$1"; ERRORS=$((ERRORS + 1)); }
warn()  { printf '\033[1;33m  ⚠ %s\033[0m\n' "$1"; WARNINGS=$((WARNINGS + 1)); }
info()  { printf '\033[1;34m  ℹ %s\033[0m\n' "$1"; }
header(){ printf '\n\033[1;35m━━━ %s ━━━\033[0m\n' "$1"; }

# ─── Extract version from Cargo.toml ───
CARGO_VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
echo ""
printf '\033[1;37m  Pre-release checks for v%s\033[0m\n' "$CARGO_VERSION"

# ═══════════════════════════════════════
header "1. Formatting (cargo fmt)"
# ═══════════════════════════════════════
if $FIX; then
    cargo fmt --all
    pass "Formatted (cargo fmt --all)"
else
    if cargo fmt --all --check 2>/dev/null; then
        pass "Code is formatted"
    else
        fail "Code needs formatting — run with --fix or: cargo fmt --all"
    fi
fi

# ═══════════════════════════════════════
header "2. Version Consistency"
# ═══════════════════════════════════════

# SKILL.md frontmatter version
SKILL_VERSION=$(grep '^version:' SKILL.md | head -1 | awk '{print $2}')
if [[ "$SKILL_VERSION" == "$CARGO_VERSION" ]]; then
    pass "SKILL.md version: $SKILL_VERSION"
else
    fail "SKILL.md version ($SKILL_VERSION) ≠ Cargo.toml ($CARGO_VERSION)"
fi

# SKILL.md expected version string
SKILL_EXPECTED=$(grep 'expected:' SKILL.md | head -1 | sed 's/.*`\(.*\)`.*/\1/' | sed 's/[^0-9.]//g')
if [[ "$SKILL_EXPECTED" == "$CARGO_VERSION" ]]; then
    pass "SKILL.md expected version: $SKILL_EXPECTED"
else
    fail "SKILL.md expected version ($SKILL_EXPECTED) ≠ Cargo.toml ($CARGO_VERSION)"
fi

# CHANGELOG has an entry for this version
if grep -q "^## v${CARGO_VERSION}" CHANGELOG.md; then
    pass "CHANGELOG.md has v${CARGO_VERSION} entry"
else
    fail "CHANGELOG.md is missing v${CARGO_VERSION} entry"
fi

# demo.sh banner version
DEMO_VERSION=$(grep 'context-builder.*v[0-9]' scripts/demo.sh | head -1 | sed 's/.*v\([0-9.]*\).*/\1/')
if [[ "$DEMO_VERSION" == "$CARGO_VERSION" ]]; then
    pass "demo.sh banner version: $DEMO_VERSION"
else
    warn "demo.sh banner version ($DEMO_VERSION) ≠ Cargo.toml ($CARGO_VERSION)"
fi

# ═══════════════════════════════════════
header "3. Build"
# ═══════════════════════════════════════
if cargo build --features tree-sitter-all 2>&1; then
    pass "Build succeeded"
else
    fail "Build failed"
fi

# ═══════════════════════════════════════
header "4. Tests"
# ═══════════════════════════════════════
if cargo test --features tree-sitter-all 2>&1; then
    TEST_SUMMARY=$(cargo test --features tree-sitter-all 2>&1 | grep "^test result:" | tail -1)
    pass "Tests passed — $TEST_SUMMARY"
else
    fail "Tests failed"
fi

# ═══════════════════════════════════════
header "5. Demo GIF"
# ═══════════════════════════════════════
if command -v asciinema &>/dev/null && command -v agg &>/dev/null; then
    if $FIX; then
        info "Recording demo..."
        asciinema rec --cols 100 --rows 32 --command="bash scripts/demo.sh" docs/demo.cast --overwrite
        agg docs/demo.cast docs/demo.gif --theme monokai
        rm -f docs/demo.cast
        pass "Demo GIF regenerated at docs/demo.gif"
    else
        if [[ -f docs/demo.gif ]]; then
            DEMO_AGE=$(( ($(date +%s) - $(stat -c %Y docs/demo.gif 2>/dev/null || stat -f %m docs/demo.gif)) / 86400 ))
            if [[ $DEMO_AGE -gt 30 ]]; then
                warn "Demo GIF is ${DEMO_AGE} days old — consider regenerating with --fix"
            else
                pass "Demo GIF exists (${DEMO_AGE} days old)"
            fi
        else
            warn "Demo GIF not found — run with --fix to generate"
        fi
    fi
else
    if $FIX; then
        warn "asciinema or agg not installed — skipping demo GIF generation"
        info "Install: cargo install agg && pip install asciinema"
    else
        info "asciinema/agg not installed — demo GIF check skipped"
    fi
fi

# ═══════════════════════════════════════
header "6. Working Tree"
# ═══════════════════════════════════════
if [[ -z "$(git status --porcelain)" ]]; then
    pass "Working tree is clean"
else
    DIRTY=$(git status --porcelain | wc -l | tr -d ' ')
    warn "${DIRTY} uncommitted change(s) — commit before tagging"
    git status --short | head -10 | sed 's/^/    /'
fi

# ═══════════════════════════════════════
header "7. Tag Check"
# ═══════════════════════════════════════
if git tag -l "v${CARGO_VERSION}" | grep -q .; then
    warn "Tag v${CARGO_VERSION} already exists locally"
else
    pass "Tag v${CARGO_VERSION} is available"
fi

# ═══════════════════════════════════════
# Summary
# ═══════════════════════════════════════
echo ""
if [[ $ERRORS -gt 0 ]]; then
    printf '\033[1;31m  ✗ %d error(s), %d warning(s) — fix before tagging\033[0m\n\n' "$ERRORS" "$WARNINGS"
    exit 1
elif [[ $WARNINGS -gt 0 ]]; then
    printf '\033[1;33m  ⚠ %d warning(s) — review before tagging\033[0m\n\n' "$WARNINGS"
    exit 0
else
    printf '\033[1;32m  ✓ All checks passed — ready to tag v%s\033[0m\n' "$CARGO_VERSION"
    echo ""
    printf '    git tag v%s && git push && git push --tags\n\n' "$CARGO_VERSION"
    exit 0
fi
