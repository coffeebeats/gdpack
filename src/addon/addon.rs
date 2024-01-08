use std::path::Path;
use std::path::PathBuf;
use typed_builder::TypedBuilder;

use crate::config::gdext::Extension;
use crate::config::manifest::Dependency;
use crate::config::manifest::Manifest;
use crate::config::plugin::Plugin;

/* -------------------------------------------------------------------------- */
/*                                Struct: Addon                               */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug, TypedBuilder)]
pub struct Addon {
    #[allow(dead_code)]
    #[builder(default)]
    extension: Option<Extension>,
    #[allow(dead_code)]
    #[builder(default)]
    manifest: Option<Manifest>,
    #[allow(dead_code)]
    path: PathBuf,
    #[allow(dead_code)]
    #[builder(default)]
    plugin: Option<Plugin>,
}

/* ------------------------------- Impl: Addon ------------------------------ */

impl Addon {
    pub fn new(_: impl AsRef<Path>, _: Option<&str>) -> anyhow::Result<Addon> {
        todo!()
    }

    pub fn install_to(&self, _: impl AsRef<Path>) -> anyhow::Result<()> {
        todo!();
    }

    pub fn dependencies(&self) -> anyhow::Result<Vec<Dependency>> {
        todo!()
    }
}

/* -------------------------------------------------------------------------- */
/*                         Function: copy_recursively                         */
/* -------------------------------------------------------------------------- */

/// Clone files recursively from source to destination using the provided copy
/// function. See https://nick.groenen.me/notes/recursively-copy-files-in-rust/.
#[allow(dead_code)]
fn copy_recursively(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    copy_fn: fn(&Path, &Path) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(&dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            copy_recursively(entry.path(), dst.as_ref().join(entry.file_name()), copy_fn)?;
        } else {
            copy_fn(
                entry.path().as_ref(),
                dst.as_ref().join(entry.file_name()).as_ref(),
            )?;
        }
    }

    Ok(())
}
