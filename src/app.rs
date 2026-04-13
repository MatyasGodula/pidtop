use crate::config::{Config, ThemeConfig};
use crate::process::{ProcStatus, ProcessInfo, poll_process};
use std::collections::BTreeMap;
use std::time::Instant;
use sysinfo::{ProcessRefreshKind, System, UpdateKind};

pub struct App {
    pub panes: BTreeMap<u32, Pane>,
    pub selected: usize,
    pub sys: System,
    pub config: Config,
    pub theme: ThemeConfig,
}

pub struct Pane {
    pub pid: u32,
    pub label: String,
    pub info: ProcessInfo,
    pub sparkline: Vec<u64>,
    pub first_seen: Instant,
    pub dead_since: Option<Instant>,
}

impl App {
    pub fn new(config: Config) -> Self {
        let theme = ThemeConfig::load(&config.display.theme);
        Self {
            panes: BTreeMap::new(),
            selected: 0,
            sys: System::new(),
            config,
            theme,
        }
    }

    pub fn reload_config(&mut self) {
        self.config = Config::load();
        self.theme = ThemeConfig::load(&self.config.display.theme);
    }

    pub fn add_pane(&mut self, pid: u32, label: String) {
        if self.panes.contains_key(&pid) {
            return;
        }
        let info = ProcessInfo {
            pid,
            label: if label.is_empty() {
                pid.to_string()
            } else {
                label.clone()
            },
            status: ProcStatus::Unknown,
            cpu_percent: 0.0,
            memory_bytes: 0,
        };
        self.panes.insert(
            pid,
            Pane {
                pid,
                label: if label.is_empty() {
                    pid.to_string()
                } else {
                    label
                },
                info,
                sparkline: Vec::new(),
                first_seen: Instant::now(),
                dead_since: None,
            },
        );
    }

    pub fn remove_pane(&mut self, pid: u32) {
        if let Some(pane) = self.panes.get_mut(&pid) {
            if pane.dead_since.is_none() {
                pane.dead_since = Some(Instant::now());
                pane.info.status = ProcStatus::Dead;
            }
        }
    }

    pub fn dismiss_selected(&mut self) -> Option<u32> {
        let pids: Vec<u32> = self.panes.keys().copied().collect();
        if let Some(&pid) = pids.get(self.selected) {
            if self.panes.get(&pid).is_some_and(|p| p.dead_since.is_some()) {
                self.panes.remove(&pid);
                if self.selected > 0 && self.selected >= self.panes.len() {
                    self.selected = self.panes.len().saturating_sub(1);
                }
                return Some(pid);
            }
        }
        None
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_processes_specifics(
            sysinfo::ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing()
                .with_cpu()
                .with_memory()
                .with_cmd(UpdateKind::Never),
        );

        let max_history = self.config.display.sparkline_history;
        for pane in self.panes.values_mut() {
            if pane.dead_since.is_some() {
                continue;
            }
            let info = poll_process(&self.sys, pane.pid, &pane.label);
            if info.status == ProcStatus::Dead && pane.dead_since.is_none() {
                pane.dead_since = Some(Instant::now());
            }
            // Scale CPU to 0-100 as u64 for sparkline
            let cpu_val = info.cpu_percent.round() as u64;
            pane.sparkline.push(cpu_val);
            if pane.sparkline.len() > max_history {
                pane.sparkline.remove(0);
            }
            pane.info = info;
        }
    }

    pub fn selected_pid(&self) -> Option<u32> {
        let pids: Vec<u32> = self.panes.keys().copied().collect();
        pids.get(self.selected).copied()
    }

    pub fn select_left(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn select_right(&mut self) {
        if self.selected + 1 < self.panes.len() {
            self.selected += 1;
        }
    }

    pub fn select_up(&mut self, cols: usize) {
        if self.selected >= cols {
            self.selected -= cols;
        }
    }

    pub fn select_down(&mut self, cols: usize) {
        let new = self.selected + cols;
        if new < self.panes.len() {
            self.selected = new;
        }
    }
}
