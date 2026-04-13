use sysinfo::{Pid, ProcessStatus, System};

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub label: String,
    pub status: ProcStatus,
    pub cpu_percent: f32,
    pub memory_bytes: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProcStatus {
    Running,
    Sleeping,
    Zombie,
    Dead,
    Unknown,
}

impl std::fmt::Display for ProcStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcStatus::Running => write!(f, "running"),
            ProcStatus::Sleeping => write!(f, "sleeping"),
            ProcStatus::Zombie => write!(f, "zombie"),
            ProcStatus::Dead => write!(f, "dead"),
            ProcStatus::Unknown => write!(f, "unknown"),
        }
    }
}

impl From<ProcessStatus> for ProcStatus {
    fn from(s: ProcessStatus) -> Self {
        match s {
            ProcessStatus::Run => ProcStatus::Running,
            ProcessStatus::Sleep | ProcessStatus::Idle => ProcStatus::Sleeping,
            ProcessStatus::Zombie => ProcStatus::Zombie,
            ProcessStatus::Dead => ProcStatus::Dead,
            _ => ProcStatus::Unknown,
        }
    }
}

pub fn poll_process(sys: &System, pid: u32, label: &str) -> ProcessInfo {
    let sysinfo_pid = Pid::from_u32(pid);
    match sys.process(sysinfo_pid) {
        Some(proc) => ProcessInfo {
            pid,
            label: if label.is_empty() {
                pid.to_string()
            } else {
                label.to_string()
            },
            status: proc.status().into(),
            cpu_percent: proc.cpu_usage(),
            memory_bytes: proc.memory(),
        },
        None => ProcessInfo {
            pid,
            label: if label.is_empty() {
                pid.to_string()
            } else {
                label.to_string()
            },
            status: ProcStatus::Dead,
            cpu_percent: 0.0,
            memory_bytes: 0,
        },
    }
}
