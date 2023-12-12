use std::path::PathBuf;

use crate::manifest;

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

    let mut m = manifest::parse_from(&path)?;

    if let Some(_) = m.remove(manifest::MANIFEST_SECTION_KEY_ADDONS_DEV, &args.name) {
        println!("removed dependency: '{}'", &args.name);
        return Ok(());
    }

    if let Some(_) = m.remove(manifest::MANIFEST_SECTION_KEY_ADDONS, &args.name) {
        println!("removed dependency: '{}'", &args.name);
        return Ok(());
    }

    Ok(())
}
