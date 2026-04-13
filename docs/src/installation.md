# Installation

## From source (requires Rust)

```bash
git clone https://github.com/matyas/pidtop.git
cd pidtop
cargo install --path .
```

This installs the `pidtop` binary to `~/.cargo/bin/`. Make sure it's in your `PATH`:

```bash
# Add to your shell profile if not already present
export PATH="$HOME/.cargo/bin:$PATH"
```

## Verify

```bash
pidtop --help
```

You should see:

```
A session-based process monitor TUI

Usage: pidtop <COMMAND>

Commands:
  start   Create a named session
  add     Add a PID to a session
  show    Open the TUI for a session
  remove  Remove a process from a session by PID or name
  list    List all active sessions
  help    Print this message or the help of the given subcommand(s)
```
