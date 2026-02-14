---
description: workaround for Antigravity terminal hang bug where commands complete but remain listed as "running"
---

# Terminal Hang Workaround

## The Problem

There is a known Antigravity platform bug where `run_command` executes successfully in the terminal, but Antigravity's internal state fails to detect the completion. This causes:

1. `command_status` reports the command as still "running" indefinitely
2. The command appears stuck in the user's background process list
3. After enough zombie processes accumulate, new commands may fail to execute

This is NOT caused by the user's environment — it is a platform-side state synchronization bug.

## Detection

You are likely hitting this bug if:
- `command_status` keeps returning "running" for a command that should have finished
- The user tells you a command "completed" or "is stuck"
- You are waiting on build output (e.g., `cmake`, `ctest`) that never arrives

## Workaround: Use `read_terminal`

When `command_status` is stuck, you can bypass it by reading the terminal directly.

### Steps

1. **Ask the user for the terminal PID** — they can find this with `echo $$` in the terminal, or from the terminal UI
2. **Use `read_terminal`** to read the terminal buffer directly:
   ```
   read_terminal(Name="terminal-<PID>", ProcessID="<PID>")
   ```
3. **Parse the output** — the terminal buffer will contain the full command output including any build errors, test results, etc.
4. **Continue your work** — don't wait for `command_status`, use the terminal output you read directly

### Example

If the user says "PID is 1292":
```
read_terminal(Name="terminal-1292", ProcessID="1292")
```

This returns the full terminal scrollback buffer, including any completed command output that `command_status` failed to capture.

## Recovery

If the user dismisses stuck background tasks via the ✕ button in the UI, subsequent commands should work normally again. The user can inform you that a command completed, and you should proceed without re-running it.
