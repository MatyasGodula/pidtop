# Getting Started

## Quick example

Create a session, add some processes, and open the TUI:

```bash
# 1. Create a session
pidtop start my-session

# 2. Start a background process and add it
sleep 300 &
pidtop add my-session $! --name "sleeper"

# 3. Add another by PID
pidtop add my-session 1 --name "init"

# 4. Open the TUI
pidtop show my-session
```

That's it. The TUI shows each process in its own pane with live CPU, memory, status, and a CPU sparkline.

## What you'll see

Each pane displays:

- **Label** — the name you gave it, or the PID if unnamed
- **PID**
- **Status** — running, sleeping, zombie, or dead
- **CPU %** — current CPU usage
- **Memory** — resident set size (RSS)
- **Uptime** — time since this TUI instance first saw the process
- **Sparkline** — CPU usage over the last 60 seconds

## Adding processes dynamically

The TUI watches the session directory in real time. You can add or remove processes from another terminal and the TUI updates immediately:

```bash
# In another terminal
pidtop add my-session $(pgrep firefox) --name "firefox"
```

A new pane appears in the TUI without restarting it.

## Cleaning up

Sessions live in `$TMPDIR/pidtop/` and are gone on reboot. To remove a process from a session manually:

```bash
pidtop remove my-session firefox   # by name
pidtop remove my-session 12345     # by PID
```
