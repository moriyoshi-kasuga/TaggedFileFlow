use anyhow::Context;
use color_print::{cformat, cprintln, cstr};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Document {
    File(PathBuf),
    Dir(PathBuf),
}

impl Document {
    pub fn as_path(&self) -> &PathBuf {
        match self {
            Document::File(path) => path,
            Document::Dir(path) => path,
        }
    }

    pub fn is_file(&self) -> bool {
        matches!(self, Document::File(_))
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SaveType {
    MV,
    CP,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DocumentBlock {
    pub current: PathBuf,
    pub name: String,
    pub save: SaveType,
    pub documents: Vec<Document>,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(default)]
    blocks: Vec<DocumentBlock>,
}

impl Data {
    pub fn add(&mut self, document: DocumentBlock) -> anyhow::Result<()> {
        if self.contains_name(&document.name) {
            anyhow::bail!("document with name '{}' already exists", document.name);
        }
        self.blocks.push(document);
        Ok(())
    }

    fn contains_name(&self, name: &str) -> bool {
        self.blocks.iter().any(|block| block.name == name)
    }

    pub fn get(&self, name: &str) -> Option<&DocumentBlock> {
        self.blocks.iter().find(|block| block.name == name)
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, name: &str) -> Option<&mut DocumentBlock> {
        self.blocks.iter_mut().find(|block| block.name == name)
    }

    pub fn del(&mut self, name: &str) -> Option<DocumentBlock> {
        self.blocks
            .iter()
            .position(|block| block.name == name)
            .map(|pos| self.blocks.remove(pos))
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    pub fn blocks(&self) -> &Vec<DocumentBlock> {
        &self.blocks
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let file_path = get_file()?;
        let json = serde_json::to_string(self).context("failed to serialize data")?;
        std::fs::write(&file_path, json)
            .with_context(|| format!("failed to write data to file: {}", file_path.display()))?;
        Ok(())
    }

    pub fn load() -> anyhow::Result<Self> {
        let file_path = get_file()?;
        let data = std::fs::read(&file_path)
            .with_context(|| format!("failed to read data from file: {}", file_path.display()))?;
        let data = serde_json::from_slice(&data).context("failed to deserialize data")?;
        Ok(data)
    }
}

fn get_file() -> anyhow::Result<PathBuf> {
    let mut cache = dirs::cache_dir().context("not exists cache directory")?;
    std::fs::create_dir_all(&cache).context("failed to create cache directory")?;
    cache.push("tagged-file-flow-documents.json");
    if !cache.exists() {
        std::fs::write(&cache, "{}")
            .with_context(|| format!("failed to create file: {}", cache.display()))?;
    } else {
        let metadata = std::fs::metadata(&cache)
            .with_context(|| format!("failed to read metadata for file: {}", cache.display()))?;
        if !metadata.is_file() {
            anyhow::bail!(
                "expected a file at {}, but it is not a file",
                cache.display()
            );
        }
        let file_size = metadata.len();

        if file_size == 0 {
            // If the file is empty, we can write an empty JSON object to it
            std::fs::write(&cache, "{}")
                .with_context(|| format!("failed to write to file: {}", cache.display()))?;
        }
    }
    Ok(cache)
}

pub fn show_block(block: &DocumentBlock) {
    let save_type = match block.save {
        SaveType::MV => cstr!("<blue>Move"),
        SaveType::CP => cstr!("<yellow>Copy"),
    };
    println!();
    cprintln!(
        r###"<white>=> </>{}: <red>{}</> on <white>[{}]</>"###,
        save_type,
        block.name,
        block.current.display(),
    );
    for path in &block.documents {
        let doc_type = match path {
            Document::File(path) => {
                let opt = path.to_string_lossy();
                let opt = opt.rsplit_once('/');
                match opt {
                    Some((path, file)) => cformat!("<cyan>{}/<green!>{}", path, file),
                    None => cformat!("<green!>{}", path.display()),
                }
            }
            Document::Dir(path) => cformat!("<cyan>{}/", path.display()),
        };
        cprintln!(" <white>-</white> {}", doc_type);
    }
}
