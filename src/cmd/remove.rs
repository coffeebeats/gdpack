use clap::value_parser;
use std::path::PathBuf;

/* -------------------------------------------------------------------------- */
/*                                Struct: Args                                */
/* -------------------------------------------------------------------------- */

#[derive(clap::Args, Debug)]
pub struct Args {
    /// A `PATH` to the Godot project containing the manifest.
    #[arg(short, long, value_name = "PATH", value_parser = value_parser!(PathBuf))]
    pub project: Option<String>,

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

pub fn handle(_: Args) -> anyhow::Result<()> {
    Ok(())
}
