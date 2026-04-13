# Configuration

pidtop uses an optional config file at:

```
~/.config/pidtop/config.toml
```

No configuration is required — all defaults are sensible out of the box.

## Options

```toml
[display]
# How often to poll process info (milliseconds)
refresh_interval_ms = 1000

# Seconds of CPU history to keep per sparkline
sparkline_history = 60

# Which theme to use (looks for ~/.config/pidtop/themes/<name>.toml)
theme = "default"
```

## Display

All settings live under the `[display]` section:

| Key | Default | Description |
|-----|---------|-------------|
| `refresh_interval_ms` | `1000` | Polling interval for process stats in milliseconds |
| `sparkline_history` | `60` | Number of seconds of CPU history shown in sparklines |
| `theme` | `"default"` | Theme name — loads `~/.config/pidtop/themes/<name>.toml` |

The `theme` value references a file in the [themes directory](./themes.md). Changes to the config or theme files are picked up live — no restart needed.

## Example

A complete config file:

```toml
[display]
refresh_interval_ms = 500
sparkline_history = 120
theme = "matcha"
```
