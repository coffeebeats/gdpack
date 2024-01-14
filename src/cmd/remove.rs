use anyhow::anyhow;
use std::path::PathBuf;

use crate::config::manifest::Manifest;
use crate::config::manifest::Query;
use crate::config::Configuration;
use crate::config::Parsable;
use crate::config::Persistable;

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

    /// The `NAME` of an installed addon to remove.
    #[arg(index = 1, required = true, value_name = "NAME")]
    pub name: String,
}

/* -------------------------------------------------------------------------- */
/*                              Function: handle                              */
/* -------------------------------------------------------------------------- */

pub fn handle(args: Args) -> anyhow::Result<()> {
    let path = super::parse_project(args.project.as_ref())?;

    let path_manifest = path.join(Manifest::file_name().unwrap());
    let mut m = Manifest::parse_file(&path_manifest)?;

    let targets = match args.target.is_empty() {
        true => vec![None],
        false => args.target.iter().map(Some).collect(),
    };

    let name = args.name.as_str();

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
            .remove(name);

        if prev.is_some_and(|d| d.name().is_some_and(|n| n == name)) {
            println!("removed dependency: {}", name);
        }
    }

    m.persist(path_manifest)?;

    install(
        InstallArgs::builder()
            .production(false)
            .project(args.project)
            .target(args.target)
            .build(),
    )?;

    Ok(())
}
