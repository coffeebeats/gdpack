use anyhow::anyhow;
use std::path::PathBuf;
use typed_builder::TypedBuilder;

use crate::addon::Addon;
use crate::addon::Installable;
use crate::config::manifest::Manifest;
use crate::config::manifest::Query;
use crate::config::Configuration;
use crate::config::Parsable;

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
        false => args.target.iter().map(Some).collect(),
    };

    let path_project_addons = path_project.as_path().join("addons");
    if path_project_addons.is_dir() {
        std::fs::remove_dir_all(path_project_addons)
            .map_err(|e| anyhow!("failed to remove existing 'addons' directory: {}", e))?;
    }

    for target in &targets {
        if target.as_ref().is_some_and(|t| t.is_empty()) {
            return Err(anyhow!("missing target"));
        }

        // TODO: Handle duplicated dependencies across targets.
        // TODO: Handle replacing dependencies.
        // TODO: Handle duplicated replaced dependencies across targets.

        for dep in m.addons(
            Query::builder()
                .dev(false)
                .target(target.map(String::as_str))
                .build(),
        ) {
            dep.source.fetch()?;

            let addon = Addon::try_from(&dep)
                .map_err(|e| anyhow!("failed to load addon from disk: {:}", e))?;

            addon
                .install_to(&path_project)
                .map_err(|e| anyhow!("failed to install addon: {:}", e))?;
        }

        if args.production {
            continue;
        }

        // TODO: Update [`Manifest`] and [`Query`] operations to return errors.
        // TODO: Update [`Query`] to only allow one addon per target.

        for dep in m.addons(
            Query::builder()
                .dev(true)
                .target(target.map(String::as_str))
                .build(),
        ) {
            Addon::try_from(&dep)?.install_to(&path_project)?;
        }
    }

    Ok(())
}
