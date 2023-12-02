use clap::value_parser;
use clap::Args;
use std::path::PathBuf;
use url::Url;

/* -------------------------------------------------------------------------- */
/*                               Struct: AddArgs                              */
/* -------------------------------------------------------------------------- */

#[derive(Args, Debug)]
pub struct AddArgs {
    /// Add a development-only dependency (will not be propagated to dependents'
    /// installs).
    ///
    /// During installation, this addon will be linked into the target addon
    /// installation directory for the project unless `--production` is
    /// specified.
    #[arg(short, long)]
    pub dev: bool,

    /// Install the addon named `NAME` from a multi-addon dependency; if
    /// omitted, assumed to be repository name or filepath base name.
    ///
    /// During installation, this addon will be linked into the target addon
    /// installation directory for the project.
    #[arg(short, long, value_name = "NAME")]
    pub name: Option<String>,

    /// A local filepath to the addon directory containing `plugin.cfg`.
    ///
    /// During installation, this addon will be linked into the target addon
    /// installation directory for the project.
    #[arg(short, long, value_parser = value_parser!(PathBuf))]
    pub path: Option<PathBuf>,

    /// Add the dependency only for `TARGET` (can be specified more than once).
    ///
    /// During installation, this addon will be linked into the target addon
    /// installation directory for the project if the `install` command
    /// specified a target of `TARGET`.
    #[arg(short, long, value_name = "TARGET")]
    pub target: Option<String>,

    /// A URI to a Git repository containing the addon to install. If the
    /// repository contains more than one addon, or does not have a `plugin.cfg`
    /// then `path` must be specified.
    ///
    /// During installation, this addon will be linked into the target addon
    /// installation directory for the project.
    #[arg(index = 0, value_parser = value_parser!(Url))]
    pub git: Url,
}

/* -------------------------- Function: handle_add -------------------------- */

pub fn handle_add(args: AddArgs) -> anyhow::Result<()> {
    match (args.git, args.path) {
        (Some(url), _) => {
            println!("Added dependency from '{:?}'", url)
        }
        (_, Some(path)) => {
            println!("Added dependency from '{:?}'", path)
        }
        _ => unreachable!(),
    }

    Ok(())
}
