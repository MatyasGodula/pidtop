# TUI Reference

## Layout

Panes tile in a fixed grid, filling left-to-right and wrapping to the next row. The number of columns adjusts automatically based on terminal width (minimum 40 characters per pane). No manual layout management.

## Pane contents

Each pane displays:

| Field | Description |
|-------|-------------|
| Label | Human-readable name if set, PID otherwise |
| PID | Process ID |
| Status | `running`, `sleeping`, `zombie`, or `dead` |
| CPU % | Current CPU usage |
| Memory | Resident set size (RSS) |
| Uptime | Time since this TUI instance first saw the process |
| Sparkline | CPU usage over the last N seconds (configurable, default 60) |

## Keybindings

| Key | Action |
|-----|--------|
| `q` | Quit |
| `d` | Dismiss selected pane (removes the pidfile too) |
| `r` | Force refresh process stats |
| `arrow_up` | Move selection up one row |
| `arrow_down` | Move selection down one row |
| `arrow_left` | Move selection left |
| `arrow_right` | Move selection right |

## Dead processes

When a process dies or its pidfile is removed:

- The pane border turns red (or whatever `border_dead` is set to in your theme)
- The status shows `dead` with time since death
- The sparkline grays out
- The pane stays visible until dismissed with `d`

Dead panes are kept visible intentionally — they're informative. Dismissing a dead pane with `d` also deletes the pidfile from the session directory.

## Change detection

The TUI watches the session directory using filesystem events (`inotify` on Linux, `FSEvents`/`kqueue` on macOS):

- **File created** in session directory: new pane appears
- **File deleted** from session directory: pane marked as dead

No polling for file changes — updates are instant.

## Empty sessions

Opening a TUI on an empty session shows a help message with instructions on how to add processes. Panes appear automatically as pidfiles are created.
