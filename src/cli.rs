use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pidtop", about = "A session-based process monitor TUI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Create a named session
    Start {
        /// Session name
        session: String,
    },
    /// Add a PID to a session (pass as argument, or pipe via stdin)
    Add {
        /// Session name
        session: String,
        /// PID to watch (reads from stdin if omitted)
        pid: Option<u32>,
        /// Human-readable label for the process
        #[arg(long)]
        name: Option<String>,
    },
    /// Open the TUI for a session
    Show {
        /// Session name
        session: String,
    },
    /// Remove a process from a session by PID or name
    Remove {
        /// Session name
        session: String,
        /// PID or name to remove
        target: String,
    },
    /// Delete a session and all its pidfiles
    Stop {
        /// Session name
        session: String,
    },
    /// List all active sessions
    List,
}
