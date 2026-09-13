#![allow(clippy::print_stdout, clippy::print_stderr)]

use color_print::{cformat, cprintln, cstr};

use crate::model::{Document, Mode, Tag, TagName};

pub(crate) fn print_tag(name: &TagName, tag: &Tag) {
    println!();
    println!(
        "{} on {}",
        cformat!("<white>=></> {}: <red>{}</>", mode_label(tag.mode), name),
        cformat!("<white>[{}]</>", tag.origin.display()),
    );
    for document in &tag.documents {
        println!(" {} {}", cformat!("<white>-</>"), document_path(document));
    }
}

pub(crate) fn print_no_tags() {
    cprintln!("<white>No tags saved</>");
}

pub(crate) fn print_deleted(names: &[TagName]) {
    let joined = names
        .iter()
        .map(TagName::as_str)
        .collect::<Vec<_>>()
        .join(", ");
    cprintln!("<green>Successfully deleted:</> <red>{}</>", joined);
}

pub(crate) fn print_script(script: &str) {
    println!("{script}");
}

pub(crate) fn report_error(error: &anyhow::Error) {
    color_print::ceprintln!("<red>error</>: {:#}", error);
}

fn mode_label(mode: Mode) -> &'static str {
    match mode {
        Mode::Move => cstr!("<blue>Move</>"),
        Mode::Copy => cstr!("<yellow>Copy</>"),
    }
}

fn document_path(document: &Document) -> String {
    let path = document.as_path();
    match document {
        Document::File(_) => {
            let lossy = path.to_string_lossy();
            match lossy.rsplit_once('/') {
                Some((dir, file)) => cformat!("<cyan>{}/<green!>{}", dir, file),
                None => cformat!("<green!>{}", path.display()),
            }
        }
        Document::Dir(_) => cformat!("<cyan>{}/", path.display()),
        Document::Symlink(_) => {
            let lossy = path.to_string_lossy();
            match lossy.rsplit_once('/') {
                Some((dir, file)) => cformat!("<cyan>{}/<magenta!>{}", dir, file),
                None => cformat!("<magenta!>{}", path.display()),
            }
        }
    }
}
