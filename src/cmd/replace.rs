use std::path::PathBuf;

use super::add::SourceArgs;
use crate::addon::Addon;
use crate::addon::Spec;
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

    let mut m = manifest::parse_from(&path)?;

    let section = match args.dev {
        true => manifest::MANIFEST_SECTION_KEY_ADDONS_DEV,
        false => manifest::MANIFEST_SECTION_KEY_ADDONS,
    };

    let spec: Spec = args.source.into();

    let name = args.addon;
    let mut addon = Addon::new(spec, Some(name.to_owned()));

    if name == addon.name() {
        let _ = addon.replace.take();
    }

    if let Some(_) = m.add(section, &addon) {
        println!("updated dependency: '{}'", addon.name())
    }

    manifest::write_to(&m, &path)
}
