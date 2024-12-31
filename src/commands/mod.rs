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
    MV(MV),
    /// cp to global
    CP(CP),
    /// paste from list
    Paste(Paste),
    /// delete from list
    Del(Del),
    /// show list
    List(List),
    /// init alias
    Init(Init),
}

#[derive(Parser)]
#[command(about)]
pub struct Paste {
    /// Names of files
    #[clap(num_args = 1.., required = true)]
    pub names: Vec<String>,
    /// If doc is a file, override If doc is a directory, merge the contents of the folder
    #[clap(short, long)]
    pub force: bool,
}

#[derive(Parser)]
#[command(about)]
pub struct CP {
    /// Names of files [default: Random characters in the range a to z]
    #[arg(short, long)]
    pub name: Option<String>,
    /// Path to files
    #[clap(num_args = 1.., required = true)]
    pub files: Vec<String>,
}
#[derive(Parser)]
#[command(about)]
pub struct MV {
    /// Names of files [default: Random characters in the range a to z]
    #[arg(short, long)]
    pub name: Option<String>,
    /// Path to files
    #[clap(num_args = 1.., required = true)]
    pub files: Vec<String>,
}

#[derive(Parser)]
#[command(about)]
pub struct List {
    /// Names of files
    pub names: Vec<String>,
}

#[derive(Parser)]
#[command(about)]
pub struct Del {
    /// Names of files
    #[clap(num_args = 1.., required = true)]
    pub names: Vec<String>,
}

#[derive(Parser)]
#[command(about)]
pub struct Init {}

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
