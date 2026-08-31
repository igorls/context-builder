# PROBLEM PACKET → Deep Think

> Fill this, attach `00-context.md` from context-builder, paste into Gemini Ultra Deep Think.

## Goal

One sentence: what “done” looks like.

## Context package

- Context file: `docs/handoffs/<slug>/00-context.md` (attached)
- Generated with: `context-builder ...` (paste exact command)
- Project / version / branch:

## Observed behavior

- What happens:
- What should happen:
- Repro (commands / steps):

## Evidence (minimal)

- Logs / test output (trimmed):
- Critical snippets only if not already in the context file:

## Constraints

- Must not break:
- Prefer (smallest fix / no new deps / etc.):
- Out of scope:

## Prior art (optional)

- Known fixed issues to avoid re-reporting:
- Previous AUTHORITY IDs:

## Required output shape

Reply **only** with an AUTHORITY document matching this structure:

1. **Verdict** (1–3 sentences)
2. **Non-goals**
3. **Findings** (numbered; severity; location by path + symbol; why it matters)
4. **Design** (approach + why not alternatives)
5. **Implementation plan** (ordered steps; each step: files/symbols + change + why)
6. **Invariants** (must still hold after the change)
7. **Verification** (exact commands + expected outcomes)
8. **Open questions** (only if blocking; else state assumptions)

Prefer the **smallest correct** change set. Depth over breadth. No full multi-file patches unless a step is trivial one-liners.
