pub mod add;
pub mod init;
pub mod install;
pub mod remove;
pub mod replace;

/* -------------------------------------------------------------------------- */
/*                               Enum: Commands                               */
/* -------------------------------------------------------------------------- */

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /* ----------------------- Category: Dependencies ----------------------- */
    /// Add the dependency at the provided `URI`; can be a filepath or a URL to
    /// a Git repository.
    Add(add::Args),
    /// Remove the specified dependency.
    Remove(remove::Args),
    /// Replace a dependency with one at the provided URI; can be a filepath or a URL to a git repository.
    Replace(replace::Args),

    /* --------------------------- Category: Init --------------------------- */
    /// Create a new `gdpack.toml` manifest for the Godot project.
    Init(init::Args),

    /* -------------------------- Category: Install ------------------------- */
    /// Install addon dependencies into the Godot project's addons/ directory.
    #[clap(alias = "i")]
    Install(install::Args),
}

/* -------------------------------------------------------------------------- */
/*                           Function: parse_project                          */
/* -------------------------------------------------------------------------- */

use anyhow::anyhow;
use std::path::Path;
use std::path::PathBuf;

fn parse_project(project: Option<impl AsRef<Path>>) -> anyhow::Result<PathBuf> {
    let path: PathBuf;
    if let Some(project) = project {
        if !project.as_ref().is_dir() {
            return Err(anyhow!(
                "invalid argument: expected a directory for 'project'"
            ));
        }

        path = project.as_ref().to_owned()
    } else {
        path = std::env::current_dir()?;
    }

    Ok(path.join(crate::config::Manifest::file_name()))
}
