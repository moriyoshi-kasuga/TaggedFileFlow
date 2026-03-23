use anyhow::Context;
use color_print::{cformat, cstr};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Document {
    File(PathBuf),
    Dir(PathBuf),
}

impl Document {
    pub fn as_path(&self) -> &Path {
        match self {
            Document::File(path) | Document::Dir(path) => path,
        }
    }

    pub fn is_file(&self) -> bool {
        matches!(self, Document::File(_))
    }

    pub fn type_label(&self) -> &'static str {
        match self {
            Document::File(_) => "file",
            Document::Dir(_) => "folder",
        }
    }

    /// Color-formatted display string for the document path
    pub fn colored_path(&self) -> String {
        match self {
            Document::File(path) => {
                let lossy = path.to_string_lossy();
                match lossy.rsplit_once('/') {
                    Some((dir, file)) => cformat!("<cyan>{}/<green!>{}", dir, file),
                    None => cformat!("<green!>{}", path.display()),
                }
            }
            Document::Dir(path) => cformat!("<cyan>{}/", path.display()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SaveType {
    MV,
    CP,
}

impl fmt::Display for SaveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SaveType::MV => write!(f, "Move"),
            SaveType::CP => write!(f, "Copy"),
        }
    }
}

impl SaveType {
    pub fn colored_label(&self) -> &'static str {
        match self {
            SaveType::MV => cstr!("<blue>Move</>"),
            SaveType::CP => cstr!("<yellow>Copy</>"),
        }
    }
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
            anyhow::bail!("document '{}' already exists", document.name);
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

    pub fn del(&mut self, name: &str) -> Option<DocumentBlock> {
        self.blocks
            .iter()
            .position(|block| block.name == name)
            .map(|pos| self.blocks.remove(pos))
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    pub fn blocks(&self) -> &[DocumentBlock] {
        &self.blocks
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let file_path = get_file()?;
        let json = serde_json::to_string(self).context("failed to serialize data")?;
        std::fs::write(&file_path, json)
            .with_context(|| format!("failed to write data to '{}'", file_path.display()))?;
        Ok(())
    }

    pub fn load() -> anyhow::Result<Self> {
        let file_path = get_file()?;
        let data = std::fs::read(&file_path)
            .with_context(|| format!("failed to read data from '{}'", file_path.display()))?;
        let data = serde_json::from_slice(&data).with_context(|| {
            format!("failed to deserialize data from '{}'", file_path.display())
        })?;
        Ok(data)
    }
}

fn ensure_empty_json(path: &Path) -> anyhow::Result<()> {
    std::fs::write(path, "{}")
        .with_context(|| format!("failed to initialize data file '{}'", path.display()))
}

fn get_file() -> anyhow::Result<PathBuf> {
    let mut cache = dirs::cache_dir().context("failed to determine cache directory")?;
    std::fs::create_dir_all(&cache)
        .with_context(|| format!("failed to create cache directory '{}'", cache.display()))?;
    cache.push("tagged-file-flow-documents.json");

    if !cache.exists() {
        ensure_empty_json(&cache)?;
    } else {
        let metadata = std::fs::metadata(&cache)
            .with_context(|| format!("failed to read metadata for '{}'", cache.display()))?;
        if !metadata.is_file() {
            anyhow::bail!(
                "expected a file at '{}', but it is not a file",
                cache.display()
            );
        }
        if metadata.len() == 0 {
            ensure_empty_json(&cache)?;
        }
    }

    Ok(cache)
}

pub fn show_block(block: &DocumentBlock) {
    let save_type = block.save.colored_label();
    println!();
    println!(
        "{} on {}",
        cformat!("<white>=></> {}: <red>{}</>", save_type, block.name),
        cformat!("<white>[{}]</>", block.current.display()),
    );
    for doc in &block.documents {
        println!(" {} {}", cformat!("<white>-</>"), doc.colored_path());
    }
}
