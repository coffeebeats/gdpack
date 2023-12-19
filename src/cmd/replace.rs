use anyhow::anyhow;
use std::path::PathBuf;

use super::add::SourceArgs;
use crate::addon::Addon;
use crate::manifest;

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
    let path = super::parse_project(args.project)?;

    let mut m = manifest::init_from(&path)?;

    let name = args.addon;
    let mut addon = Addon::builder()
        .name(args.source.name.to_owned())
        .replace(Some(name.to_owned()))
        .spec(args.source.into())
        .build();

    if name == addon.package().ok_or(anyhow!("missing addon name"))? {
        let _ = addon.replace.take();
    }

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

    println!("replaced dependency: {}", name);

    Ok(())
}
