use std::path::PathBuf;

use crate::manifest;

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
    pub target: Option<Vec<String>>,

    /// The `NAME` of an installed addon to remove.
    #[arg(index = 1, required = true, value_name = "NAME")]
    pub name: String,
}

/* -------------------------------------------------------------------------- */
/*                              Function: handle                              */
/* -------------------------------------------------------------------------- */

pub fn handle(args: Args) -> anyhow::Result<()> {
    let path = super::parse_project(args.project)?;

    let mut m = manifest::init_from(&path)?;

    if let Some(targets) = args.target {
        for target in targets {
            m.remove(
                &manifest::Key::builder()
                    .dev(args.dev)
                    .target(target)
                    .build(),
                &args.name,
            )?;
        }
    } else {
        m.remove(&manifest::Key::builder().dev(args.dev).build(), &args.name)?;
    }

    manifest::write_to(&m, &path)?;

    println!("removed dependency: {}", &args.name);

    Ok(())
}
