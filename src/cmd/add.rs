use anyhow::anyhow;
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

use crate::addon::git;
use crate::addon::Addon;
use crate::addon::Spec;
use crate::manifest;

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
    #[arg(value_name = "URI", value_parser = parse_source)]
    pub uri: Source,

    /// Install the addon named `NAME` from a multi-addon dependency; if
    /// omitted, assumed to be repository name or filepath base name.
    #[arg(short, long, value_name = "NAME")]
    pub name: Option<String>,

    #[clap(flatten)]
    pub commit: GitCommitArgs,
}

/* -------------------------- Struct: GitCommitArgs ------------------------- */

/// GitCommitArgs specifies a particular commit within a Git repository.
#[derive(clap::Args, Debug)]
#[group(required = false, multiple = false)]
pub struct GitCommitArgs {
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

/* ------------------------------- Impl: Into ------------------------------- */

impl From<SourceArgs> for Spec {
    fn from(value: SourceArgs) -> Self {
        match value.uri {
            Source::Path(path) => Spec::Path(path),
            Source::Url(repo) => Spec::Git(git::Spec::new(repo, value.commit.into())),
        }
    }
}

impl From<GitCommitArgs> for git::Commit {
    fn from(value: GitCommitArgs) -> Self {
        match value {
            GitCommitArgs {
                branch: None,
                rev: None,
                tag: None,
            } => git::Commit::Default,
            GitCommitArgs {
                branch: Some(b),
                rev: None,
                tag: None,
            } => git::Commit::Branch(b),
            GitCommitArgs {
                branch: None,
                rev: Some(r),
                tag: None,
            } => git::Commit::Rev(r),
            GitCommitArgs {
                branch: None,
                rev: None,
                tag: Some(t),
            } => git::Commit::Tag(t),
            _ => unreachable!(),
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                              Function: handle                              */
/* -------------------------------------------------------------------------- */

pub fn handle(args: Args) -> anyhow::Result<()> {
    let path = super::parse_project(args.project)?;

    let mut m = manifest::init_from(&path)?;

    let addon = Addon::builder().spec(args.source.into()).build();

    let targets = match args.target.is_empty() {
        true => vec![None],
        false => args.target.into_iter().map(Some).collect(),
    };

    for target in targets {
        if target.as_ref().is_some_and(|t| t.is_empty()) {
            return Err(anyhow!("missing target"));
        }

        m.add(
            &manifest::Key::builder()
                .dev(args.dev)
                .target(target)
                .build(),
            &addon,
        )?;
    }

    manifest::write_to(&m, &path)?;

    println!("added dependency: {}", addon.name());

    Ok(())
}

/* -------------------------------------------------------------------------- */
/*                                Enum: Source                                */
/* -------------------------------------------------------------------------- */

/// Source contains a specification of where the addon source code is located.
#[derive(Clone, Debug)]
pub enum Source {
    Url(Url),
    Path(PathBuf),
}

/* ------------------------- Function: parse_source ------------------------- */

/// parse_source attempts to parse either a URL or a filepath from the provided
/// string.
pub(in crate::cmd) fn parse_source(s: &str) -> Result<Source, anyhow::Error> {
    // NOTE: Parse a `Url` first as it's more specific than a `PathBuf`.
    if let Ok(u) = Url::parse(s) {
        return Ok(Source::Url(u));
    }

    // TODO: Properly identify the plugin path according to documentation.
    if let Ok(p) = PathBuf::from_str(s) {
        if !p.exists() {
            return Err(anyhow!("path does not exist: {}", s));
        }

        if !p.is_dir() {
            return Err(anyhow!("path is not a directory: {}", s));
        }

        return Ok(Source::Path(p));
    }

    Err(anyhow!("could not parse source: {}", s))
}
