use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SaveType {
    MV,
    CP,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DocumentPath {
    File(String),
    Dir(String),
}

impl DocumentPath {
    pub fn to_path(&self) -> String {
        match self {
            DocumentPath::File(path) => path.to_string(),
            DocumentPath::Dir(path) => path.to_string(),
        }
    }
    pub fn is_file(&self) -> bool {
        matches!(self, DocumentPath::File(_))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    pub current: String,
    pub name: String,
    pub save: SaveType,
    pub files: Vec<DocumentPath>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(default)]
    documents: Vec<Document>,
}

impl Data {
    pub fn add(&mut self, document: Document) {
        self.documents.push(document);
    }
    pub fn get(&self, name: &str) -> Option<&Document> {
        self.documents.iter().find(|document| document.name == name)
    }
    pub fn del(&mut self, name: &str) -> bool {
        let len = self.documents.len();
        self.documents.retain(|document| document.name != name);
        len != self.documents.len()
    }
    pub fn pop(&mut self, name: &str) -> Option<Document> {
        let doc = self.get(name);
        match doc {
            Some(doc) => {
                let doc = doc.clone();
                self.del(name);
                Some(doc)
            }
            None => None,
        }
    }
    pub fn all(&self) -> &Vec<Document> {
        &self.documents
    }
    pub fn save(&self) -> anyhow::Result<()> {
        fs::write(get_file()?, serde_json::to_string(self)?)?;
        Ok(())
    }

    pub fn default() -> anyhow::Result<Self> {
        let data = &fs::read(get_file()?)?;
        let data = String::from_utf8_lossy(data).to_string();
        if data.is_empty() {
            return Ok(serde_json::from_str("{}")?);
        }
        Ok(serde_json::from_str(&data)?)
    }
}

fn get_file() -> anyhow::Result<PathBuf> {
    let mut cache = dirs::cache_dir().with_context(|| "not exists cache directory")?;
    cache.push(env!("CARGO_PKG_NAME"));
    fs::create_dir_all(&cache)?;
    cache.push("documents.json");
    if !cache.exists() {
        let _ = fs::write(&cache, "{}");
    }
    Ok(cache)
}
