use anyhow::Result;
use clap::Parser;

mod del;
mod init;
mod list;
mod paste;
mod save;

pub trait Run {
    fn run(self) -> Result<()>;
}

#[derive(Parser)]
#[clap(about, version)]
pub enum Commands {
    /// mv to global
    MV(save::MV),
    /// cp to global
    CP(save::CP),
    /// paste from list
    Paste(paste::Paste),
    /// delete from list
    Del(del::Del),
    /// show list
    List(list::List),
    /// init alias
    Init(init::Init),
}

impl Run for Commands {
    fn run(self) -> Result<()> {
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
