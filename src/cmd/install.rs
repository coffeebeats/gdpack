use anyhow::anyhow;
use std::path::PathBuf;
use typed_builder::TypedBuilder;

use crate::config::manifest::Manifest;
use crate::config::Configuration;
use crate::config::Parsable;
use crate::core::Installable;

/* -------------------------------------------------------------------------- */
/*                                Struct: Args                                */
/* -------------------------------------------------------------------------- */

#[derive(clap::Args, Debug, TypedBuilder)]
pub struct Args {
    /// Add a development-only dependency (will not be propagated to dependents'
    /// installs).
    #[arg(long, alias = "prod")]
    pub production: bool,

    /// A `PATH` to the Godot project containing the manifest.
    #[arg(short, long, value_name = "PATH")]
    #[builder(default)]
    pub project: Option<PathBuf>,

    /// Add the dependency only for `TARGET` (can be specified more than once
    /// and accepts multiple values delimited by `,`).
    #[arg(short, long, value_name = "TARGET", value_delimiter = ',', num_args = 1..)]
    #[builder(default)]
    pub target: Vec<String>,
}

/* -------------------------------------------------------------------------- */
/*                              Function: handle                              */
/* -------------------------------------------------------------------------- */

pub fn handle(args: Args) -> anyhow::Result<()> {
    let path_project = super::parse_project(args.project)?;

    let path_manifest = path_project.join(Manifest::file_name().unwrap());
    let m = Manifest::parse_file(path_manifest)
        .map_err(|_| anyhow!("missing 'gdpack.toml' manifest; try calling 'gdpack init'"))?;

    let targets = match args.target.is_empty() {
        true => vec![None],
        false => std::iter::once(None)
            .chain(args.target.iter().map(String::as_str).map(Some))
            .collect(),
    };

    let path_addons = path_project.as_path().join("addons");
    if path_addons.is_dir() {
        std::fs::remove_dir_all(&path_addons)
            .map_err(|e| anyhow!("failed to remove existing 'addons' directory: {}", e))?;
    }

    let install = crate::core::Install::builder()
        .dev(!args.production)
        .targets(targets)
        .root(&m)
        .build();

    let addons = install.resolve_addons()?;

    for addon in addons {
        addon.install_to(&path_addons)?;
    }

    Ok(())
}
