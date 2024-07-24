use core::panic;
use std::path::Path;

use itertools::Itertools;

use crate::{
    data::{Data, Document, DocumentPath, Save},
    types::SaveArgs,
};

pub fn action(save: Save, args: SaveArgs) {
    let mut files = args
        .files
        .iter()
        .map(|f| {
            let path = Path::new(&f);
            if !path.exists() {
                panic!("{} is not exists", f);
            }
            if path.is_file() {
                return DocumentPath::FILE(f.to_string());
            }
            if path.is_dir() {
                return DocumentPath::FOLDER(f.trim_end_matches('/').to_string());
            }
            panic!("{} is not a file or dir", &f);
        })
        .collect_vec();
    let mut current = 0;
    while current != files.len() {
        let path = &files[current];
        if path.is_file() {
            current += 1;
            continue;
        }
        let path = path.to_path();
        let mut removal = vec![];
        for (index, f) in files[(current + 1)..].iter().enumerate() {
            if f.to_path().starts_with(&path) {
                removal.push(index + current + 1);
            }
        }
        removal.into_iter().rev().for_each(|i| {
            files.remove(i);
        });
        current += 1;
    }
    let mut data = Data::default();
    data.add(Document {
        current: std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        name: args.name.unwrap_or("example".to_string()),
        save,
        files,
    });
    data.save();
}
