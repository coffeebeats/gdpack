pub mod add;
pub mod init;
pub mod install;
pub mod remove;
pub mod replace;
pub mod update;

/* -------------------------------------------------------------------------- */
/*                               Enum: Commands                               */
/* -------------------------------------------------------------------------- */

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /* ----------------------- Category: Dependencies ----------------------- */
    /// Add the dependency at the provided 'URI'; can be a filepath or a URL to
    /// a Git repository.
    Add(add::Args),
    /// Remove the specified dependency.
    Remove(remove::Args),
    /// Replace a dependency with one at the provided URI; can be a filepath or a URL to a git repository.
    Replace(replace::Args),
    /// Update one or more remote addon dependencies to their latest version.
    Update(update::Args),

    /* --------------------------- Category: Init --------------------------- */
    /// Create a new `gdpack.toml` manifest for the Godot project.
    Init(init::Args),

    /* -------------------------- Category: Install ------------------------- */
    /// Install addon dependencies into the Godot project's addons/ directory.
    Install(install::Args),
}
