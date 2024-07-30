use anyhow::anyhow;
use color_print::cprintln;

use crate::data::Data;

use super::{Del, Run};

impl Run for Del {
    fn run(&self) -> anyhow::Result<()> {
        let mut data = Data::default()?;

        if !data.del(&self.name) {
            return Err(anyhow!("not found {} document", self.name));
        }

        data.save()?;

        cprintln!("<green>successfully deleted {} document</green>", self.name);
        Ok(())
    }
}
