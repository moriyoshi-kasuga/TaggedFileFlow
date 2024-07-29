use std::{fs, path::PathBuf};

use crate::{
    data::{Data, Save},
    types::PasteArgs,
};

pub fn action(args: PasteArgs) {
    let mut data = Data::default();
    let doc = data.pop(&args.name);
    if doc.is_none() {
        panic!("not found document: {}", args.name);
    }
    let doc = doc.unwrap();
    let from = PathBuf::from(doc.current);
    let current = std::env::current_dir().unwrap();
    for file in doc.files {
        let path = &file.to_path();
        let from = from.clone().join(path);
        let to = current.clone().join(path);
        match doc.save {
            Save::MV => {
                fs::rename(from, to).unwrap();
            }
            Save::CP => {
                fs::copy(from, to).unwrap();
            }
        };
    }
    data.save();
}
