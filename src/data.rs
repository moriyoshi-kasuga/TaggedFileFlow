use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Save {
    MV,
    CP,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DocumentPath {
    FILE(String),
    FOLDER(String),
}

impl DocumentPath {
    pub fn to_path(&self) -> String {
        match self {
            DocumentPath::FILE(path) => path.to_string(),
            DocumentPath::FOLDER(path) => path.to_string(),
        }
    }
    pub fn is_file(&self) -> bool {
        matches!(self, DocumentPath::FILE(_))
    }
    pub fn is_folder(&self) -> bool {
        matches!(self, DocumentPath::FOLDER(_))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    pub current: String,
    pub name: String,
    pub save: Save,
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
    pub fn all(&self) -> Vec<&Document> {
        self.documents.iter().collect()
    }
    pub fn save(&self) {
        fs::write(get_file(), serde_json::to_string(self).unwrap()).unwrap();
    }
}

impl Default for Data {
    fn default() -> Self {
        let data = &fs::read(get_file()).unwrap();
        let data = String::from_utf8_lossy(data).to_string();
        if data.is_empty() {
            return serde_json::from_str("{}").unwrap();
        }
        serde_json::from_str(&data).unwrap()
    }
}

fn get_file() -> PathBuf {
    let mut cache = dirs::cache_dir().unwrap();
    cache.push(env!("CARGO_PKG_NAME"));
    fs::create_dir_all(&cache).unwrap();
    cache.push("documents.json");
    if !cache.exists() {
        let _ = fs::write(&cache, "{}");
    }
    cache
}
