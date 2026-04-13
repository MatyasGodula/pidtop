# pidtop

A lightweight, session-based process monitor TUI for Linux and macOS, written in Rust.
Like a pitstop — you pull in, check what's running, pull out.

---

## Design Philosophy

- **Read-only** — pidtop never touches the processes it watches. No kill, no restart.
- **Filesystem as protocol** — sessions are directories, watched processes are files. No sockets, no daemons, no custom IPC.
- **Composable** — the interface is stdin and the filesystem. Any shell script or program can integrate without knowing pidtop exists.
- **Ephemeral by default** — session state lives in `$TMPDIR`, gone on reboot.
- **Cross-platform** — Linux and macOS via the `notify` and `sysinfo` crates.

---

## Components

### `pidtop start <session>`
Creates a named session directory under `$TMPDIR/pidtop/<session>/`. Does not open a TUI.

### `pidtop add <session> [--name <label>]`
Reads a PID from stdin, writes a pidfile into the session directory. The optional `--name` flag sets a human-readable label stored inside the pidfile. If omitted, the PID is used as the display label. If the session does not exist, creates it.

### `pidtop show <session>`
Opens a TUI window watching the session directory. Multiple instances can watch the same session simultaneously — all are read-only. Sparkline history is per-TUI-instance, not per-session — a fresh `show` starts with empty sparklines.

### `pidtop remove <session> <pid|name>`
Removes a pidfile from the session directory by PID or name. The corresponding pane disappears from any watching TUI instances.

### `pidtop list`
Lists all active sessions under `$TMPDIR/pidtop/`.

---

## Session Layout

```
$TMPDIR/pidtop/
  vm-monitor/
    1234.pid       ← contains "ubuntu"
    5678.pid       ← contains "alpine"
    9012.pid       ← contains ""  (no label, PID used as display label)
  web-servers/
    3456.pid       ← contains "nginx"
```

Filename is always the PID — unambiguous, no collisions. Contents are the optional human-readable label. The TUI shows the label if present, PID if not.

---

## Composition

Designed for startup scripts rather than hand-rolling:

```bash
pidtop start vm-monitor
vzvm start ubuntu | pidtop add vm-monitor --name ubuntu
vzvm start alpine | pidtop add vm-monitor --name alpine
pidtop show vm-monitor
```

Name is always optional — any program that prints a PID composes naturally:

```bash
some-other-daemon --start | pidtop add my-session
echo $! | pidtop add my-session          # background shell process
pgrep nginx | pidtop add my-session --name nginx
```

---

## TUI Behavior

- Opens empty if the session directory is empty
- Panes tile automatically as pidfiles appear in the watched directory
- Each pane shows:
  - Label (human-readable name if set, PID otherwise)
  - PID
  - Status (running / sleeping / zombie / dead)
  - CPU %
  - RSS memory
  - Uptime since first seen by this TUI instance
  - Sparkline of CPU over last 60 seconds (per-instance, resets on fresh `show`)
- When a process disappears (pidfile removed or process dies), the pane goes visually distinct and shows time of death
- Dead panes are kept visible until manually dismissed — they are informative

### Tiling

Fixed grid layout. Panes fill left-to-right, wrap to next row. No manual layout management.

### Keybindings

| Key | Action |
|-----|--------|
| `q` | Quit |
| `d` | Remove selected pane (also deletes the pidfile) |
| `r` | Refresh / re-poll selected pid |
| `↑↓←→` | Navigate panes |

---

## Change Detection

The TUI watches the session directory using the `notify` crate, which abstracts:
- Linux: `inotify`
- macOS: `FSEvents` / `kqueue`

File created → new pane added
File deleted → pane marked dead (kept visible until dismissed)

---

## Process Info

Uses the `sysinfo` crate on both Linux and macOS for consistency. Polling interval is 1s by default, configurable. Fields read per process: status, CPU %, RSS memory.

---

## Rust Crates

| Purpose | Crate |
|---------|-------|
| TUI rendering | `ratatui` |
| Filesystem watching | `notify` |
| Process info | `sysinfo` |
| Async runtime | `tokio` |
| Argument parsing | `clap` |
| TOML config (optional) | `toml` + `serde` |

---

## Configuration

Optional `~/.config/pidtop/config.toml` for defaults:

```toml
[display]
refresh_interval_ms = 1000
sparkline_history   = 60     # seconds of CPU history to keep per pane

[theme]
dead_style = "red"           # color of dead process panes
```

No configuration is required — all defaults are sensible out of the box.

---

## Non-Goals

- No process control (kill, pause, resume)
- No log/stdout tailing
- No alerting or notifications
- No persistent sessions across reboots
- No network process monitoring
- No watching processes by name (PID only at the protocol level)