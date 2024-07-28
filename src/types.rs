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
    Del(DelArgs),
    /// show list
    List(ListArgs),
    /// init alias
    Init,
}

#[derive(Parser, Debug)]
#[command(about)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Parser, Debug, Clone)]
#[command(about)]
pub struct PasteArgs {
    /// Names of files
    pub name: String,
}

#[derive(Parser, Debug, Clone)]
#[command(about)]
pub struct SaveArgs {
    /// Names of files [default: first file name]
    #[arg(short, long)]
    pub name: Option<String>,
    /// Path to files
    #[clap(required = true)]
    pub files: Vec<String>,
}

#[derive(Parser, Debug, Clone)]
#[command(about)]
pub struct ListArgs {
    /// Names of files
    pub name: Option<String>,
}

#[derive(Parser, Debug, Clone)]
#[command(about)]
pub struct DelArgs {
    /// Names of files
    pub name: String,
}
