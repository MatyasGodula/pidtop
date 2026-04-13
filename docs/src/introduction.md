# pidtop

A lightweight, session-based process monitor TUI for Linux and macOS, written in Rust.

Like a pitstop — you pull in, check what's running, pull out.

## Design Philosophy

- **Read-only** — pidtop never touches the processes it watches. No kill, no restart.
- **Filesystem as protocol** — sessions are directories, watched processes are files. No sockets, no daemons, no custom IPC.
- **Composable** — the interface is stdin and the filesystem. Any shell script or program can integrate without knowing pidtop exists.
- **Ephemeral by default** — session state lives in `$TMPDIR`, gone on reboot.
- **Cross-platform** — Linux and macOS via the `notify` and `sysinfo` crates.

## Non-Goals

pidtop is intentionally scoped. It does **not**:

- Kill, pause, or restart processes
- Tail logs or stdout
- Send alerts or notifications
- Persist sessions across reboots
- Monitor network connections
- Watch processes by name at the protocol level (PID only)
