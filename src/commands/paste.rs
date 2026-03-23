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

fn transfer_document(
    save: SaveType,
    doc: &Document,
    from: std::path::PathBuf,
    to: std::path::PathBuf,
) -> anyhow::Result<()> {
    match (save, doc) {
        (SaveType::MV, Document::File(_)) => {
            fs_more::file::move_file(&from, &to, Default::default()).with_context(|| {
                format!(
                    "failed to move file '{}' to '{}'",
                    from.display(),
                    to.display()
                )
            })?;
        }
        (SaveType::MV, Document::Dir(_)) => {
            fs_more::directory::move_directory(&from, &to, Default::default()).with_context(
                || {
                    format!(
                        "failed to move directory '{}' to '{}'",
                        from.display(),
                        to.display()
                    )
                },
            )?;
        }
        (SaveType::CP, Document::File(_)) => {
            fs_more::file::copy_file(&from, &to, Default::default()).with_context(|| {
                format!(
                    "failed to copy file '{}' to '{}'",
                    from.display(),
                    to.display()
                )
            })?;
        }
        (SaveType::CP, Document::Dir(_)) => {
            fs_more::directory::copy_directory(&from, &to, Default::default()).with_context(
                || {
                    format!(
                        "failed to copy directory '{}' to '{}'",
                        from.display(),
                        to.display()
                    )
                },
            )?;
        }
    }
    Ok(())
}

impl Run for Paste {
    fn run(self) -> anyhow::Result<()> {
        let current = std::env::current_dir().context("failed to get current working directory")?;
        let mut data = Data::load()?;

        for name in &self.names {
            let doc = data
                .del(name)
                .ok_or_else(|| anyhow::anyhow!("document '{}' not found", name))?;

            for doc_entry in &doc.documents {
                let path = doc_entry.as_path();
                let from = doc.current.join(path);
                let to = current.join(path);
                if !self.force && to.exists() {
                    cprintln!(
                        "<red>{} already exists: <white>{}</>",
                        doc_entry.type_label(),
                        to.display(),
                    );
                    continue;
                }
                transfer_document(doc.save, doc_entry, from, to)?;
            }
            show_block(&doc);
        }
        data.save()?;
        Ok(())
    }
}
