use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;
use toml_edit::de::ValueDeserializer;
use toml_edit::Item;
use typed_builder::TypedBuilder;

use crate::git;

/* -------------------------------------------------------------------------- */
/*                                 Enum: Error                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Git(git::Error),
    #[error("insecure path found: {0}")]
    InsecurePath(PathBuf),
    #[error("cannot access path to dependency: {0}: {1}")]
    InvalidPath(PathBuf, std::io::Error),
    #[error("cannot determine path to dependency")]
    MissingPath,
    #[error("dependency not found: {0}")]
    NotFound(PathBuf),
}

/* -------------------------------------------------------------------------- */
/*                             Struct: Dependency                             */
/* -------------------------------------------------------------------------- */

/// Defines a single addon dependency for the project. A dependency can be
/// sourced from either a local path, a remote Git repository, or a release
/// asset of a hosted Git repository. [`Dependency`] also tracks whether the
/// addon it represents should replace another [`Dependency`] during installs.
#[derive(
    Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, TypedBuilder,
)]
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
    ///
    /// TODO: This value is not serialized into the [`super::Manifest`] table
    /// value. Instead, it's populated post-parse from the entry key. This
    /// should eventually be split out of this type and placed in some sort of
    /// wrapper struct which isn't used to serialize/deserialize TOML values.
    #[builder(default)]
    #[serde(skip)]
    pub addon: Option<String>,
    /// The on-disk path to the [`Dependency`] that included this one. Useful
    /// for resolving a [`Source`] specification with a relative path.
    ///
    /// NOTE: This must be manually defined after downloading/extracting the
    /// [`Dependency`] as it's dependent on install-time context. As such, it
    /// should not be serialized.
    #[builder(default)]
    #[serde(skip)]
    pub included_from: Option<PathBuf>,
    /// Name of an addon to replace during installation.
    ///
    /// NOTE: This value will not be propagated to consumers of this project.
    #[builder(default)]
    pub replace: Option<String>,
    /// A specification of the source location of the addon repository/directory.
    #[builder(setter(into))]
    #[serde(flatten)]
    pub source: Source,
}

/* ---------------------------- Impl: Dependency ---------------------------- */

impl Dependency {
    /* --------------------------- Methods: Public -------------------------- */

    /// `download` retrieves the [`Dependency`] and stores it in the `gdpack`
    /// store (defined by `$GDPACK_HOME`). This method has no effect if the
    /// [`Dependency`] is already downloaded.
    pub fn download(&self) -> Result<PathBuf, Error> {
        let path = match &self.source {
            Source::Git(s) => crate::git::checkout(s).map(|c| c.path).map_err(Error::Git),
            Source::Path { path } => self
                .included_from
                .as_ref()
                .ok_or(Error::MissingPath)
                .and_then(|path_root| Dependency::get_rooted_path(path_root, path)),
            Source::Release(release) => release
                .download()
                .and_then(|_| release.get_path())
                .map_err(Error::Git),
        }?;

        if !path.is_dir() {
            return Err(Error::NotFound(path));
        }

        Ok(path)
    }

    /// `rooted_at` returns a new [`Dependency`] with the `included_from` field
    /// set to the provided path. This is a convenience method for specifying
    /// where local dependencies of this [`Dependency`] should be relative to.
    pub fn rooted_at(&self, path: impl AsRef<Path>) -> Dependency {
        let mut dep = self.clone();
        dep.included_from.replace(path.as_ref().to_owned());

        dep
    }

    /* -------------------------- Methods: Private -------------------------- */

    /// `get_rooted_path` is a convenience function for turning a potentially
    /// relative path (`path`) into one joined onto `path_root`. Note that if
    /// `path` is absolute then it must be prefixed by `path_root`, otherwise
    /// an error will be returned.
    fn get_rooted_path(
        path_root: impl AsRef<Path>,
        path: impl AsRef<Path>,
    ) -> Result<PathBuf, Error> {
        let path_root = path_root.as_ref();
        let path = path.as_ref();

        let path = if path.is_absolute() {
            path.to_owned()
        } else {
            path_root.join(path)
        };

        match path.canonicalize() {
            Err(e) => return Err(Error::InvalidPath(path, e)),
            Ok(path) => {
                if path.strip_prefix(path_root).is_ok() {
                    return Ok(path);
                }
            }
        };

        Err(Error::InsecurePath(path.to_owned()))
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
#[derive(Clone, Debug, Deserialize, Hash, Ord, PartialEq, PartialOrd, Eq, Serialize)]
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
    /// The name of the [`Dependency`]'s project; this value is used as a fall-
    /// back key within the [`super::Manifest`] "addon" sections.
    pub fn name(&self) -> Option<String> {
        match self {
            Source::Release(r) => r.repo.name(),
            Source::Git(g) => g.repo.name(),
            Source::Path { path } => path
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .map(str::to_owned),
        }
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
