/* ------------------------------- Mod: addon ------------------------------- */

mod addon;

pub use addon::Addon;

/* ----------------------------- Mod: dependency ---------------------------- */

mod dependency;

pub use dependency::Dependency;
pub use dependency::Source;

/* ------------------------------ Mod: install ------------------------------ */

mod install;

pub use install::Install;

/* ------------------------------ Mod: project ------------------------------ */

mod project;

pub use project::ExportFiles;
pub use project::ScriptTemplateScan;
pub use project::ScriptTemplates;

/* -------------------------------------------------------------------------- */
/*                             Trait: Installable                             */
/* -------------------------------------------------------------------------- */

pub trait Installable {
    /// `install_to` installs the implementer (typically an [`Addon`]) into the
    /// _Godot_ project specified by `target`.
    fn install_to(&self, target: impl AsRef<Path>) -> std::io::Result<()>;
}

/* -------------------------------------------------------------------------- */
/*                         Function: clone_recursively                        */
/* -------------------------------------------------------------------------- */

use std::path::Path;

/// Clone files recursively from source to destination using the provided copy
/// function. See https://nick.groenen.me/notes/recursively-copy-files-in-rust/.
fn clone_recursively<F: Fn(&Path, &Path) -> std::io::Result<()>>(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    copy_fn: &F,
) -> std::io::Result<()> {
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;

        let target = dst.as_ref().join(entry.file_name());

        if entry.file_type()?.is_dir() {
            clone_recursively(entry.path(), &target, copy_fn)?;
        } else {
            std::fs::create_dir_all(&dst)?;

            copy_fn(entry.path().as_ref(), &target)?;
        }

        // NOTE: This is a hack, but there's no easy way to restructure this
        // to check before creating directories.
        if let Some(parent) = target.parent() {
            if std::fs::read_dir(parent).is_ok_and(|entries| entries.count() == 0) {
                std::fs::remove_dir(parent)?;
            }
        }
    }

    Ok(())
}
