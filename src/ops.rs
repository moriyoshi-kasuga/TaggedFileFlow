use std::path::{Path, PathBuf};

use anyhow::Context;
use rand::RngExt;

use crate::cli::{DelArgs, InitArgs, ListArgs, PasteArgs, StashArgs};
use crate::model::{Document, Mode, RelPath, Tag, TagName};
use crate::store::Store;
use crate::{shell, ui};

const NAME_LEN: usize = 3;
const NAME_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

pub(crate) fn stash(mode: Mode, args: StashArgs) -> anyhow::Result<()> {
    let origin = std::env::current_dir().context("failed to get current working directory")?;
    let documents = collect_documents(&origin, &args.files)?;

    let mut store = Store::load()?;
    let name = match args.name {
        Some(name) => {
            anyhow::ensure!(
                !store.contains(name.as_str()),
                "tag '{name}' already exists"
            );
            name
        }
        None => generate_unique_name(&store)?,
    };

    let tag = Tag {
        origin,
        mode,
        documents,
    };
    ui::print_tag(&name, &tag);
    store.insert(name, tag);
    store.commit()
}

pub(crate) fn paste(args: PasteArgs) -> anyhow::Result<()> {
    let dest = std::env::current_dir().context("failed to get current working directory")?;
    let mut store = Store::load()?;
    let taken = store.take_all(&args.names)?;

    // Nothing is persisted until commit, so bailing anywhere below leaves the tags intact.
    if !args.force {
        let mut conflicts = Vec::new();
        for (_, tag) in &taken {
            for document in &tag.documents {
                let to = dest.join(document.as_path());
                if to.exists() {
                    conflicts.push(format!(
                        "{} already exists: {}",
                        document.kind_label(),
                        to.display()
                    ));
                }
            }
        }
        anyhow::ensure!(
            conflicts.is_empty(),
            "paste aborted, nothing was changed. Use --force to overwrite:\n{}",
            conflicts.join("\n")
        );
    }

    for (name, tag) in &taken {
        for document in &tag.documents {
            let from = tag.origin.join(document.as_path());
            let to = dest.join(document.as_path());
            if to.exists() {
                remove_destination(&to)?;
            }
            transfer(tag.mode, document, &from, &to)?;
        }
        ui::print_tag(name, tag);
    }
    store.commit()
}

pub(crate) fn del(args: DelArgs) -> anyhow::Result<()> {
    let mut store = Store::load()?;
    let names: Vec<TagName> = store
        .take_all(&args.names)?
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    store.commit()?;
    ui::print_deleted(&names);
    Ok(())
}

pub(crate) fn list(args: ListArgs) -> anyhow::Result<()> {
    let store = Store::load()?;
    if args.names.is_empty() {
        if store.is_empty() {
            ui::print_no_tags();
            return Ok(());
        }
        for (name, tag) in store.iter() {
            ui::print_tag(name, tag);
        }
    } else {
        for (name, tag) in store.get_all(&args.names)? {
            ui::print_tag(name, tag);
        }
    }
    Ok(())
}

pub(crate) fn init(args: InitArgs) -> anyhow::Result<()> {
    ui::print_script(&shell::script(args.shell));
    Ok(())
}

fn collect_documents(origin: &Path, files: &[PathBuf]) -> anyhow::Result<Vec<Document>> {
    let mut documents = Vec::with_capacity(files.len());
    for file in files {
        let absolute = file
            .canonicalize()
            .with_context(|| format!("failed to resolve path '{}'", file.display()))?;
        let relative = RelPath::strip_prefix(origin, &absolute)
            .with_context(|| format!("'{}' is not under the current directory", file.display()))?;

        let document = if absolute.is_file() {
            Document::File(relative)
        } else if absolute.is_dir() {
            Document::Dir(relative)
        } else {
            anyhow::bail!("'{}' is not a file or directory", file.display());
        };
        documents.push(document);
    }
    Ok(normalize(documents))
}

/// Drops duplicates and paths nested inside a directory that is also listed.
fn normalize(mut documents: Vec<Document>) -> Vec<Document> {
    documents.sort_by(|a, b| a.as_path().cmp(b.as_path()));
    let mut kept: Vec<Document> = Vec::with_capacity(documents.len());
    for document in documents {
        let redundant = kept.iter().any(|k| {
            document.as_path() == k.as_path()
                || (!k.is_file() && document.as_path().starts_with(k.as_path()))
        });
        if !redundant {
            kept.push(document);
        }
    }
    kept
}

fn generate_unique_name(store: &Store) -> anyhow::Result<TagName> {
    let mut rng = rand::rng();
    loop {
        let candidate: String = (0..NAME_LEN)
            .map(|_| NAME_CHARS[rng.random_range(0..NAME_CHARS.len())] as char)
            .collect();
        if !store.contains(&candidate) {
            return TagName::new(candidate);
        }
    }
}

fn transfer(mode: Mode, document: &Document, from: &Path, to: &Path) -> anyhow::Result<()> {
    let result: anyhow::Result<()> = match (mode, document) {
        (Mode::Move, Document::File(_)) => fs_more::file::move_file(from, to, Default::default())
            .map(|_| ())
            .map_err(Into::into),
        (Mode::Move, Document::Dir(_)) => {
            fs_more::directory::move_directory(from, to, Default::default())
                .map(|_| ())
                .map_err(Into::into)
        }
        (Mode::Copy, Document::File(_)) => fs_more::file::copy_file(from, to, Default::default())
            .map(|_| ())
            .map_err(Into::into),
        (Mode::Copy, Document::Dir(_)) => {
            fs_more::directory::copy_directory(from, to, Default::default())
                .map(|_| ())
                .map_err(Into::into)
        }
    };
    result.with_context(|| {
        format!(
            "failed to {} {} '{}' to '{}'",
            mode.verb(),
            document.kind_label(),
            from.display(),
            to.display()
        )
    })
}

/// `--force` means "overwrite", not "merge": a colliding directory is replaced wholesale.
fn remove_destination(to: &Path) -> anyhow::Result<()> {
    if to.is_dir() {
        std::fs::remove_dir_all(to)
            .with_context(|| format!("failed to remove existing directory '{}'", to.display()))
    } else {
        std::fs::remove_file(to)
            .with_context(|| format!("failed to remove existing file '{}'", to.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(path: &str) -> Document {
        Document::File(RelPath::for_test(path))
    }

    fn dir(path: &str) -> Document {
        Document::Dir(RelPath::for_test(path))
    }

    #[test]
    fn collect_documents_resolves_files() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let files = vec![
            manifest_dir.join("src/ops.rs"),
            manifest_dir.join("src/model.rs"),
        ];
        let actual = collect_documents(&manifest_dir, &files).unwrap();
        assert_eq!(actual.len(), 2);
        assert!(actual.iter().all(Document::is_file));
    }

    #[test]
    fn collect_documents_rejects_nonexistent() {
        let current = std::env::current_dir().unwrap();
        let err =
            collect_documents(&current, &[PathBuf::from("nonexistent_file.txt")]).unwrap_err();
        let msg = format!("{err:#}");
        assert!(
            msg.contains("failed to resolve path 'nonexistent_file.txt'"),
            "unexpected error message: {msg}"
        );
    }

    #[test]
    fn normalize_keeps_independent_paths() {
        let documents = vec![file("a/b"), file("c/d")];
        assert_eq!(normalize(documents.clone()), documents);
    }

    #[test]
    fn normalize_drops_duplicates() {
        assert_eq!(normalize(vec![file("a/b"), file("a/b")]), vec![file("a/b")]);
    }

    #[test]
    fn normalize_drops_nested_paths() {
        let documents = vec![dir("a"), file("a/b"), dir("a/c"), file("a/c/d"), file("b")];
        assert_eq!(normalize(documents), vec![dir("a"), file("b")]);
    }

    #[test]
    fn normalize_sorts_directories_before_their_contents() {
        assert_eq!(normalize(vec![file("a/b"), dir("a")]), vec![dir("a")]);
    }

    #[test]
    fn normalize_empty() {
        assert!(normalize(vec![]).is_empty());
    }
}
