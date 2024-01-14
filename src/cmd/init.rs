use std::path::PathBuf;

use crate::config::Configuration;
use crate::config::Manifest;
use crate::config::Persistable;

const STR_USAGE: &str = "
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

    Manifest::persist(
        &Manifest::default(),
        path.join(Manifest::file_name().unwrap()),
    )?;

    println!("{}", STR_USAGE);

    Ok(())
}
