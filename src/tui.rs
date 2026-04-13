use crate::app::App;
use crate::config::Config;
use crate::process::ProcStatus;
use crate::session;
use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Sparkline};
use ratatui::Terminal;
use std::fs;
use std::io;
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub fn run(session_name: &str) -> Result<()> {
    let config = Config::load();
    let dir = session::session_dir(session_name);
    if !dir.exists() {
        anyhow::bail!("session '{session_name}' does not exist — run `pidtop start {session_name}` first");
    }

    let mut app = App::new(config);

    // Load existing pidfiles
    load_pidfiles(&mut app, session_name)?;

    // Set up filesystem watcher for session directory
    let (fs_tx, fs_rx) = mpsc::channel();
    let _watcher = {
        let tx = fs_tx.clone();
        let mut w: RecommendedWatcher = notify::recommended_watcher(move |_res| {
            let _ = tx.send(());
        })?;
        w.watch(&dir, RecursiveMode::NonRecursive)?;
        w
    };

    // Set up watcher for config/theme changes
    let (cfg_tx, cfg_rx) = mpsc::channel();
    let _config_watcher = {
        let config_dir = dirs::config_dir().map(|d| d.join("pidtop"));
        let mut watcher: Option<RecommendedWatcher> = None;
        if let Some(ref cd) = config_dir {
            if cd.exists() {
                let tx = cfg_tx.clone();
                let mut w: RecommendedWatcher = notify::recommended_watcher(move |_res| {
                    let _ = tx.send(());
                })?;
                w.watch(cd, RecursiveMode::Recursive)?;
                watcher = Some(w);
            }
        }
        watcher
    };

    // Set up terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initial refresh
    app.refresh();

    let tick_rate = app.config.refresh_interval();
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| draw(f, &app, session_name))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);

        let key_event = if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => Some(key),
                _ => None,
            }
        } else {
            None
        };

        // Check filesystem events (session directory)
        let mut fs_changed = false;
        while fs_rx.try_recv().is_ok() {
            fs_changed = true;
        }
        if fs_changed {
            reload_pidfiles(&mut app, session_name)?;
        }

        // Check config/theme file changes
        let mut cfg_changed = false;
        while cfg_rx.try_recv().is_ok() {
            cfg_changed = true;
        }
        if cfg_changed {
            app.reload_config();
        }

        if let Some(key) = key_event {
            let cols = compute_cols(terminal.size()?.width);
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('d') => {
                    if let Some(pid) = app.dismiss_selected() {
                        // Also delete the pidfile
                        let pidfile = dir.join(format!("{pid}.pid"));
                        let _ = fs::remove_file(pidfile);
                    }
                }
                KeyCode::Char('r') => {
                    app.refresh();
                }
                KeyCode::Left => app.select_left(),
                KeyCode::Right => app.select_right(),
                KeyCode::Up => app.select_up(cols),
                KeyCode::Down => app.select_down(cols),
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.refresh();
            last_tick = Instant::now();
        }
    }

    // Restore terminal
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn load_pidfiles(app: &mut App, session: &str) -> Result<()> {
    let dir = session::session_dir(session);
    for entry in fs::read_dir(&dir).context("failed to read session directory")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("pid") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                if let Ok(pid) = stem.parse::<u32>() {
                    let label = fs::read_to_string(&path).unwrap_or_default().trim().to_string();
                    app.add_pane(pid, label);
                }
            }
        }
    }
    Ok(())
}

fn reload_pidfiles(app: &mut App, session: &str) -> Result<()> {
    let dir = session::session_dir(session);
    let mut current_pids: std::collections::HashSet<u32> = std::collections::HashSet::new();

    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("pid") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Ok(pid) = stem.parse::<u32>() {
                        current_pids.insert(pid);
                        let label = fs::read_to_string(&path).unwrap_or_default().trim().to_string();
                        app.add_pane(pid, label);
                    }
                }
            }
        }
    }

    // Mark removed pidfiles as dead
    let known_pids: Vec<u32> = app.panes.keys().copied().collect();
    for pid in known_pids {
        if !current_pids.contains(&pid) {
            app.remove_pane(pid);
        }
    }

    Ok(())
}

fn compute_cols(terminal_width: u16) -> usize {
    let min_pane_width = 40u16;
    (terminal_width / min_pane_width).max(1) as usize
}

fn draw(f: &mut ratatui::Frame, app: &App, session_name: &str) {
    let theme = &app.theme;
    let area = f.area();

    if app.panes.is_empty() {
        let msg = Paragraph::new(format!(
            "Session '{session_name}' — no processes. Add PIDs with:\n  echo <PID> | pidtop add {session_name} --name <label>"
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" pidtop — {session_name} ")),
        );
        f.render_widget(msg, area);
        return;
    }

    let cols = compute_cols(area.width);
    let panes: Vec<(&u32, &crate::app::Pane)> = app.panes.iter().collect();
    let rows_count = (panes.len() + cols - 1) / cols;

    let row_constraints: Vec<Constraint> = (0..rows_count)
        .map(|_| Constraint::Min(10))
        .collect();
    let row_areas = Layout::vertical(row_constraints).split(area);

    let col_constraints: Vec<Constraint> = (0..cols)
        .map(|_| Constraint::Ratio(1, cols as u32))
        .collect();

    for (i, pane_entry) in panes.iter().enumerate() {
        let (_, pane) = pane_entry;
        let row = i / cols;
        let col = i % cols;

        if row >= row_areas.len() {
            break;
        }

        let col_areas = Layout::horizontal(col_constraints.clone()).split(row_areas[row]);
        if col >= col_areas.len() {
            break;
        }

        let pane_area = col_areas[col];
        let is_selected = i == app.selected;
        draw_pane(f, pane, pane_area, is_selected, theme);
    }
}

fn draw_pane(
    f: &mut ratatui::Frame,
    pane: &crate::app::Pane,
    area: Rect,
    selected: bool,
    theme: &crate::config::ThemeConfig,
) {
    let is_dead = pane.dead_since.is_some();

    let border_style = if is_dead {
        Style::default().fg(theme.border_dead_color())
    } else if selected {
        Style::default()
            .fg(theme.border_selected_color())
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.border_color())
    };

    let title = format!(" {} (PID {}) ", pane.label, pane.pid);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(title);

    let inner = block.inner(area);
    f.render_widget(block, area);

    if inner.height < 2 {
        return;
    }

    let status_style = match pane.info.status {
        ProcStatus::Running => Style::default().fg(theme.status_running_color()),
        ProcStatus::Sleeping => Style::default().fg(theme.status_sleeping_color()),
        ProcStatus::Dead | ProcStatus::Zombie => Style::default().fg(theme.status_dead_color()),
        ProcStatus::Unknown => Style::default().fg(theme.status_unknown_color()),
    };

    let uptime = pane.first_seen.elapsed();
    let uptime_str = format_duration(uptime);

    let dead_str = if let Some(dead_at) = pane.dead_since {
        let ago = dead_at.elapsed();
        format!("  (dead {})", format_duration(ago))
    } else {
        String::new()
    };

    let memory_str = format_bytes(pane.info.memory_bytes);

    let lines = vec![
        Line::from(vec![
            Span::raw("Status: "),
            Span::styled(format!("{}{dead_str}", pane.info.status), status_style),
        ]),
        Line::from(format!("CPU:    {:.1}%", pane.info.cpu_percent)),
        Line::from(format!("Memory: {memory_str}")),
        Line::from(format!("Uptime: {uptime_str}")),
    ];

    let text_height = lines.len() as u16;
    let text = Paragraph::new(lines);
    let text_area = Rect {
        x: inner.x,
        y: inner.y,
        width: inner.width,
        height: text_height.min(inner.height),
    };
    f.render_widget(text, text_area);

    // Sparkline below text
    let sparkline_y = inner.y + text_height;
    if sparkline_y < inner.y + inner.height && !pane.sparkline.is_empty() {
        let sparkline_area = Rect {
            x: inner.x,
            y: sparkline_y,
            width: inner.width,
            height: (inner.y + inner.height) - sparkline_y,
        };
        let sparkline = Sparkline::default()
            .data(&pane.sparkline)
            .style(if is_dead {
                Style::default().fg(theme.sparkline_dead_color())
            } else {
                Style::default().fg(theme.sparkline_color())
            });
        f.render_widget(sparkline, sparkline_area);
    }
}

fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();
    if secs < 60 {
        format!("{secs}s")
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}
