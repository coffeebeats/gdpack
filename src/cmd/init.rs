use anyhow::anyhow;
use std::path::PathBuf;

use crate::config::Configuration;
use crate::config::Manifest;
use crate::config::Parsable;
use crate::config::Persistable;

const STR_USAGE: &str = "
Thanks for using 'gdpack'!

Use `gdpack add` to add plugin dependencies to your project.

For example:
   gdpack add https://github.com/bitwes/Gut --tag 9.1.1 -d
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
    let path_project = super::parse_project(args.project)?;

    let path_manifest = path_project.join(Manifest::file_name().unwrap());
    if path_manifest.is_file() && Manifest::parse_file(&path_manifest).is_ok() {
        return Err(anyhow!(
            "manifest already exists: {}",
            path_manifest.to_str().unwrap()
        ));
    }

    Manifest::persist(
        &Manifest::default(),
        path_project.join(Manifest::file_name().unwrap()),
    )?;

    println!("{}", STR_USAGE);

    Ok(())
}
