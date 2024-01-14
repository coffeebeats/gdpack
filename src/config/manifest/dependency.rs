use anyhow::anyhow;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;
use toml_edit::de::ValueDeserializer;
use toml_edit::Item;
use typed_builder::TypedBuilder;

use crate::git;

/* -------------------------------------------------------------------------- */
/*                             Struct: Dependency                             */
/* -------------------------------------------------------------------------- */

/// Defines a single addon dependency for the project. A dependency can be
/// sourced from either a local path, a remote Git repository, or a release
/// asset of a hosted Git repository. [`Dependency`] also tracks whether the
/// addon it represents should replace another [`Dependency`] during installs.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct Dependency {
    /// A specific addon name used to distinguish between addons within a multi-
    /// addon repository. This value will be used to match against either a
    /// `plugin.cfg` file (by `plugin.name` property) or a `*.gdextension` file
    /// name. The containing directory of the matching file will be used as the
    /// target directory for installation.
    ///
    /// NOTE: If no matching `plugin.cfg` or `*.gdextension` file is found
    /// (i.e. if the entire root directory is to be included) then specifying
    /// this value is an error.
    #[builder(default)]
    pub addon: Option<String>,
    /// Name of an addon to replace during installation.
    ///
    /// NOTE: This value will not be propagated to consumers of this project.
    #[builder(default)]
    pub replace: Option<String>,
    /// A specification of the source location of the addon repository/directory.
    #[serde(flatten)]
    #[builder(setter(into))]
    pub source: Source,
}

/* ---------------------------- Impl: Dependency ---------------------------- */

impl Dependency {
    /// The name of the addon's project; this value is used as the key within
    /// the [`super::Manifest`] addon sections.
    pub fn name(&self) -> Option<String> {
        match &self.source {
            Source::Release(r) => r.repo.name(),
            Source::Git(g) => g.repo.name(),
            Source::Path { path } => path
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .map(str::to_owned),
        }
    }
}

/* ---------------------------- Impl: From<&Item> --------------------------- */

impl TryFrom<&Item> for Dependency {
    type Error = toml_edit::de::Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        value
            .to_string()
            .trim()
            .parse::<ValueDeserializer>()
            .and_then(Dependency::deserialize)
    }
}

/* -------------------------------------------------------------------------- */
/*                                Enum: Source                                */
/* -------------------------------------------------------------------------- */

/// [`Source`] specifies where the addon source code is located.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum Source {
    // NOTE: `Source::Release` must come before `Source::Git` because of their
    // shared use of the `git` property during serialization. `Source::Release`
    // is more specific and will only match for releases, whereas `Source::Git`
    // will match on any usage of the `git` within the serialized data.
    Release(git::GitHubRelease),
    Git(git::Source),
    Path { path: PathBuf },
}

/* ------------------------------ Impl: Source ------------------------------ */

impl Source {
    /// `fetch` retrieves the [`Dependency`] and stores it in the `gdpack`
    /// store. This method has no effect if the [`Dependency`] is already
    /// downloaded.
    pub fn fetch(&self) -> anyhow::Result<PathBuf> {
        let path = match self {
            Source::Path { path } => Ok(path.to_owned()),
            Source::Git(s) => crate::git::checkout(s)
                .map(|c| c.path)
                .map_err(|e| anyhow!(e)),
            Source::Release(release) => release
                .download()
                .and_then(|_| release.get_path())
                .map_err(|e| anyhow!(e)),
        }?;

        if !path.exists() {
            let err =
                std::io::Error::new(std::io::ErrorKind::NotFound, path.to_str().unwrap_or(""));

            return Err(anyhow!(err));
        }

        if !path.is_dir() {
            let err = std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("expected a directory: {}", path.to_str().unwrap_or("")),
            );

            return Err(anyhow!(err));
        }

        Ok(path)
    }
}

/* ------------------------- Impl: Into<Dependency> ------------------------- */

impl From<&Source> for Dependency {
    fn from(value: &Source) -> Self {
        Dependency::builder().source(value.to_owned()).build()
    }
}

impl From<Source> for Dependency {
    fn from(value: Source) -> Self {
        (&value).into()
    }
}

/* ---------------------------- Impl: From<Path> ---------------------------- */

impl<T> From<T> for Source
where
    T: AsRef<Path>,
{
    fn from(value: T) -> Self {
        Source::Path {
            path: value.as_ref().to_owned(),
        }
    }
}
