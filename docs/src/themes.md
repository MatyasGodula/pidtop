# Themes

Themes are standalone TOML files stored in:

```
~/.config/pidtop/themes/
```

Each file defines the color palette for the TUI. To use a theme, set its name (filename without `.toml`) in your config:

```toml
# ~/.config/pidtop/config.toml
theme = "gruvbox"
```

This loads `~/.config/pidtop/themes/gruvbox.toml`.

## Bundled themes

pidtop ships with four themes in the `themes/` directory of the repository. Copy whichever you like into `~/.config/pidtop/themes/`:

```bash
mkdir -p ~/.config/pidtop/themes
cp themes/gruvbox.toml ~/.config/pidtop/themes/
```

| Theme | Description |
|-------|-------------|
| `default` | Standard terminal colors — white, cyan, green, red |
| `gruvbox` | Warm retro palette based on the Gruvbox color scheme |
| `ayu` | Clean modern palette based on the Ayu color scheme |
| `matcha` | Soft green accent palette |

## Creating your own theme

A theme file defines every color used in the TUI. All fields have defaults, so you can override just the ones you want.

```toml
# ~/.config/pidtop/themes/mytheme.toml

border          = "white"       # normal pane border
border_selected = "cyan"        # selected pane border
border_dead     = "red"         # dead process pane border

status_running  = "green"       # "running" status text
status_sleeping = "yellow"      # "sleeping" status text
status_dead     = "red"         # "dead" / "zombie" status text
status_unknown  = "darkgray"    # "unknown" status text

sparkline       = "cyan"        # sparkline for live processes
sparkline_dead  = "darkgray"    # sparkline for dead processes
```

## Color values

Colors can be specified as:

**Named colors:**

`black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, `gray`, `darkgray`, `light_red`, `light_green`, `light_yellow`, `light_blue`, `light_magenta`, `light_cyan`

**Hex RGB:**

```toml
border_selected = "#fabd2f"
sparkline       = "#83a598"
```

## Example: Gruvbox

```toml
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
