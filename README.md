# pidtop

A lightweight, session-based process monitor TUI for Linux and macOS, written in Rust.

Like a pitstop — you pull in, check what's running, pull out.

## Quick Start

```bash
cargo install --path .

pidtop start my-session
sleep 300 &
pidtop add my-session $! --name sleeper
pidtop show my-session
```

## Commands

| Command | Description |
|---------|-------------|
| `pidtop start <session>` | Create a named session |
| `pidtop add <session> [pid] [--name <label>]` | Add a PID (argument or stdin) |
| `pidtop show <session>` | Open the TUI |
| `pidtop remove <session> <pid\|name>` | Remove a process |
| `pidtop list` | List all sessions |

## TUI Keybindings

| Key | Action |
|-----|--------|
| `q` | Quit |
| `d` | Dismiss selected dead pane |
| `r` | Force refresh |
| Arrows | Navigate panes |

## Configuration

Optional config at `~/.config/pidtop/config.toml`:

```toml
[display]
refresh_interval_ms = 1000
sparkline_history = 60
theme = "gruvbox"
```

## Themes

Theme files live in `~/.config/pidtop/themes/<name>.toml`. Ships with `default`, `gruvbox`, `ayu`, and `matcha`. Changes are picked up live.

```toml
# ~/.config/pidtop/themes/gruvbox.toml
border          = "#ebdbb2"
border_selected = "#fabd2f"
border_dead     = "#fb4934"
status_running  = "#b8bb26"
status_sleeping = "#fabd2f"
status_dead     = "#fb4934"
status_unknown  = "#928374"
sparkline       = "#83a598"
sparkline_dead  = "#665c54"
```

## Composition

Designed for scripts. Any program that prints a PID composes naturally:

```bash
pidtop start web
pgrep nginx | pidtop add web --name nginx
./my-server &
pidtop add web $! --name my-server
pidtop show web
```

The protocol is just files — `$TMPDIR/pidtop/<session>/<pid>.pid` containing an optional label.

## Docs

Full documentation available at the [project docs](https://MatyasGodula.github.io/pidtop/) or build locally:

```bash
mdbook serve docs
```

## License

[MIT](LICENSE)
