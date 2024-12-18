use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(about, disable_help_subcommand = true, version)]
pub enum Commands {
    /// mv to global
    MV(MV),
    /// cp to global
    CP(CP),
    /// paste from list
    Paste(Paste),
    /// del
    Del(Del),
    /// show list
    List(List),
    /// init alias
    Init(Init),
}

#[derive(Parser, Debug, Clone)]
#[command(about)]
pub struct Paste {
    /// Names of files
    #[clap(num_args = 1.., required = true)]
    pub names: Vec<String>,
}

#[derive(Parser, Debug, Clone)]
#[command(about)]
pub struct CP {
    /// Names of files [default: Random characters in the range a to z]
    #[arg(short, long)]
    pub name: Option<String>,
    /// Path to files
    #[clap(num_args = 1.., required = true)]
    pub files: Vec<String>,
}
#[derive(Parser, Debug, Clone)]
#[command(about)]
pub struct MV {
    /// Names of files [default: Random characters in the range a to z]
    #[arg(short, long)]
    pub name: Option<String>,
    /// Path to files
    #[clap(num_args = 1.., required = true)]
    pub files: Vec<String>,
}

#[derive(Parser, Debug, Clone)]
#[command(about)]
pub struct List {
    /// Names of files
    pub names: Vec<String>,
}

#[derive(Parser, Debug, Clone)]
#[command(about)]
pub struct Del {
    /// Names of files
    #[clap(num_args = 1.., required = true)]
    pub names: Vec<String>,
}

#[derive(Parser, Debug, Clone)]
#[command(about)]
pub struct Init {}
