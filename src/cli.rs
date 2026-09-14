use std::path::PathBuf;

use clap::{Args, Parser};

use crate::model::TagName;
use crate::shell::Shell;

pub(crate) const BIN_NAME: &str = "tff";

#[derive(Parser)]
#[command(name = BIN_NAME, about, version)]
pub(crate) enum Cli {
    /// Tag files and folders to move on paste
    Mv(StashArgs),
    /// Tag files and folders to copy on paste
    Cp(StashArgs),
    /// Transfer tagged files into the current directory
    Paste(PasteArgs),
    /// Delete tags
    Del(DelArgs),
    /// Show tags
    List(ListArgs),
    /// Print shell alias definitions
    Init(InitArgs),
}

#[derive(Args)]
pub(crate) struct StashArgs {
    /// Name of the tag [default: random characters]
    #[arg(short, long)]
    pub(crate) name: Option<TagName>,
    /// Paths to files or folders under the current directory
    #[arg(num_args = 1.., required = true)]
    pub(crate) files: Vec<PathBuf>,
}

#[derive(Args)]
pub(crate) struct PasteArgs {
    /// Names of tags
    #[arg(num_args = 1.., required = true)]
    pub(crate) names: Vec<TagName>,
    /// Overwrite destination paths that already exist
    #[arg(short, long)]
    pub(crate) force: bool,
}

#[derive(Args)]
pub(crate) struct DelArgs {
    /// Names of tags
    #[arg(num_args = 1.., required = true)]
    pub(crate) names: Vec<TagName>,
}

#[derive(Args)]
pub(crate) struct ListArgs {
    /// Names of tags [default: all]
    pub(crate) names: Vec<TagName>,
}

#[derive(Args)]
pub(crate) struct InitArgs {
    pub(crate) shell: Shell,
    /// Preferred characters for random tag names (e.g. "jkl"), tried in order before falling back to a random unused letter
    #[arg(long)]
    pub(crate) priority: Option<String>,
}
