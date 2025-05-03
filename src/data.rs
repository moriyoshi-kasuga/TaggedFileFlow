use anyhow::Context;
use color_print::{cformat, cprintln, cstr};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Clone)]
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

    pub fn is_dir(&self) -> bool {
        matches!(self, Document::Dir(_))
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
    pub current: String,
    pub name: String,
    pub save: SaveType,
    pub paths: Vec<Document>,
}

// #[derive(Serialize, Deserialize)]
// pub struct Data {
//     #[serde(default)]
//     documents: Vec<Document>,
// }
//
// impl Data {
//     pub fn add(&mut self, document: Document) {
//         self.documents.push(document);
//     }
//
//     pub fn get(&self, name: &str) -> Option<&Document> {
//         self.documents.iter().find(|document| document.name == name)
//     }
//
//     pub fn del(&mut self, name: &str) -> Option<Document> {
//         self.documents
//             .iter()
//             .position(|v| v.name == name)
//             .map(|v| self.documents.remove(v))
//     }
//
//     pub fn is_empty(&self) -> bool {
//         self.documents.is_empty()
//     }
//
//     pub fn documents(&self) -> &Vec<Document> {
//         &self.documents
//     }
//
//     pub fn save(&self) -> anyhow::Result<()> {
//         fs::write(get_file()?, serde_json::to_string(self)?)?;
//         Ok(())
//     }
//
//     pub fn default() -> anyhow::Result<Self> {
//         let data = fs::read(get_file()?)?;
//         if data.is_empty() {
//             return Ok(serde_json::from_str("{}")?);
//         }
//         Ok(serde_json::from_slice(&data)?)
//     }
// }
//
// fn get_file() -> anyhow::Result<PathBuf> {
//     let mut cache = dirs::cache_dir().with_context(|| "not exists cache directory")?;
//     fs::create_dir_all(&cache).with_context(|| "failed to create cache directory")?;
//     cache.push("tagged-file-flow-documents.json");
//     if !cache.exists() {
//         fs::write(&cache, "{}")?;
//     }
//     Ok(cache)
// }
//
// pub fn show_doc(doc: &Document) {
//     let save_type = match doc.save {
//         SaveType::MV => cstr!("<blue>Move"),
//         SaveType::CP => cstr!("<yellow>Copy"),
//     };
//     println!();
//     cprintln!(
//         r###"<white>=> </>{}: <red>{}</> on <white>[{}]</>"###,
//         save_type,
//         doc.name,
//         doc.current,
//     );
//     for path in &doc.paths {
//         let doc_type = match path {
//             DocumentPath::File(path) => {
//                 let opt = path.rsplit_once('/');
//                 match opt {
//                     Some((path, file)) => cformat!("<cyan>{}/<green!>{}", path, file),
//                     None => cformat!("<green!>{}", path),
//                 }
//             }
//             DocumentPath::Dir(path) => cformat!("<cyan>{}/", path),
//         };
//         cprintln!(" <white>-</white> {}", doc_type);
//     }
// }
