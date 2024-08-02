use crate::data::{Data, Document, DocumentPath, SaveType};
use anyhow::{anyhow, Context, Ok};
use color_print::{cformat, cprintln, cstr};

use super::{List, Run};

impl Run for List {
    fn run(&self) -> anyhow::Result<()> {
        if self.names.is_empty() {
            all()
        } else {
            for name in &self.names {
                doc(name)?;
            }
            Ok(())
        }
    }
}

fn doc(name: &String) -> anyhow::Result<()> {
    let data = Data::default()?;
    let doc = data
        .get(name)
        .with_context(|| format!("not found {} document", name))?;
    send(doc);
    Ok(())
}

fn all() -> anyhow::Result<()> {
    let binding = Data::default()?;
    let docs = binding.all();

    if docs.is_empty() {
        return Err(anyhow!("no documents"));
    }
    for doc in docs {
        send(doc);
    }
    Ok(())
}

pub fn send(doc: &Document) {
    let save_type = match doc.save {
        SaveType::MV => cstr!("<blue>Move"),
        SaveType::CP => cstr!("<yellow>Copy"),
    };
    println!();
    cprintln!(
        r###"<white>=> </>{}: <red>{}</> on <white>[{}]</>"###,
        save_type,
        doc.name,
        doc.current,
    );
    for path in &doc.files {
        let doc_type = match path {
            DocumentPath::File(path) => {
                let opt = path.rsplit_once('/');
                match opt {
                    Some((path, file)) => cformat!("<cyan>{}/<green!>{}", path, file),
                    None => cformat!("<green!>{}", path),
                }
            }
            DocumentPath::Dir(path) => cformat!("<cyan>{}/", path),
        };
        cprintln!("{}", doc_type);
    }
}
