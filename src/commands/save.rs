use rand::seq::SliceRandom;
use std::path::Path;

use crate::data::{show_doc, Data, Document, DocumentPath, SaveType};
use anyhow::{anyhow, Context};

use super::{Run, CP, MV};

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
    let current_path = std::env::current_dir()?
        .to_str()
        .with_context(|| "Parse Error of Path to String")?
        .to_string();

    let include_slash = current_path.clone() + "/";

    let mut saves: Vec<DocumentPath> = Vec::default();
    for f in files {
        match Path::new(&f).canonicalize()? {
            path if !path.exists() => {
                return Err(anyhow!("{} is not exists", f));
            }
            path if path.is_file() => {
                saves.push(DocumentPath::File(
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| anyhow!("uft8 error"))?
                        .strip_prefix(&include_slash)
                        .with_context(|| "System Error")?
                        .to_owned(),
                ));
            }
            path if path.is_dir() => {
                saves.push(DocumentPath::Dir(
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| anyhow!("uft8 error"))?
                        .strip_prefix(&include_slash)
                        .with_context(|| "System Error")?
                        .to_owned(),
                ));
            }
            _ => return Err(anyhow!("{} is not a file or dir", &f)),
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
        current: current_path,
        name,
        save,
        paths: saves,
    };
    show_doc(&doc);
    data.add(doc);
    data.save()?;
    Ok(())
}
