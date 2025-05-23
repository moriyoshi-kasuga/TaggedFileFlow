use clap::Parser;
use rand::seq::SliceRandom;
use std::path::PathBuf;

use crate::data::{show_block, Data, Document, DocumentBlock, SaveType};
use anyhow::anyhow;

use super::Run;

#[derive(Parser)]
#[command(about)]
pub struct CP {
    /// Names of files [default: Random characters in the range a to z]
    #[arg(short, long)]
    pub name: Option<String>,
    /// Path to files
    #[clap(num_args = 1.., required = true)]
    pub files: Vec<String>,
}
#[derive(Parser)]
#[command(about)]
pub struct MV {
    /// Names of files [default: Random characters in the range a to z]
    #[arg(short, long)]
    pub name: Option<String>,
    /// Path to files
    #[clap(num_args = 1.., required = true)]
    pub files: Vec<String>,
}

impl Run for MV {
    fn run(self) -> anyhow::Result<()> {
        action(SaveType::MV, self.files, self.name)
    }
}
impl Run for CP {
    fn run(self) -> anyhow::Result<()> {
        action(SaveType::CP, self.files, self.name)
    }
}

fn action(save: SaveType, files: Vec<String>, name: Option<String>) -> anyhow::Result<()> {
    let current_path = std::env::current_dir()?;

    let mut saves: Vec<Document> = Vec::default();
    for f in files {
        match PathBuf::from(f) {
            path if !path.exists() => {
                return Err(anyhow!("{} is not exists", path.display()));
            }
            path if path.is_file() => {
                saves.push(Document::File(path));
            }
            path if path.is_dir() => {
                saves.push(Document::Dir(path));
            }
            path => return Err(anyhow!("{} is not a file or dir", path.display())),
        }
    }
    saves.sort();

    let mut current = 0;
    while current != saves.len() {
        let path = &saves[current];
        if path.is_file() {
            current += 1;
            continue;
        }
        let path = path.as_path();
        let mut removal = vec![];
        for (index, f) in saves[(current + 1)..].iter().enumerate() {
            if f.as_path().starts_with(path) {
                removal.push(index + current + 1);
            }
        }
        removal.into_iter().rev().for_each(|i| {
            saves.remove(i);
        });
        current += 1;
    }
    let mut data = Data::load()?;
    let name = match name {
        Some(name) => {
            if data.get(&name).is_some() {
                Err(anyhow!("document {} is already exists", name))
            } else {
                Ok(name)
            }
        }
        None => {
            let max_depth = 3;
            let mut stack = vec![String::new()];
            let random = &mut rand::rng();

            'outer: loop {
                if stack.is_empty() {
                    break Err(anyhow!("document name is not specified"));
                }
                let prefix = stack.remove(0);
                let should_push = prefix.len() + 1 != max_depth;
                let mut chars: Vec<char> = ('a'..='z').collect();
                chars.shuffle(random);
                for c in chars {
                    let mut new_prefix = prefix.clone();
                    new_prefix.push(c);
                    if data.get(&new_prefix).is_none() {
                        break 'outer Ok(new_prefix);
                    }
                    if should_push {
                        stack.push(new_prefix)
                    }
                }
            }
        }
    }?;
    let doc = DocumentBlock {
        current: current_path,
        name,
        save,
        documents: saves,
    };
    show_block(&doc);
    data.add(doc)?;
    data.save()?;
    Ok(())
}
