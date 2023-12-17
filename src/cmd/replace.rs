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
    pub target: Option<Vec<String>>,

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
        .replace(Some(name.to_owned()))
        .spec(args.source.into())
        .build();

    if name == addon.name() {
        let _ = addon.replace.take();
    }

    if let Some(targets) = args.target {
        for target in targets {
            m.add(
                &manifest::Key::builder()
                    .dev(args.dev)
                    .target(target)
                    .build(),
                &addon,
            )?;
        }
    } else {
        m.add(&manifest::Key::builder().dev(args.dev).build(), &addon)?;
    }

    manifest::write_to(&m, &path)?;

    println!("replaced dependency: {}", name);

    Ok(())
}
