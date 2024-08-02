use std::{fs, path::PathBuf};

use anyhow::Context;

use crate::data::{Data, SaveType};

use super::{list::send, Paste, Run};

impl Run for Paste {
    fn run(&self) -> anyhow::Result<()> {
        let mut data = Data::default()?;
        for name in &self.names {
            let doc = data
                .pop(name)
                .with_context(|| format!("not found {} document", name))?;
            let from = PathBuf::from(&doc.current);
            let current = std::env::current_dir()?;
            for file in &doc.files {
                let path = &file.to_path();
                let from = from.clone().join(path);
                let to = current.clone().join(path);
                match doc.save {
                    SaveType::MV => {
                        fs::rename(from, to)?;
                    }
                    SaveType::CP => {
                        fs::copy(from, to)?;
                    }
                };
            }
            send(&doc);
        }
        data.save()?;
        Ok(())
    }
}
