use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::model::{Tag, TagName};

#[derive(Debug, Default, Serialize, Deserialize)]
struct Snapshot {
    #[serde(default)]
    tags: BTreeMap<TagName, Tag>,
}

/// In-memory view of the persisted tags.
///
/// Mutations stay in memory until [`Store::commit`]; dropping the store
/// without committing leaves the file untouched.
#[derive(Debug)]
pub(crate) struct Store {
    path: PathBuf,
    tags: BTreeMap<TagName, Tag>,
}

impl Store {
    pub(crate) fn load() -> anyhow::Result<Self> {
        Self::load_from(default_path()?)
    }

    fn load_from(path: PathBuf) -> anyhow::Result<Self> {
        let tags = match std::fs::read(&path) {
            Ok(bytes) => {
                serde_json::from_slice::<Snapshot>(&bytes)
                    .with_context(|| {
                        format!("failed to deserialize tags from '{}'", path.display())
                    })?
                    .tags
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => BTreeMap::new(),
            Err(e) => {
                return Err(e)
                    .with_context(|| format!("failed to read tags from '{}'", path.display()));
            }
        };
        Ok(Self { path, tags })
    }

    pub(crate) fn commit(self) -> anyhow::Result<()> {
        let dir = self
            .path
            .parent()
            .context("tags file path has no parent directory")?;
        std::fs::create_dir_all(dir)
            .with_context(|| format!("failed to create directory '{}'", dir.display()))?;

        let json = serde_json::to_string(&Snapshot { tags: self.tags })
            .context("failed to serialize tags")?;
        let tmp_path = self.path.with_extension("json.tmp");
        std::fs::write(&tmp_path, json)
            .with_context(|| format!("failed to write temporary file '{}'", tmp_path.display()))?;
        std::fs::rename(&tmp_path, &self.path).with_context(|| {
            format!(
                "failed to move temporary file '{}' into place at '{}'",
                tmp_path.display(),
                self.path.display()
            )
        })
    }

    pub(crate) fn insert(&mut self, name: TagName, tag: Tag) {
        self.tags.insert(name, tag);
    }

    pub(crate) fn contains(&self, name: &str) -> bool {
        self.tags.contains_key(name)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    /// Iterates in tag-name order.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&TagName, &Tag)> {
        self.tags.iter()
    }

    /// Looks up every name in `names`, failing only once all misses are known.
    ///
    /// # Errors
    /// Returns an error listing every name that has no matching tag.
    pub(crate) fn get_all<'s>(
        &'s self,
        names: &[TagName],
    ) -> anyhow::Result<Vec<(&'s TagName, &'s Tag)>> {
        self.ensure_all_exist(names)?;
        Ok(names
            .iter()
            .filter_map(|name| self.tags.get_key_value(name.as_str()))
            .collect())
    }

    /// Removes every named tag and returns them, deduplicating repeated names.
    ///
    /// # Errors
    /// Returns an error listing every name that has no matching tag; nothing is removed in that case.
    pub(crate) fn take_all(&mut self, names: &[TagName]) -> anyhow::Result<Vec<(TagName, Tag)>> {
        self.ensure_all_exist(names)?;
        Ok(names
            .iter()
            .filter_map(|name| self.tags.remove_entry(name.as_str()))
            .collect())
    }

    fn ensure_all_exist(&self, names: &[TagName]) -> anyhow::Result<()> {
        let missing: Vec<&str> = names
            .iter()
            .filter(|name| !self.tags.contains_key(name.as_str()))
            .map(TagName::as_str)
            .collect();
        anyhow::ensure!(
            missing.is_empty(),
            "tag(s) not found: {}",
            missing.join(", ")
        );
        Ok(())
    }
}

fn default_path() -> anyhow::Result<PathBuf> {
    let base = dirs::state_dir()
        .or_else(dirs::data_dir)
        .context("failed to determine state directory")?;
    Ok(base.join("tff").join("tags.json"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Mode;

    fn empty_store() -> Store {
        Store {
            path: PathBuf::from("/unused/tags.json"),
            tags: BTreeMap::new(),
        }
    }

    fn tag() -> Tag {
        Tag {
            origin: PathBuf::from("/origin"),
            mode: Mode::Move,
            documents: vec![],
        }
    }

    fn name(s: &str) -> TagName {
        TagName::new(s).unwrap()
    }

    #[test]
    fn load_from_missing_file_is_empty() {
        let store = Store::load_from(PathBuf::from("/nonexistent/tags.json")).unwrap();
        assert!(store.is_empty());
    }

    #[test]
    fn take_all_reports_every_missing_name_and_removes_nothing() {
        let mut store = empty_store();
        store.insert(name("a"), tag());
        let err = store
            .take_all(&[name("a"), name("b"), name("c")])
            .unwrap_err();
        assert!(err.to_string().contains("b, c"), "unexpected error: {err}");
        assert!(store.contains("a"));
    }

    #[test]
    fn take_all_removes_taken_tags_and_dedupes_names() {
        let mut store = empty_store();
        store.insert(name("a"), tag());
        let taken = store.take_all(&[name("a"), name("a")]).unwrap();
        assert_eq!(taken.len(), 1);
        assert!(store.is_empty());
    }

    #[test]
    fn commit_then_load_roundtrip() {
        let path = std::env::temp_dir().join(format!("tff-store-test-{}.json", std::process::id()));
        let _ = std::fs::remove_file(&path);

        let mut store = Store::load_from(path.clone()).unwrap();
        store.insert(name("abc"), tag());
        store.commit().unwrap();

        let reloaded = Store::load_from(path.clone()).unwrap();
        assert!(reloaded.contains("abc"));
        let _ = std::fs::remove_file(&path);
    }
}
