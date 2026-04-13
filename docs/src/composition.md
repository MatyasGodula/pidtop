# Composition

pidtop is designed to be composed into scripts and workflows. Its interface is stdin, arguments, and the filesystem — no sockets, no daemons, no custom IPC.

## Startup scripts

The typical pattern is: start a session, launch processes, add their PIDs, then show the TUI.

```bash
#!/bin/bash
pidtop start vm-monitor

vzvm start ubuntu  | pidtop add vm-monitor --name ubuntu
vzvm start alpine  | pidtop add vm-monitor --name alpine
vzvm start fedora  | pidtop add vm-monitor --name fedora

pidtop show vm-monitor
```

## Background processes

Use `$!` to capture the PID of the last backgrounded process:

```bash
pidtop start workers

./worker-a &
pidtop add workers $! --name "worker-a"

./worker-b &
pidtop add workers $! --name "worker-b"

pidtop show workers
```

## Using pgrep

Find running processes by name and add them:

```bash
pidtop start web

pgrep nginx | pidtop add web --name "nginx"
pgrep -f "node server.js" | pidtop add web --name "node"
```

## Dynamic monitoring

Because the TUI watches the session directory in real time, you can add and remove processes while it's running:

```bash
# Terminal 1: open the TUI
pidtop show my-session

# Terminal 2: add processes on the fly
pidtop add my-session $(pgrep postgres) --name "postgres"

# Terminal 2: remove when done
pidtop remove my-session postgres
```

## Session layout on disk

Sessions are plain directories. Each watched process is a `.pid` file:

```
$TMPDIR/pidtop/
  vm-monitor/
    1234.pid       # contains "ubuntu"
    5678.pid       # contains "alpine"
    9012.pid       # contains "" (no label)
  web-servers/
    3456.pid       # contains "nginx"
```

Filename is always the PID. File contents are the optional human-readable label. Any tool that can create or delete files can integrate with pidtop.

## Integration without pidtop CLI

You don't even need the `pidtop` CLI to add processes. Since the protocol is just files:

```bash
# Equivalent to: pidtop add my-session 1234 --name "myapp"
echo "myapp" > "$TMPDIR/pidtop/my-session/1234.pid"

# Equivalent to: pidtop remove my-session 1234
rm "$TMPDIR/pidtop/my-session/1234.pid"
```
