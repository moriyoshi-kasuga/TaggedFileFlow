use clap::Parser;
use rand::seq::SliceRandom;
use std::path::PathBuf;

use crate::data::{Data, Document, DocumentBlock, SaveType, show_block};
use anyhow::anyhow;

use super::Run;

const MAX_NAME_DEPTH: usize = 3;

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
    let documents = collect_documents(files)?;
    let deduplicated_documents = remove_nested_paths(documents);

    let mut data = Data::load()?;
    let document_name = resolve_document_name(&data, name)?;

    let doc = DocumentBlock {
        current: current_path,
        name: document_name,
        save,
        documents: deduplicated_documents,
    };

    show_block(&doc);
    data.add(doc)?;
    data.save()?;
    Ok(())
}

fn collect_documents(files: Vec<String>) -> anyhow::Result<Vec<Document>> {
    let mut documents = Vec::with_capacity(files.len());

    for file in files {
        let path = PathBuf::from(file);

        if !path.exists() {
            return Err(anyhow!("{} does not exist", path.display()));
        }

        let document = if path.is_file() {
            Document::File(path)
        } else if path.is_dir() {
            Document::Dir(path)
        } else {
            return Err(anyhow!("{} is not a file or directory", path.display()));
        };

        documents.push(document);
    }

    documents.sort();
    Ok(documents)
}

fn remove_nested_paths(mut documents: Vec<Document>) -> Vec<Document> {
    let mut current = 0;

    while current < documents.len() {
        let current_doc = &documents[current];

        if current_doc.is_file() {
            current += 1;
            continue;
        }

        let current_path = current_doc.as_path();
        let mut indices_to_remove = Vec::new();

        for (index, doc) in documents[(current + 1)..].iter().enumerate() {
            if doc.as_path().starts_with(current_path) {
                indices_to_remove.push(index + current + 1);
            }
        }

        for &index in indices_to_remove.iter().rev() {
            documents.remove(index);
        }

        current += 1;
    }

    documents
}

fn resolve_document_name(data: &Data, name: Option<String>) -> anyhow::Result<String> {
    match name {
        Some(name) => {
            if data.get(&name).is_some() {
                Err(anyhow!("document '{}' already exists", name))
            } else {
                Ok(name)
            }
        }
        None => generate_unique_name(data),
    }
}

fn generate_unique_name(data: &Data) -> anyhow::Result<String> {
    let mut stack = vec![String::new()];
    let mut rng = rand::rng();

    loop {
        if stack.is_empty() {
            return Err(anyhow!("failed to generate unique document name"));
        }

        let prefix = stack.remove(0);
        let should_expand = prefix.len() + 1 < MAX_NAME_DEPTH;

        let mut chars: Vec<char> = ('a'..='z').collect();
        chars.shuffle(&mut rng);

        for c in chars {
            let candidate = format!("{prefix}{c}");

            if data.get(&candidate).is_none() {
                return Ok(candidate);
            }

            if should_expand {
                stack.push(candidate);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_remove_nested_paths() {
        let documents = vec![
            Document::Dir(PathBuf::from("a")),
            Document::File(PathBuf::from("a/b")),
            Document::Dir(PathBuf::from("a/c")),
            Document::File(PathBuf::from("a/c/d")),
            Document::File(PathBuf::from("b")),
        ];
        let actual = remove_nested_paths(documents);
        let expected = vec![
            Document::Dir(PathBuf::from("a")),
            Document::File(PathBuf::from("b")),
        ];
        assert_eq!(actual, expected);
    }
}
