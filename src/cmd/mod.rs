/* ----------------------------- Mod: Dependency ---------------------------- */

mod dependency;

pub use dependency::AddArgs;

pub use dependency::handle_add;

/* -------------------------------------------------------------------------- */
/*                               Enum: Commands                               */
/* -------------------------------------------------------------------------- */

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /* ----------------------- Category: Dependencies ----------------------- */
    /// Add the specified 'addon' as a dependency of a Godot project.
    Add(AddArgs),
}
