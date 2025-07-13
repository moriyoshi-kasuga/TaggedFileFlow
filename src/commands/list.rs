use crate::data::{Data, show_block};
use anyhow::{Context, Ok};
use clap::Parser;
use color_print::cprintln;

use super::Run;

#[derive(Parser)]
#[command(about)]
pub struct List {
    /// Names of files
    pub names: Vec<String>,
}

impl Run for List {
    fn run(self) -> anyhow::Result<()> {
        let data = Data::load()?;
        if self.names.is_empty() {
            if data.is_empty() {
                cprintln!("<white>no documents");
            }
            for block in data.blocks() {
                show_block(block);
            }
        } else {
            for name in &self.names {
                let doc = data
                    .get(name)
                    .with_context(|| format!("document '{name}' not found"))?;
                show_block(doc);
            }
        }
        Ok(())
    }
}
