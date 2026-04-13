use ratatui::style::Color;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub display: DisplayConfig,
}

#[derive(Deserialize)]
pub struct DisplayConfig {
    #[serde(default = "default_refresh_ms")]
    pub refresh_interval_ms: u64,
    #[serde(default = "default_sparkline_history")]
    pub sparkline_history: usize,
    #[serde(default = "default_theme_name")]
    pub theme: String,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            refresh_interval_ms: default_refresh_ms(),
            sparkline_history: default_sparkline_history(),
            theme: default_theme_name(),
        }
    }
}

fn default_refresh_ms() -> u64 {
    1000
}
fn default_sparkline_history() -> usize {
    60
}
fn default_theme_name() -> String {
    "default".into()
}

// --- Theme (loaded from ~/.config/pidtop/themes/<name>.toml) ---

#[derive(Deserialize)]
pub struct ThemeConfig {
    #[serde(default = "default_border")]
    pub border: String,
    #[serde(default = "default_border_selected")]
    pub border_selected: String,
    #[serde(default = "default_border_dead")]
    pub border_dead: String,
    #[serde(default = "default_status_running")]
    pub status_running: String,
    #[serde(default = "default_status_sleeping")]
    pub status_sleeping: String,
    #[serde(default = "default_status_dead")]
    pub status_dead: String,
    #[serde(default = "default_status_unknown")]
    pub status_unknown: String,
    #[serde(default = "default_sparkline")]
    pub sparkline: String,
    #[serde(default = "default_sparkline_dead")]
    pub sparkline_dead: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            border: default_border(),
            border_selected: default_border_selected(),
            border_dead: default_border_dead(),
            status_running: default_status_running(),
            status_sleeping: default_status_sleeping(),
            status_dead: default_status_dead(),
            status_unknown: default_status_unknown(),
            sparkline: default_sparkline(),
            sparkline_dead: default_sparkline_dead(),
        }
    }
}

impl ThemeConfig {
    pub fn border_color(&self) -> Color {
        parse_color(&self.border)
    }
    pub fn border_selected_color(&self) -> Color {
        parse_color(&self.border_selected)
    }
    pub fn border_dead_color(&self) -> Color {
        parse_color(&self.border_dead)
    }
    pub fn status_running_color(&self) -> Color {
        parse_color(&self.status_running)
    }
    pub fn status_sleeping_color(&self) -> Color {
        parse_color(&self.status_sleeping)
    }
    pub fn status_dead_color(&self) -> Color {
        parse_color(&self.status_dead)
    }
    pub fn status_unknown_color(&self) -> Color {
        parse_color(&self.status_unknown)
    }
    pub fn sparkline_color(&self) -> Color {
        parse_color(&self.sparkline)
    }
    pub fn sparkline_dead_color(&self) -> Color {
        parse_color(&self.sparkline_dead)
    }
}

fn default_border() -> String { "white".into() }
fn default_border_selected() -> String { "cyan".into() }
fn default_border_dead() -> String { "red".into() }
fn default_status_running() -> String { "green".into() }
fn default_status_sleeping() -> String { "yellow".into() }
fn default_status_dead() -> String { "red".into() }
fn default_status_unknown() -> String { "darkgray".into() }
fn default_sparkline() -> String { "cyan".into() }
fn default_sparkline_dead() -> String { "darkgray".into() }

fn parse_color(s: &str) -> Color {
    match s.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "gray" | "grey" => Color::Gray,
        "darkgray" | "darkgrey" | "dark_gray" | "dark_grey" => Color::DarkGray,
        "lightred" | "light_red" => Color::LightRed,
        "lightgreen" | "light_green" => Color::LightGreen,
        "lightyellow" | "light_yellow" => Color::LightYellow,
        "lightblue" | "light_blue" => Color::LightBlue,
        "lightmagenta" | "light_magenta" => Color::LightMagenta,
        "lightcyan" | "light_cyan" => Color::LightCyan,
        hex if hex.starts_with('#') && hex.len() == 7 => {
            let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(255);
            let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(255);
            let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(255);
            Color::Rgb(r, g, b)
        }
        _ => Color::White,
    }
}

fn themes_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("pidtop").join("themes"))
}

impl ThemeConfig {
    pub fn load(name: &str) -> Self {
        let path = themes_dir().map(|d| d.join(format!("{name}.toml")));

        match path {
            Some(p) if p.exists() => {
                let contents = fs::read_to_string(&p).unwrap_or_default();
                toml::from_str(&contents).unwrap_or_default()
            }
            _ => ThemeConfig::default(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = dirs::config_dir()
            .map(|d| d.join("pidtop").join("config.toml"))
            .filter(|p| p.exists());

        match path {
            Some(p) => {
                let contents = fs::read_to_string(&p).unwrap_or_default();
                toml::from_str(&contents).unwrap_or_default()
            }
            None => Config::default(),
        }
    }

    pub fn refresh_interval(&self) -> Duration {
        Duration::from_millis(self.display.refresh_interval_ms)
    }
}
