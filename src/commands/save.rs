use rand::seq::SliceRandom;
use std::path::Path;

use crate::data::{Data, Document, DocumentPath, SaveType};
use anyhow::{anyhow, Context};

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
    let mut data = Data::default()?;
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
            let random = &mut rand::thread_rng();

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
