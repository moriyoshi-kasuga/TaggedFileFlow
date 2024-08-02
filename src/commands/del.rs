use anyhow::anyhow;
use color_print::cprintln;

use crate::data::Data;

use super::{Del, Run};

impl Run for Del {
    fn run(&self) -> anyhow::Result<()> {
        let mut data = Data::default()?;

        for name in &self.names {
            if !data.del(name) {
                return Err(anyhow!("not found {} document", name));
            }
        }

        data.save()?;

        cprintln!(
            "<green>successfully deleted {} document</green>",
            self.names.join(", ")
        );
        Ok(())
    }
}
