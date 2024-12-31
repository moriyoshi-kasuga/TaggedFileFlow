use std::{fs, path::PathBuf};

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
                        fs::rename(from, to)?;
                    }
                    (SaveType::MV, false) => {
                        run_to_dir_all(from, to, fs::rename)?;
                        fs::remove_dir(from)?;
                    }
                    (SaveType::CP, true) => {
                        fs::copy(from, to)?;
                    }
                    (SaveType::CP, false) => {
                        run_to_dir_all(from, to, |from, to| fs::copy(from, to).map(|_| ()))?;
                    }
                };
            }
            show_doc(&doc);
        }
        data.save()?;
        Ok(())
    }
}

fn run_to_dir_all(
    src: &PathBuf,
    dst: &PathBuf,
    func: fn(PathBuf, PathBuf) -> std::io::Result<()>,
) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            run_to_dir_all(&entry.path(), &dst.join(entry.file_name()), func)?;
        } else {
            func(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
