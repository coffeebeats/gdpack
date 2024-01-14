use anyhow::anyhow;
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

use crate::config::manifest::Dependency;
use crate::config::manifest::Manifest;
use crate::config::manifest::Query;
use crate::config::manifest::Source;
use crate::config::Configuration;
use crate::config::Parsable;
use crate::config::Persistable;
use crate::git;

use super::install::handle as install;
use super::install::Args as InstallArgs;

/* -------------------------------------------------------------------------- */
/*                                Struct: Args                                */
/* -------------------------------------------------------------------------- */

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Add a development-only dependency (will not be propagated to dependents'
    /// installs).
    #[arg(short, long)]
    pub dev: bool,

    /// A `PATH` to the Godot project containing the manifest.
    #[arg(short, long, value_name = "PATH")]
    pub project: Option<PathBuf>,

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

    /// Install the addon named `NAME` from a multi-addon dependency; if
    /// omitted, assumed to be repository name or filepath base name.
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
            .addon(value.name.to_owned())
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

pub fn handle(args: Args) -> anyhow::Result<()> {
    let path = super::parse_project(args.project.as_ref())?;

    let path_manifest = path.join(Manifest::file_name().unwrap());
    let mut m = Manifest::parse_file(&path_manifest)?;

    let dep = Dependency::from(args.source);

    let targets = match args.target.is_empty() {
        true => vec![None],
        false => args.target.iter().map(Some).collect(),
    };

    for target in &targets {
        if target.as_ref().is_some_and(|t| t.is_empty()) {
            return Err(anyhow!("missing target"));
        }

        let prev = m
            .addons_mut(
                Query::builder()
                    .dev(args.dev)
                    .target(target.map(String::as_str))
                    .build(),
            )
            .insert(
                &dep.name()
                    .ok_or(anyhow!("missing dependency name"))?
                    .to_owned(),
                &dep,
            );

        if prev.is_none() || prev.is_some_and(|p| p != dep) {
            println!(
                "added dependency: {}",
                dep.name().unwrap_or(String::from("unknown"))
            );
        }
    }

    m.persist(path_manifest)
        .map_err(|e| anyhow!("failed to persist manifest: {:}", e))?;

    install(
        InstallArgs::builder()
            .production(false)
            .project(args.project)
            .target(args.target)
            .build(),
    )?;

    Ok(())
}

/* -------------------------------------------------------------------------- */
/*                                  Enum: Uri                                 */
/* -------------------------------------------------------------------------- */

/// Uri contains a specification of where the addon source code is located.
#[derive(Clone, Debug)]
pub enum Uri {
    Url(Url),
    Path(PathBuf),
}

/* -------------------------------- Impl: Uri ------------------------------- */

impl Uri {
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

            return Ok(Uri::Path(p));
        }

        Err(UriError::Invalid(s.to_owned()))
    }
}

/* ----------------------------- Enum: UriError ----------------------------- */

#[derive(Clone, Debug, thiserror::Error)]
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
