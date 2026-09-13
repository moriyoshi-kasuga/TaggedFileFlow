use std::borrow::Borrow;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub(crate) struct TagName(String);

impl TagName {
    pub(crate) fn new(name: impl Into<String>) -> anyhow::Result<Self> {
        let name = name.into();
        anyhow::ensure!(!name.is_empty(), "tag name must not be empty");
        Ok(Self(name))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for TagName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<String> for TagName {
    type Error = anyhow::Error;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Self::new(name)
    }
}

impl From<TagName> for String {
    fn from(name: TagName) -> Self {
        name.0
    }
}

impl Borrow<str> for TagName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TagName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A path guaranteed to be relative to a [`Tag::origin`] directory.
///
/// The only way to construct one is [`RelPath::strip_prefix`], so a
/// [`Document`] can never end up holding an absolute path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct RelPath(PathBuf);

impl RelPath {
    /// Strips `base` off `absolute`, keeping the remainder as a relative path.
    ///
    /// # Errors
    /// Returns an error if `absolute` is not located under `base`.
    pub(crate) fn strip_prefix(base: &Path, absolute: &Path) -> anyhow::Result<Self> {
        let relative = absolute.strip_prefix(base).with_context(|| {
            format!("'{}' is not under '{}'", absolute.display(), base.display())
        })?;
        Ok(Self(relative.to_path_buf()))
    }

    #[cfg(test)]
    pub(crate) fn for_test(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }
}

impl AsRef<Path> for RelPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Document {
    File(RelPath),
    Dir(RelPath),
    Symlink(RelPath),
}

impl Document {
    pub(crate) fn as_path(&self) -> &Path {
        match self {
            Document::File(path) | Document::Dir(path) | Document::Symlink(path) => path.as_ref(),
        }
    }

    pub(crate) fn is_dir(&self) -> bool {
        matches!(self, Document::Dir(_))
    }

    pub(crate) fn kind_label(&self) -> &'static str {
        match self {
            Document::File(_) => "file",
            Document::Dir(_) => "folder",
            Document::Symlink(_) => "symlink",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Mode {
    Move,
    Copy,
}

impl Mode {
    pub(crate) fn verb(self) -> &'static str {
        match self {
            Mode::Move => "move",
            Mode::Copy => "copy",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Tag {
    pub(crate) origin: PathBuf,
    pub(crate) mode: Mode,
    pub(crate) documents: Vec<Document>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_name_rejects_empty() {
        assert!(TagName::new("").is_err());
        assert!("".parse::<TagName>().is_err());
    }

    #[test]
    fn tag_name_roundtrips_through_serde() {
        let name: TagName = serde_json::from_str("\"abc\"").unwrap();
        assert_eq!(name.as_str(), "abc");
        assert!(serde_json::from_str::<TagName>("\"\"").is_err());
    }
}
