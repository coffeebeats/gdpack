use anyhow::anyhow;
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

/* -------------------------------------------------------------------------- */
/*                                Struct: Args                                */
/* -------------------------------------------------------------------------- */

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Add a development-only dependency (will not be propagated to dependents'
    /// installs).
    #[arg(short, long)]
    pub dev: bool,

    /// Install the addon named `NAME` from a multi-addon dependency; if
    /// omitted, assumed to be repository name or filepath base name.
    #[arg(short, long, value_name = "NAME")]
    pub name: Option<String>,

    /// Add the dependency only for `TARGET` (can be specified more than once).
    #[arg(short, long, value_name = "TARGET")]
    pub target: Option<String>,

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
    #[arg(index = 1, required = true, value_name = "URI", value_parser = parse_source)]
    pub uri: Option<Source>,

    #[command(flatten)]
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

/* -------------------------------------------------------------------------- */
/*                              Function: handle                              */
/* -------------------------------------------------------------------------- */

pub fn handle(_: Args) -> anyhow::Result<()> {
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
fn parse_source(s: &str) -> Result<Source, anyhow::Error> {
    // NOTE: Parse a 'Url' first as it's more specific than a 'PathBuf'.
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

    return Err(anyhow!("could not parse source: {}", s));
}
