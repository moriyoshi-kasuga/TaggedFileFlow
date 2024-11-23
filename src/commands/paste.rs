use std::{
    fs,
    path::{Path, PathBuf},
};

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
                        if file.is_file() {
                            fs::rename(from, to)?;
                        } else {
                            run_to_dir_all(from, to, fs::rename)?;
                        }
                    }
                    SaveType::CP => {
                        if file.is_file() {
                            fs::copy(from, to)?;
                        } else {
                            run_to_dir_all(from, to, |from, to| fs::copy(from, to).map(|_| ()))?;
                        }
                    }
                };
            }
            send(&doc);
        }
        data.save()?;
        Ok(())
    }
}

fn run_to_dir_all(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    func: fn(PathBuf, PathBuf) -> std::io::Result<()>,
) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            run_to_dir_all(entry.path(), dst.as_ref().join(entry.file_name()), func)?;
        } else {
            func(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
