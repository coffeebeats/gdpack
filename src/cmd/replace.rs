use anyhow::anyhow;
use std::path::PathBuf;

use crate::addon::Addon;
use crate::config::manifest::Dependency;
use crate::config::manifest::Manifest;
use crate::config::manifest::Query;
use crate::config::Configuration;
use crate::config::Parsable;
use crate::config::Persistable;

use super::add::SourceArgs;
use super::install::handle as install;
use super::install::Args as InstallArgs;

/* -------------------------------------------------------------------------- */
/*                                Struct: Args                                */
/* -------------------------------------------------------------------------- */

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Replace a development-only dependency (will not be propagated to dependents' installs).
    #[arg(short, long)]
    pub dev: bool,

    /// A `PATH` to the Godot project containing the manifest.
    #[arg(short, long, value_name = "PATH")]
    pub project: Option<PathBuf>,

    /// Replace the dependency only for TARGET (can be specified more than once
    /// and accepts multiple values delimited by `,`).
    #[arg(short, long, value_name = "TARGET", value_delimiter = ',', num_args = 1..)]
    pub target: Vec<String>,

    /// The `NAME` of an installed addon to replace.
    #[arg(required = true, value_name = "NAME")]
    pub addon: String,

    #[clap(flatten)]
    pub source: SourceArgs,
}

/* -------------------------------------------------------------------------- */
/*                              Function: handle                              */
/* -------------------------------------------------------------------------- */

pub fn handle(args: Args) -> anyhow::Result<()> {
    let path_project = super::parse_project(args.project.as_ref())?;

    let path_manifest = path_project.join(Manifest::file_name().unwrap());
    let mut m = Manifest::parse_file(&path_manifest)
        .map_err(|_| anyhow!("missing 'gdpack.toml' manifest; try calling 'gdpack init'"))?;

    let mut dep = Dependency::from(args.source);
    dep.replace = Some(args.addon.clone());

    if &args.addon == dep.name().as_ref().ok_or(anyhow!("missing addon name"))? {
        let _ = dep.replace.take();
    }

    // Determine whether an installation is required by default. This is the
    // case when there is no "addons" directory or the [`Addon`] isn't found.
    let path_addons = path_project.as_path().join("addons");
    let mut should_install = !path_addons.as_path().is_dir();

    let targets = match args.target.is_empty() {
        true => vec![None],
        false => args.target.iter().map(Some).collect(),
    };

    for target in &targets {
        if target.as_ref().is_some_and(|t| t.is_empty()) {
            return Err(anyhow!("missing target"));
        }

        // Install if the [`Addon`] isn't present in the project's "addons"
        // folder, even if the [`Manifest`] doesn't change.
        should_install = should_install
            || (target.is_none()
                && !Addon::try_from(&dep).is_ok_and(|a| path_addons.join(a.subfolder).is_dir()));

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
            // Fetch the [`Dependency`] before continuing.
            dep.source.fetch()?;

            // Install if the [`Manifest`] was modified somehow. Note that the
            // implicit installation performed by `gdpack` manifest
            // modification commands should never use a target.
            should_install = should_install || target.is_none();

            println!(
                "added dependency{} to replace '{}': {}",
                match target {
                    None => "".to_owned(),
                    Some(t) => format!(" in target '{}'", t),
                },
                args.addon,
                dep.name().unwrap_or(String::from("unknown"))
            );
        }
    }

    m.persist(path_manifest)?;

    if !should_install {
        return Ok(());
    }

    install(
        InstallArgs::builder()
            .production(false)
            .project(args.project)
            .build(),
    )?;

    Ok(())
}
