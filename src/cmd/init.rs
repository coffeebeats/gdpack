use std::path::PathBuf;

use crate::manifest;

const STR_USAGE: &'static str = "
Use `gdpack add` to add plugin dependencies to your project.

For example:
   gdpack add https://github.com/bitwes/Gut --tag 9.1.1
";

/* -------------------------------------------------------------------------- */
/*                                Struct: Args                                */
/* -------------------------------------------------------------------------- */

#[derive(clap::Args, Debug)]
pub struct Args {
    /// A `PATH` to the Godot project containing the manifest.
    #[arg(short, long, value_name = "PATH")]
    pub project: Option<PathBuf>,
}

/* -------------------------------------------------------------------------- */
/*                              Function: handle                              */
/* -------------------------------------------------------------------------- */

pub fn handle(args: Args) -> anyhow::Result<()> {
    let path = super::parse_project(args.project)?;

    manifest::write_to(&manifest::Manifest::default(), &path)?;

    println!("{}", STR_USAGE);

    Ok(())
}
