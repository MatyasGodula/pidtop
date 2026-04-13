mod app;
mod cli;
mod config;
mod process;
mod session;
mod tui;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Start { session } => session::start(&session),
        Command::Add { session, pid, name } => session::add(&session, pid, name.as_deref()),
        Command::Show { session } => tui::run(&session),
        Command::Remove { session, target } => session::remove(&session, &target),
        Command::List => session::list(),
    }
}
