use anyhow::Result;

mod cmd;
mod del;
mod init;
mod list;
mod paste;
mod save;

pub use crate::commands::cmd::*;

pub trait Run {
    fn run(&self) -> Result<()>;
}

impl Run for Commands {
    fn run(&self) -> Result<()> {
        match self {
            Commands::MV(cmd) => cmd.run(),
            Commands::CP(cmd) => cmd.run(),
            Commands::Paste(cmd) => cmd.run(),
            Commands::Del(cmd) => cmd.run(),
            Commands::List(cmd) => cmd.run(),
            Commands::Init(cmd) => cmd.run(),
        }
    }
}
