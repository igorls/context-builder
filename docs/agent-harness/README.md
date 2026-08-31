# Deep Think Authority Harness

> **Thesis:** The intelligence gap between Gemini Flash (agent harness) and Deep Think (Ultra web) is real. Closing it is not reverse-engineering Deep Think into the agent — it is **separating roles** and **injecting Deep Think as authoritative context** that Flash must obey.

This harness is the organized skill layer on top of [context-builder](../../README.md):

| Phase | Actor | Tooling |
| --- | --- | --- |
| **1. Package** | You / agent | `context-builder` → scoped, relevance-ordered context |
| **2. Reason** | Deep Think (Ultra web) | One-shot review over that context → **AUTHORITY** document |
| **3. Execute** | Antigravity / Flash agent | System prompt + AUTHORITY + tools on the real repo |
| **4. Verify** | Agent | Commands from AUTHORITY; stop on conflict |
| **5. Re-escalate** | You → Deep Think | Only when evidence contradicts AUTHORITY |

Your multi-model research already supports the packaging side: well-structured context lets one-shot Deep Think match agentic reviewers on *bug depth*. The missing half is the **return path** into a weaker tool-using model without letting it freestyle.

---

## What this does *not* claim

- Flash + good context ≠ Deep Think. You do **not** recover Deep Think’s novel reasoning mid-flight.
- You **do** recover: discipline, correct targets, ordered steps, fewer architecture thrash loops, higher verification fidelity.
- Dumping a 40-page freeform Deep Think essay into the agent **hurts**. Structure + budget beat volume.

---

## Core invariant (put this in every agent system prompt)

```
AUTHORITY documents produced by Deep Think are law for implementation.
If the codebase contradicts AUTHORITY, STOP and emit a CONFLICT packet.
Do not redesign, re-prioritize, or invent alternate architectures.
You may fix local mechanical details required to implement AUTHORITY.
```

---

## Files in this harness

| Path | Purpose |
| --- | --- |
| [`antigravity-system-prompt.md`](./antigravity-system-prompt.md) | Paste / mount as Antigravity (or any agent) system instructions |
| [`templates/01-problem.md`](./templates/01-problem.md) | Human → Deep Think intake |
| [`templates/02-authority.md`](./templates/02-authority.md) | Deep Think output shape (required) |
| [`templates/03-build.md`](./templates/03-build.md) | Human → agent task wrapper |
| [`templates/04-result.md`](./templates/04-result.md) | Agent → human completion report |
| [`templates/05-conflict.md`](./templates/05-conflict.md) | Agent stop condition when plan meets reality |

---

## Recommended on-disk layout (per project)

```
docs/handoffs/<YYYY-MM-DD-short-slug>/
  00-context.md          # from context-builder (or path to it)
  01-problem.md
  02-authority.md        # paste Deep Think response here (edited)
  03-build.md
  04-result.md
```

Keep handoffs **out of** the next context-builder snapshot if they bloat tokens (`-i handoffs` or output only under a ignored pattern). Prefer `docs/handoffs/` + ignore in `context-builder.toml` when regenerating review context.

---

## End-to-end recipe

### A. Package for Deep Think

```bash
# Preview size
context-builder -d /abs/path/to/project --token-count

# Full review context (adjust filters per language)
context-builder -d /abs/path/to/project \
  -f rs,toml,md \
  --max-tokens 120000 \
  -y -o docs/handoffs/2026-07-22-example/00-context.md
```

For large trees, prefer signatures first:

```bash
context-builder -d /abs/path/to/project \
  --signatures --structure --visibility public \
  -f rs -y -o docs/handoffs/.../00-context-signatures.md
```

### B. Deep Think pass

1. Upload `00-context.md` in Gemini Ultra.
2. Paste `templates/01-problem.md` filled in, plus the closing line that **forces AUTHORITY shape**.
3. Save response as `02-authority.md`. Human-edit: delete prose fluff; keep verdict, plan, verification.

### C. Antigravity / Flash pass

1. Ensure agent system prompt includes `antigravity-system-prompt.md`.
2. Open the **project root** (not the handoff folder alone).
3. Paste `03-build.md` with AUTHORITY inlined or `@`-referenced.
4. Agent implements; must return `04-result.md` shape.
5. On mismatch → `05-conflict.md` → back to Deep Think with new evidence only.

### D. After implementation (optional)

```bash
# Diff-only update for a second Deep Think verification pass
context-builder -d /abs/path/to/project -y --diff-only -o docs/handoffs/.../05-post-diff.md
```

---

## Attention budget for Flash (critical)

Rank content by **influence**, not chronological paste order:

1. **System prompt** (this harness) — identity + stop rules  
2. **AUTHORITY** (≤ ~2–4k tokens if possible) — law  
3. **Current task slice** — which step of the plan  
4. **Live repo via tools** — ground truth  
5. **Full context-builder dump** — *optional*; usually already consumed by Deep Think  

Do **not** re-inject the entire Deep Think *input* context into Flash unless the agent has no tool access. Flash should **read files** with tools, not re-read a 100k snapshot that Deep Think already distilled.

---

## When to re-escalate to Deep Think

| Signal | Action |
| --- | --- |
| AUTHORITY step impossible given real code | CONFLICT → Deep Think DEBUG |
| Tests fail for design reasons (not typos) | DEBUG packet + new evidence |
| Two plausible architectures appear mid-work | STOP — new PROBLEM, not agent freestyle |
| Mechanical compile/lint errors | Agent only |
| Flaky env / path issues | Agent only |

---

## Skill product direction (future CLI)

Possible context-builder features that encode this harness (not required to use it today):

1. `context-builder handoff init <slug>` — scaffold `docs/handoffs/<slug>/` from templates  
2. `context-builder handoff package` — write `00-context.md` into the active handoff  
3. `context-builder handoff distill` — (human-assisted) strip AUTHORITY to agent-sized budget  
4. Emit a ready-to-paste **BUILD** file from AUTHORITY + verification commands  

Until then, the markdown templates + Antigravity system prompt are the skill surface.
