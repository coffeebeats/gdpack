use anyhow::anyhow;
use std::path::Path;

use crate::config::manifest::Manifest;
use crate::config::manifest::Query;
use crate::config::Configuration;
use crate::config::Parsable;
use crate::config::Persistable;
use crate::core::Addon;
use crate::core::Dependency;

use super::add::SourceArgs;

/* -------------------------------------------------------------------------- */
/*                                Struct: Args                                */
/* -------------------------------------------------------------------------- */

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Replace a development-only dependency (will not be propagated to dependents' installs).
    #[arg(short, long)]
    pub dev: bool,

    /// Replace the dependency only for TARGET (can be specified more than once
    /// and accepts multiple values delimited by `,`).
    ///
    /// NOTE: This value is **required** because a replacement specified in the
    /// default target should just be expressed as _replacing the entry in the
    /// [`Manifest`]_.
    #[arg(short, long, value_name = "TARGET", value_delimiter = ',', num_args = 0..)]
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

pub fn handle(project: Option<impl AsRef<Path>>, args: Args) -> anyhow::Result<()> {
    let path_project = super::parse_project(project)?;

    let path_manifest = path_project.join(Manifest::file_name().unwrap());
    let mut m = Manifest::parse_file(&path_manifest)
        .map_err(|_| anyhow!("missing 'gdpack.toml' manifest; try calling 'gdpack init'"))?;

    let mut dep = Dependency::from(args.source).rooted_at(&path_project);
    dep.replace = Some(args.addon.clone());

    if &args.addon == dep.addon.as_ref().ok_or(anyhow!("missing addon name"))? {
        let _ = dep.replace.take();
    }

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
                "added dependency{} to replace '{}': {}",
                match target {
                    None => "".to_owned(),
                    Some(t) => format!(" in target '{}'", t),
                },
                args.addon,
                dep.addon.as_ref().unwrap_or(&String::from("unknown"))
            ));
        }
    }

    if should_install {
        let install = crate::core::Install::builder()
            .dev(true)
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
