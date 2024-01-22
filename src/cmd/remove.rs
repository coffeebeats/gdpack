use anyhow::anyhow;
use std::path::PathBuf;

use crate::config::manifest::Manifest;
use crate::config::manifest::Query;
use crate::config::Configuration;
use crate::config::Parsable;
use crate::config::Persistable;

/* -------------------------------------------------------------------------- */
/*                                Struct: Args                                */
/* -------------------------------------------------------------------------- */

#[derive(clap::Args, Debug)]
pub struct Args {
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
    let path_project = super::parse_project(args.project.as_ref())?;

    let path_manifest = path_project.join(Manifest::file_name().unwrap());
    let mut m = Manifest::parse_file(&path_manifest)
        .map_err(|_| anyhow!("missing 'gdpack.toml' manifest; try calling 'gdpack init'"))?;

    // Determine whether an installation is required by default. This is the
    // case when there is no "addons" directory or the [`Addon`] isn't found.
    let path_addons = path_project.as_path().join("addons");
    let mut should_install = !path_addons.as_path().is_dir();

    let targets = match args.target.is_empty() {
        true => vec![None],
        false => args.target.iter().map(String::as_str).map(Some).collect(),
    };

    let name = args.name.as_str();

    let mut logs: Vec<String> = vec![];

    for target in &targets {
        if target.as_ref().is_some_and(|t| t.is_empty()) {
            return Err(anyhow!("missing target"));
        }

        // Remove the specified [`Dependency`] from *both* environments.

        let prev_prod = m
            .addons_mut(
                &Query::builder()
                    .dev(false)
                    .target(target.map(str::to_owned))
                    .build(),
            )
            .remove(name);

        let prev_dev = m
            .addons_mut(
                &Query::builder()
                    .dev(true)
                    .target(target.map(str::to_owned))
                    .build(),
            )
            .remove(name);

        let prev = prev_prod.or(prev_dev); // Prioritize a production dependency.
        if prev.is_some_and(|d| d.addon.is_some_and(|n| n == name)) {
            // Install if the [`Manifest`] was modified somehow. Note that the
            // implicit installation performed by `gdpack` manifest
            // modification commands should never use a target.
            should_install = should_install || target.is_none();

            logs.push(format!(
                "removed dependency{}: {}",
                match target {
                    None => "".to_owned(),
                    Some(t) => format!(" from target '{}'", t),
                },
                name,
            ));
        }
    }

    if should_install {
        let install = crate::core::Install::builder()
            .dev(true)
            .targets(targets)
            .root(&m)
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
