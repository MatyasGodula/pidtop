# Commands

## `pidtop start <session>`

Creates a named session directory under `$TMPDIR/pidtop/<session>/`.

```bash
pidtop start vm-monitor
```

Does not open a TUI. If the session already exists, this is a no-op.

---

## `pidtop add <session> [pid] [--name <label>]`

Adds a process to a session. The PID can be passed as an argument or piped via stdin.

```bash
# PID as argument
pidtop add my-session 1234 --name "nginx"

# PID via stdin
echo 1234 | pidtop add my-session --name "nginx"

# From a command that prints a PID
pgrep nginx | pidtop add my-session --name "nginx"

# Without a name (PID is used as the display label)
pidtop add my-session 1234
```

If the session does not exist, it is created automatically.

The `--name` flag sets a human-readable label stored inside the pidfile. If omitted, the PID itself is used as the display label.

---

## `pidtop show <session>`

Opens the TUI for a session.

```bash
pidtop show vm-monitor
```

Multiple instances can watch the same session simultaneously — all are read-only. Sparkline history is per-TUI-instance; a fresh `show` starts with empty sparklines.

The session must exist before calling `show`.

---

## `pidtop remove <session> <pid|name>`

Removes a process from a session by PID or by name.

```bash
pidtop remove my-session nginx    # by name
pidtop remove my-session 1234     # by PID
```

The corresponding pane disappears from any watching TUI instances.

---

## `pidtop list`

Lists all active sessions with their process counts.

```bash
$ pidtop list
vm-monitor (3 processes)
web-servers (1 processes)
```
