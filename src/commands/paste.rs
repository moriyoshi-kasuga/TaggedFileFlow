use std::path::PathBuf;

use anyhow::Context;
use color_print::cprintln;

use crate::data::{show_doc, Data, SaveType};

use super::{Paste, Run};

impl Run for Paste {
    fn run(self) -> anyhow::Result<()> {
        let current = std::env::current_dir()?;
        let mut data = Data::default()?;

        for name in &self.names {
            let doc = data
                .del(name)
                .with_context(|| format!("not found {} document", name))?;

            let from = PathBuf::from(&doc.current);
            for doc_path in &doc.paths {
                let path = doc_path.as_path();
                let from = &from.clone().join(path);
                let to = &current.clone().join(path);
                if !self.force && to.exists() {
                    cprintln!(
                        "<red>{} is exists <white>{}</white>",
                        if doc_path.is_file() { "file" } else { "folder" },
                        path
                    );
                    continue;
                }
                match (doc.save, doc_path.is_file()) {
                    (SaveType::MV, true) => {
                        fs_more::file::move_file(from, to, Default::default())?;
                    }
                    (SaveType::MV, false) => {
                        fs_more::directory::move_directory(from, to, Default::default())?;
                    }
                    (SaveType::CP, true) => {
                        fs_more::file::copy_file(from, to, Default::default())?;
                    }
                    (SaveType::CP, false) => {
                        fs_more::directory::copy_directory(from, to, Default::default())?;
                    }
                };
            }
            show_doc(&doc);
        }
        data.save()?;
        Ok(())
    }
}
