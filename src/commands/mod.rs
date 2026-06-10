use anyhow::Result;
use clap::Parser;
use color_print::cformat;

use crate::data::DocumentBlock;

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

fn show_block(block: &DocumentBlock) {
    let save_type = block.save.colored_label();
    println!();
    println!(
        "{} on {}",
        cformat!("<white>=></> {}: <red>{}</>", save_type, block.name),
        cformat!("<white>[{}]</>", block.current.display()),
    );
    for doc in &block.documents {
        println!(" {} {}", cformat!("<white>-</>"), doc.colored_path());
    }
}
