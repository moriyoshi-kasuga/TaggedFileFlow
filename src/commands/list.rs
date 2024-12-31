use crate::data::{show_doc, Data};
use anyhow::{Context, Ok};
use color_print::cprintln;

use super::{List, Run};

impl Run for List {
    fn run(self) -> anyhow::Result<()> {
        let data = Data::default()?;
        if self.names.is_empty() {
            if data.is_empty() {
                cprintln!("<white>no documents");
            }
            for doc in data.documents() {
                show_doc(doc);
            }
        } else {
            for name in &self.names {
                let doc = data
                    .get(name)
                    .with_context(|| format!("not found {} document", name))?;
                show_doc(doc);
            }
        }
        Ok(())
    }
}
