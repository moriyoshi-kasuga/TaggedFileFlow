use anyhow::anyhow;
use clap::Parser;
use color_print::cprintln;

use crate::data::Data;

use super::Run;

#[derive(Parser)]
#[command(about)]
pub struct Del {
    /// Names of files
    #[clap(num_args = 1.., required = true)]
    pub names: Vec<String>,
}

impl Run for Del {
    fn run(self) -> anyhow::Result<()> {
        let mut data = Data::load()?;

        for name in &self.names {
            if data.del(name).is_none() {
                return Err(anyhow!("document '{name}' not found"));
            }
        }

        data.save()?;

        cprintln!(
            "<green>successfully deleted <red>{}</red> document</green>",
            self.names.join(", ")
        );
        Ok(())
    }
}
