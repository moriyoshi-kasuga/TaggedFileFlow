use std::path::Path;

use crate::data::{Data, Document, DocumentPath, SaveType};
use anyhow::{anyhow, Context};
use rand::{thread_rng, Rng};

use super::{list::send, Run, CP, MV};

impl Run for MV {
    fn run(&self) -> anyhow::Result<()> {
        action(SaveType::MV, self.files.clone(), self.name.clone())
    }
}
impl Run for CP {
    fn run(&self) -> anyhow::Result<()> {
        action(SaveType::CP, self.files.clone(), self.name.clone())
    }
}

fn action(save: SaveType, mut files: Vec<String>, name: Option<String>) -> anyhow::Result<()> {
    files.sort();
    let mut saves: Vec<DocumentPath> = Vec::default();
    for f in files {
        let path = Path::new(&f);
        if !path.exists() {
            return Err(anyhow!("{} is not exists", f));
        }
        if path.is_file() {
            saves.push(DocumentPath::File(f.to_string()));
            continue;
        }
        if path.is_dir() {
            saves.push(DocumentPath::Dir(f.trim_end_matches('/').to_string()));
            continue;
        }
        return Err(anyhow!("{} is not a file or dir", &f));
    }
    let mut current = 0;
    while current != saves.len() {
        let path = &saves[current];
        if path.is_file() {
            current += 1;
            continue;
        }
        let path = path.to_path();
        let mut removal = vec![];
        for (index, f) in saves[(current + 1)..].iter().enumerate() {
            if f.to_path().starts_with(&path) {
                removal.push(index + current + 1);
            }
        }
        removal.into_iter().rev().for_each(|i| {
            saves.remove(i);
        });
        current += 1;
    }
    let name = name.unwrap_or_else(|| thread_rng().gen_range('a'..='z').to_string());
    let mut data = Data::default()?;
    if data.get(&name).is_some() {
        return Err(anyhow!("document {} is already exists", name));
    }
    let doc = Document {
        current: std::env::current_dir()?
            .to_str()
            .with_context(|| "Parse Error of Path to String")?
            .to_string(),
        name,
        save,
        files: saves,
    };
    send(&doc);
    data.add(doc);
    data.save()?;
    Ok(())
}
