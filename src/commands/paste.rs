use anyhow::Context;
use clap::Parser;
use color_print::cprintln;

use crate::data::{show_block, Data, Document, SaveType};

use super::Run;

#[derive(Parser)]
#[command(about)]
pub struct Paste {
    /// Names of files
    #[clap(num_args = 1.., required = true)]
    pub names: Vec<String>,
    /// If doc is a file, override If doc is a directory, merge the contents of the folder
    #[clap(short, long)]
    pub force: bool,
}

impl Run for Paste {
    fn run(self) -> anyhow::Result<()> {
        let current = std::env::current_dir()?;
        let mut data = Data::load()?;

        for name in &self.names {
            let doc = data
                .del(name)
                .with_context(|| format!("not found {name} document"))?;

            for doc_path in &doc.documents {
                let path = doc_path.as_path();
                let from = doc.current.join(path);
                let to = current.join(path);
                if !self.force && to.exists() {
                    cprintln!(
                        "<red>{} is exists <white>{}</white>",
                        if doc_path.is_file() { "file" } else { "folder" },
                        to.display()
                    );
                    continue;
                }
                match (doc.save, doc_path) {
                    (SaveType::MV, Document::File(_)) => {
                        fs_more::file::move_file(from, to, Default::default())?;
                    }
                    (SaveType::MV, Document::Dir(_)) => {
                        fs_more::directory::move_directory(from, to, Default::default())?;
                    }
                    (SaveType::CP, Document::File(_)) => {
                        fs_more::file::copy_file(from, to, Default::default())?;
                    }
                    (SaveType::CP, Document::Dir(_)) => {
                        fs_more::directory::copy_directory(from, to, Default::default())?;
                    }
                };
            }
            show_block(&doc);
        }
        data.save()?;
        Ok(())
    }
}
