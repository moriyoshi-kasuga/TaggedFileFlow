use crate::data::{show_block, Data};
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
                cprintln!("<white>No documents saved</>");
                return Ok(());
            }
            for block in data.blocks() {
                show_block(block);
            }
        } else {
            for name in &self.names {
                let doc = data
                    .get(name)
                    .ok_or_else(|| anyhow::anyhow!("document '{}' not found", name))?;
                show_block(doc);
            }
        }
        Ok(())
    }
}
