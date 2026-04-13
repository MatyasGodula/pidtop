use anyhow::{Context, Result, bail};
use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub fn base_dir() -> PathBuf {
    let tmp = std::env::var("TMPDIR")
        .or_else(|_| std::env::var("TMP"))
        .unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(tmp).join("pidtop")
}

pub fn session_dir(session: &str) -> PathBuf {
    base_dir().join(session)
}

pub fn start(session: &str) -> Result<()> {
    let dir = session_dir(session);
    fs::create_dir_all(&dir).context("failed to create session directory")?;
    eprintln!("session '{}' created at {}", session, dir.display());
    Ok(())
}

pub fn add(session: &str, pid_arg: Option<u32>, name: Option<&str>) -> Result<()> {
    let dir = session_dir(session);
    fs::create_dir_all(&dir).context("failed to create session directory")?;

    let pid: u32 = match pid_arg {
        Some(p) => p,
        None => {
            let stdin = io::stdin();
            let line = stdin
                .lock()
                .lines()
                .next()
                .context("no PID provided — pass as argument or pipe via stdin")?
                .context("failed to read stdin")?;
            line.trim().parse().context("stdin is not a valid PID")?
        }
    };

    let pidfile = dir.join(format!("{pid}.pid"));
    let label = name.unwrap_or("");
    fs::write(&pidfile, label).context("failed to write pidfile")?;
    eprintln!("added PID {pid} to session '{session}'");
    Ok(())
}

pub fn remove(session: &str, target: &str) -> Result<()> {
    let dir = session_dir(session);
    if !dir.exists() {
        bail!("session '{session}' does not exist");
    }

    // Try as PID first
    let pidfile = dir.join(format!("{target}.pid"));
    if pidfile.exists() {
        fs::remove_file(&pidfile).context("failed to remove pidfile")?;
        eprintln!("removed PID {target} from session '{session}'");
        return Ok(());
    }

    // Try as name — scan pidfiles for matching label
    for entry in fs::read_dir(&dir).context("failed to read session directory")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("pid") {
            let contents = fs::read_to_string(&path)?;
            if contents.trim() == target {
                let stem = path.file_stem().unwrap().to_string_lossy();
                fs::remove_file(&path)?;
                eprintln!("removed PID {stem} ('{target}') from session '{session}'");
                return Ok(());
            }
        }
    }

    bail!("no process matching '{target}' found in session '{session}'");
}

pub fn list() -> Result<()> {
    let base = base_dir();
    if !base.exists() {
        eprintln!("no sessions found");
        return Ok(());
    }

    let mut found = false;
    for entry in fs::read_dir(&base).context("failed to read pidtop base directory")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name().to_string_lossy().to_string();
            let count = fs::read_dir(entry.path())?
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .is_some_and(|ext| ext == "pid")
                })
                .count();
            println!("{name} ({count} processes)");
            found = true;
        }
    }
    if !found {
        eprintln!("no sessions found");
    }
    Ok(())
}
