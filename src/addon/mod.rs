/* ------------------------------- Mod: Addon ------------------------------- */

mod addon;

pub use addon::Addon;

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
fn clone_recursively(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    copy_fn: fn(&Path, &Path) -> std::io::Result<()>,
) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            clone_recursively(entry.path(), dst.as_ref().join(entry.file_name()), copy_fn)?;
        } else {
            copy_fn(
                entry.path().as_ref(),
                dst.as_ref().join(entry.file_name()).as_ref(),
            )?;
        }
    }

    Ok(())
}
