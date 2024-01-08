use crate::config::manifest::Dependency;
use crate::config::manifest::Source;
use crate::git;

use super::Addon;

/* -------------------------------------------------------------------------- */
/*                             Struct: Dependency                             */
/* -------------------------------------------------------------------------- */

/* ---------------------------- Impl: Dependency ---------------------------- */

impl Dependency {
    pub fn install(&self) -> anyhow::Result<super::Addon> {
        let name = self.addon.as_deref();

        match &self.source {
            Source::Path { path } => Addon::new(&path, name),
            Source::Git(s) => Addon::new(git::checkout(s)?.path, name),
            Source::Release(release) => {
                release.download()?;
                Addon::new(release.get_path()?, name)
            }
        }
    }
}
