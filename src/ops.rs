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
    if args.force {
        // `--force` merges same-kind collisions but can't reconcile a kind mismatch
        // (e.g. an existing file where a tagged folder would go), so that still aborts.
        let mut conflicts = Vec::new();
        for (_, tag) in &taken {
            for document in &tag.documents {
                let to = dest.join(document.as_path());
                if let Ok(existing) = std::fs::symlink_metadata(&to)
                    && let Some(existing_kind) = kind_mismatch(document, &existing)
                {
                    conflicts.push(format!(
                        "cannot merge {} '{}': existing entry is a {existing_kind}",
                        document.kind_label(),
                        to.display()
                    ));
                }
            }
        }
        anyhow::ensure!(
            conflicts.is_empty(),
            "paste aborted, nothing was changed. --force cannot merge mismatched kinds:\n{}",
            conflicts.join("\n")
        );
    } else {
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
            transfer(tag.mode, document, &from, &to, args.force)?;
        }
        ui::print_tag(name, tag);
    }
    store.commit()
}

/// Returns the existing entry's kind label if it doesn't match `document`'s kind.
fn kind_mismatch(document: &Document, existing: &std::fs::Metadata) -> Option<&'static str> {
    let existing_kind = if existing.is_symlink() {
        "symlink"
    } else if existing.is_dir() {
        "folder"
    } else {
        "file"
    };
    (existing_kind != document.kind_label()).then_some(existing_kind)
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
        let absolute = lexically_absolute(origin, file);
        // `symlink_metadata` (unlike `canonicalize`) never follows the final component,
        // so a symlink is tagged as the link entry itself, not whatever it points to.
        let metadata = std::fs::symlink_metadata(&absolute)
            .with_context(|| format!("failed to resolve path '{}'", file.display()))?;
        let relative = RelPath::strip_prefix(origin, &absolute)
            .with_context(|| format!("'{}' is not under the current directory", file.display()))?;

        let document = if metadata.is_symlink() {
            Document::Symlink(relative)
        } else if metadata.is_dir() {
            Document::Dir(relative)
        } else if metadata.is_file() {
            Document::File(relative)
        } else {
            anyhow::bail!("'{}' is not a file, folder, or symlink", file.display());
        };
        documents.push(document);
    }
    Ok(normalize(documents))
}

/// Resolves `path` (relative or absolute) against `origin` purely lexically, without
/// touching the filesystem, so that no symlink along the way is ever followed.
fn lexically_absolute(origin: &Path, path: &Path) -> PathBuf {
    let mut result = if path.is_absolute() {
        PathBuf::new()
    } else {
        origin.to_path_buf()
    };
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                result.pop();
            }
            other => result.push(other.as_os_str()),
        }
    }
    result
}

/// Drops duplicates and paths nested inside a directory that is also listed.
fn normalize(mut documents: Vec<Document>) -> Vec<Document> {
    documents.sort_by(|a, b| a.as_path().cmp(b.as_path()));
    let mut kept: Vec<Document> = Vec::with_capacity(documents.len());
    for document in documents {
        let redundant = kept.iter().any(|k| {
            document.as_path() == k.as_path()
                || (k.is_dir() && document.as_path().starts_with(k.as_path()))
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

fn transfer(
    mode: Mode,
    document: &Document,
    from: &Path,
    to: &Path,
    force: bool,
) -> anyhow::Result<()> {
    let result: anyhow::Result<()> = match (mode, document) {
        (Mode::Move, Document::File(_)) => fs_more::file::move_file(
            from,
            to,
            fs_more::file::FileMoveOptions {
                colliding_file_behaviour: colliding_file_behaviour(force),
            },
        )
        .map(|_| ())
        .map_err(Into::into),
        (Mode::Move, Document::Dir(_)) => fs_more::directory::move_directory(
            from,
            to,
            fs_more::directory::DirectoryMoveOptions {
                destination_directory_rule: directory_rule(force),
                ..Default::default()
            },
        )
        .map(|_| ())
        .map_err(Into::into),
        (Mode::Copy, Document::File(_)) => fs_more::file::copy_file(
            from,
            to,
            fs_more::file::FileCopyOptions {
                colliding_file_behaviour: colliding_file_behaviour(force),
            },
        )
        .map(|_| ())
        .map_err(Into::into),
        (Mode::Copy, Document::Dir(_)) => fs_more::directory::copy_directory(
            from,
            to,
            fs_more::directory::DirectoryCopyOptions {
                destination_directory_rule: directory_rule(force),
                ..Default::default()
            },
        )
        .map(|_| ())
        .map_err(Into::into),
        (mode, Document::Symlink(_)) => transfer_symlink(mode, from, to, force),
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

fn colliding_file_behaviour(force: bool) -> fs_more::file::CollidingFileBehaviour {
    if force {
        fs_more::file::CollidingFileBehaviour::Overwrite
    } else {
        fs_more::file::CollidingFileBehaviour::Abort
    }
}

fn directory_rule(force: bool) -> fs_more::directory::DestinationDirectoryRule {
    if force {
        fs_more::directory::DestinationDirectoryRule::AllowNonEmpty {
            colliding_file_behaviour: fs_more::file::CollidingFileBehaviour::Overwrite,
            colliding_subdirectory_behaviour:
                fs_more::directory::CollidingSubDirectoryBehaviour::Continue,
        }
    } else {
        fs_more::directory::DestinationDirectoryRule::AllowEmpty
    }
}

/// fs-more's file functions always resolve a symlink-to-file source to its target before
/// transferring, so a tagged symlink (the entry itself, not its target) is handled by hand.
fn transfer_symlink(mode: Mode, from: &Path, to: &Path, force: bool) -> anyhow::Result<()> {
    if force && std::fs::symlink_metadata(to).is_ok() {
        std::fs::remove_file(to)?;
    }
    let target = std::fs::read_link(from)?;
    std::os::unix::fs::symlink(&target, to)?;
    if mode == Mode::Move {
        std::fs::remove_file(from)?;
    }
    Ok(())
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

    fn symlink(path: &str) -> Document {
        Document::Symlink(RelPath::for_test(path))
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
        assert!(actual.iter().all(|d| matches!(d, Document::File(_))));
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

    #[test]
    fn normalize_does_not_treat_symlink_as_a_container() {
        // A symlink is a leaf entry, not a directory it could nest paths under.
        let documents = vec![symlink("a"), file("a/b")];
        assert_eq!(normalize(documents.clone()), documents);
    }

    #[test]
    fn collect_documents_does_not_resolve_symlinks() {
        let tmp = std::env::temp_dir().join(format!(
            "tff-ops-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        let link = tmp.join("link");
        std::os::unix::fs::symlink(tmp.join("missing_target"), &link).unwrap();

        let actual = collect_documents(&tmp, &[PathBuf::from("link")]).unwrap();

        std::fs::remove_dir_all(&tmp).unwrap();

        assert_eq!(actual, vec![Document::Symlink(RelPath::for_test("link"))]);
    }

    #[test]
    fn lexically_absolute_resolves_dot_and_dot_dot_without_touching_the_filesystem() {
        let origin = PathBuf::from("/origin/nested");
        assert_eq!(
            lexically_absolute(&origin, &PathBuf::from("../a/./b")),
            PathBuf::from("/origin/a/b")
        );
        assert_eq!(
            lexically_absolute(&origin, &PathBuf::from("/elsewhere/c")),
            PathBuf::from("/elsewhere/c")
        );
    }
}
