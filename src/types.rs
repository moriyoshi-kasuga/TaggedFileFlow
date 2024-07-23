use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// mv to global
    MV(SaveArgs),
    /// cp to global
    CP(SaveArgs),
    /// paste from list
    Paste(PasteArgs),
    /// del
    Del,
    /// show list
    List,
    /// init alias
    Init,
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct PasteArgs {
    /// Names of files
    pub name: String,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct SaveArgs {
    /// Names of files [default: first file name]
    #[arg(short, long)]
    pub name: Option<String>,
    /// Path to files
    #[clap(required = true)]
    pub files: Vec<String>,
}
