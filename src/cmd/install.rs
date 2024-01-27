use anyhow::anyhow;
use std::path::Path;
use typed_builder::TypedBuilder;

use crate::config::manifest::Manifest;
use crate::config::Configuration;
use crate::config::Parsable;

/* -------------------------------------------------------------------------- */
/*                                Struct: Args                                */
/* -------------------------------------------------------------------------- */

#[derive(clap::Args, Debug, TypedBuilder)]
pub struct Args {
    /// Add a development-only dependency (will not be propagated to dependents'
    /// installs).
    #[arg(long, alias = "prod")]
    pub production: bool,

    /// Add the dependency only for `TARGET` (can be specified more than once
    /// and accepts multiple values delimited by `,`).
    #[arg(short, long, value_name = "TARGET", value_delimiter = ',', num_args = 1..)]
    #[builder(default)]
    pub target: Vec<String>,
}

/* -------------------------------------------------------------------------- */
/*                              Function: handle                              */
/* -------------------------------------------------------------------------- */

pub fn handle(project: Option<impl AsRef<Path>>, args: Args) -> anyhow::Result<()> {
    let path_project = super::parse_project(project)?;

    let path_manifest = path_project.join(Manifest::file_name().unwrap());
    let m = Manifest::parse_file(path_manifest)
        .map_err(|_| anyhow!("missing 'gdpack.toml' manifest; try calling 'gdpack init'"))?;

    let targets = match args.target.is_empty() {
        true => vec![None],
        false => std::iter::once(None)
            .chain(args.target.iter().map(String::as_str).map(Some))
            .collect(),
    };

    let install = crate::core::Install::builder()
        .dev(!args.production)
        .manifest(&m)
        .targets(targets)
        .build();

    install.install_to(path_project)?;

    Ok(())
}
