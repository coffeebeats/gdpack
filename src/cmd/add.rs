use anyhow::anyhow;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

use crate::config::manifest::Manifest;
use crate::config::manifest::Query;
use crate::config::Configuration;
use crate::config::Parsable;
use crate::config::Persistable;
use crate::core::Addon;
use crate::core::Dependency;
use crate::core::Source;
use crate::git;

/* -------------------------------------------------------------------------- */
/*                                Struct: Args                                */
/* -------------------------------------------------------------------------- */

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Add a development-only dependency (will not be propagated to dependents'
    /// installs). Note that a dependency can only be specified in one
    /// environment's dependencies at a time.
    #[arg(short, long)]
    pub dev: bool,

    /// Add the dependency only for `TARGET` (can be specified more than once
    /// and accepts multiple values delimited by `,`).
    #[arg(short, long, value_name = "TARGET", value_delimiter = ',', num_args = 1..)]
    pub target: Vec<String>,

    #[clap(flatten)]
    pub source: SourceArgs,
}

/* --------------------------- Struct: SourceArgs --------------------------- */

/// SourceArgs specifies the location and version of a Godot addon to install.
#[derive(clap::Args, Debug)]
pub struct SourceArgs {
    /// A URI to a directory or Git repository containing the addon to install.
    /// `gdpack` will search for a `plugin.cfg` and use the following selection
    /// strategy:
    ///     1. If only one plugin is found (i.e. `plugin.cfg`), that plugin is
    ///        selected.
    ///     2. If multiple plugins are found, then `--name` is required and the
    ///        plugin matching `NAME` will be used.
    ///     3. If no plugins are found (i.e. the directory/repository is an
    ///        asset-only addon) then `gdpack` will select one of two paths: (i)
    ///        an `addons/<BASENAME>` directory and (ii) the root of the
    ///        directory/repository, in that order. Note that in the latter
    ///        case (ii), `gdpack` will only install non-Git and non-`.`-
    ///        prefixed files into the project.
    #[arg(value_name = "URI", value_parser = Uri::parse)]
    pub uri: Uri,

    /// Install the addon named `NAME` from a multi-addon dependency or a
    /// dependency named differently than its addon. If omitted, assumed to be
    /// repository name or filepath base name.
    #[arg(short, long, value_name = "NAME")]
    pub name: Option<String>,

    #[clap(flatten)]
    pub rev: GitRevArgs,

    #[clap(flatten)]
    pub release: ReleaseArgs,
}

impl From<SourceArgs> for Dependency {
    fn from(value: SourceArgs) -> Self {
        let source = match value.uri {
            Uri::Path(path) => path.into(),
            Uri::Url(repo) => match (value.release.release, value.release.asset) {
                (Some(tag), Some(asset)) => Source::Release(
                    git::GitHubRelease::builder()
                        .repo(repo.into())
                        .tag(tag)
                        .asset(asset)
                        .build(),
                ),
                _ => Source::Git(
                    git::Source::builder()
                        .repo(repo.into())
                        .reference(<Option<git::Reference>>::from(value.rev))
                        .build(),
                ),
            },
        };

        Dependency::builder()
            .addon(value.name.or(source.name()))
            .source(source)
            .build()
    }
}

/* --------------------------- Struct: ReleaseArgs -------------------------- */

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = true)]
pub struct ReleaseArgs {
    /// Use a git `RELEASE` version (only used with a git repository `URI`)
    #[arg(long = "release", value_name = "RELEASE", requires = "asset")]
    pub release: Option<String>,

    /// A specific `ASSET` from a git `RELEASE` version (only used with a git
    /// repository `URI` and `RELEASE`)
    #[arg(long, value_name = "ASSET", requires = "release")]
    pub asset: Option<String>,
}

/* --------------------------- Struct: GitRevArgs --------------------------- */

/// GitRevArgs specifies a particular commit within a Git repository.
#[derive(clap::Args, Debug)]
#[group(required = false, multiple = false)]
pub struct GitRevArgs {
    /// Use a git `BRANCH` version (only used with a git repository `URI`)
    #[arg(long, value_name = "BRANCH")]
    pub branch: Option<String>,

    /// Use a git `REV` version (only used with a git repository `URI`)
    #[arg(long, value_name = "REV")]
    pub rev: Option<String>,

    /// Use a git `TAG` version (only used with a git repository `URI`)
    #[arg(long, value_name = "TAG")]
    pub tag: Option<String>,
}

impl From<GitRevArgs> for Option<git::Reference> {
    fn from(value: GitRevArgs) -> Self {
        match value {
            GitRevArgs { rev: Some(r), .. } => Some(git::Reference::Rev(r)),
            GitRevArgs { tag: Some(t), .. } => Some(git::Reference::Tag(t)),
            GitRevArgs {
                branch: Some(b), ..
            } => Some(git::Reference::Branch(b)),
            _ => None,
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                              Function: handle                              */
/* -------------------------------------------------------------------------- */

pub fn handle(project: Option<impl AsRef<Path>>, mut args: Args) -> anyhow::Result<()> {
    let path_project = super::parse_project(project)?;

    let path_manifest = path_project.join(Manifest::file_name().unwrap());
    let mut m = Manifest::parse_file(&path_manifest)
        .map_err(|_| anyhow!("missing 'gdpack.toml' manifest; try calling 'gdpack init'"))?;

    args.source
        .uri
        .relative_to(&path_manifest)
        .map_err(|e| anyhow!(e))?;

    let mut dep = Dependency::from(args.source).rooted_at(&path_project);
    dep.is_direct = true;

    // Determine whether an installation is required by default. This is the
    // case when there is no "addons" directory or the [`Addon`] isn't found.
    let path_addons = path_project.as_path().join("addons");
    let mut should_install = !path_addons.as_path().is_dir();

    let targets = match args.target.is_empty() {
        true => vec![None],
        false => args.target.iter().map(String::as_str).map(Some).collect(),
    };

    let mut logs: Vec<String> = vec![];

    for target in &targets {
        if target.as_ref().is_some_and(|t| t.is_empty()) {
            return Err(anyhow!("missing target"));
        }

        // Install if the [`Addon`] isn't present in the project's "addons"
        // folder, even if the [`Manifest`] doesn't change.
        should_install = should_install
            || (target.is_none()
                && !Addon::try_from(&dep).is_ok_and(|a| {
                    let path_existing = path_addons.join(a.subfolder);
                    if !path_existing.is_dir() {
                        return false;
                    }

                    dep.addon.as_ref().is_some_and(|name| {
                        Addon::find_in_dir(path_existing, name).is_ok_and(|existing| {
                            a.version.is_some_and(|next| {
                                existing.version.is_some_and(|prev| next == prev)
                            })
                        })
                    })
                }));

        if dep.addon.as_ref().is_none() {
            return Err(anyhow!("missing dependency name"));
        }

        let prev = m
            .addons_mut(
                &Query::builder()
                    .dev(args.dev)
                    .target(target.map(str::to_owned))
                    .build(),
            )
            .insert(&dep);

        if prev.is_none() || prev.is_some_and(|p| p != dep) {
            // Fetch the [`Dependency`] before continuing.
            dep.download()?;

            // Install if the [`Manifest`] was modified somehow. Note that the
            // implicit installation performed by `gdpack` manifest
            // modification commands should never use a target.
            should_install = should_install || target.is_none();

            logs.push(format!(
                "added dependency{}: {}",
                match target {
                    None => "".to_owned(),
                    Some(t) => format!(" in target '{}'", t),
                },
                dep.addon.as_ref().unwrap_or(&String::from("unknown")),
            ));
        }
    }

    if should_install {
        let install = crate::core::Install::builder()
            .dev(args.dev)
            .manifest(&m)
            .targets(targets)
            .build();

        install.install_to(path_addons)?;
    }

    m.persist(path_manifest)
        .map_err(|e| anyhow!("failed to persist manifest: {:}", e))?;

    for log in logs {
        println!("{}", log);
    }

    Ok(())
}

/* -------------------------------------------------------------------------- */
/*                                  Enum: Uri                                 */
/* -------------------------------------------------------------------------- */

/// Uri contains a specification of where the addon source code is located.
#[derive(Clone, Debug, PartialEq)]
pub enum Uri {
    Url(Url),
    Path(PathBuf),
}

/* -------------------------------- Impl: Uri ------------------------------- */

impl Uri {
    /* --------------------------- Methods: Public -------------------------- */

    /// Parse either a [`Url`] or a [`PathBuf`] from the provided [`str`].
    pub fn parse(s: &str) -> Result<Uri, UriError> {
        // NOTE: Parse a `Url` first as it's more specific than a `PathBuf`.
        if let Ok(u) = Url::parse(s) {
            return Ok(Uri::Url(u));
        }

        // TODO: Properly identify the plugin path according to documentation.
        if let Ok(p) = PathBuf::from_str(s) {
            if !p.exists() {
                return Err(UriError::NotFound(s.to_owned()));
            }

            if !p.is_dir() {
                return Err(UriError::NotADir(s.to_owned()));
            }

            if let Ok(p) = p.canonicalize() {
                return Ok(Uri::Path(p));
            }
        }

        Err(UriError::Invalid(s.to_owned()))
    }

    /// Update the underlying [`Uri::Path`] variant to be relative to the
    /// specified directory, if possible.
    pub fn relative_to(&mut self, base: impl AsRef<Path>) -> std::io::Result<()> {
        let path = match self {
            Uri::Path(p) => p.canonicalize()?,
            Uri::Url(_) => {
                return Ok(());
            }
        };

        let mut out = PathBuf::new();

        let mut path_common_ancestor = base.as_ref().canonicalize()?;
        if path_common_ancestor.is_file() {
            path_common_ancestor = path_common_ancestor
                .parent()
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "no parent directory",
                ))?
                .to_path_buf();
        }

        loop {
            match path.strip_prefix(&path_common_ancestor) {
                Err(_) => {
                    out = out.join("..");
                }
                Ok(path_addon) => {
                    *self = Uri::Path(out.join(path_addon));
                    return Ok(());
                }
            }

            path_common_ancestor = match path_common_ancestor.parent() {
                Some(parent) => parent.to_path_buf(),
                None => return Ok(()), // No common ancestor; exit without updating.
            };
        }
    }
}

/* ----------------------------- Enum: UriError ----------------------------- */

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum UriError {
    #[error("could not parse uri: {0}")]
    Invalid(String),
    #[error("path is not a directory: {0}")]
    NotADir(String),
    #[error("path does not exist: {0}")]
    NotFound(String),
    #[error(transparent)]
    Path(<PathBuf as FromStr>::Err),
    #[error(transparent)]
    Url(url::ParseError),
}

/* -------------------------------------------------------------------------- */
/*                                 Mod: Tests                                 */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::io::Write;
    use std::path::PathBuf;

    use super::Uri;

    macro_rules! write_file {
        ($path:expr, $content:expr$(,)?) => {
            std::fs::create_dir_all($path.parent().expect("invalid path"))
                .expect("failed to create directory");

            std::fs::File::create($path)
                .and_then(|mut f| f.write_all($content.as_bytes()))
                .expect("failed to create file");
        };
    }

    /* ----------------------- Test: Uri::relative_to ----------------------- */

    #[rstest]
    #[case("a", "b", "a")]
    #[case("a/b/c", "a/d/e", "../b/c")]
    #[case("a/b/c", "d", "a/b/c")]
    fn test_uri_relative_to_returns_correct_path(
        #[case] target: &str,
        #[case] from: &str,
        #[case] expected: &str,
    ) -> std::io::Result<()> {
        // Given: A temporary test directory for writing files.
        let tmp = tempfile::tempdir().unwrap();
        let base = tmp.into_path();

        // Given: Two files that exist within the temporary directory.
        let target = base.join(target);
        write_file!(&target, "");

        let from = base.join(from);
        write_file!(&from, "");

        // Given: A `Uri::Path`.
        let mut path = Uri::Path(target);

        // When: That `Uri::Path` is made relative to another.
        path.relative_to(from)?;

        // Then: The `Uri::Path` contains the correct relative path.
        assert_eq!(path, Uri::Path(PathBuf::from(expected)));

        Ok(())
    }
}
