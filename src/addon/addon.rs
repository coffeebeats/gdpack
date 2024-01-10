use std::path::Path;
use std::path::PathBuf;

use crate::config::manifest::Dependency;
use crate::config::manifest::Source;

use super::Installable;

/* -------------------------------------------------------------------------- */
/*                                Struct: Addon                               */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug)]
pub struct Addon(PathBuf);

/* ------------------------------- Impl: Addon ------------------------------ */

impl Addon {
    pub fn new(path: impl AsRef<Path>) -> Result<Addon, Error> {
        if !path.as_ref().exists() {
            return Err(Error::Io(std::io::ErrorKind::NotFound.into()));
        }

        if !path.as_ref().is_dir() {
            return Err(Error::Io(std::io::ErrorKind::AlreadyExists.into()));
        }

        Ok(Addon(path.as_ref().to_owned()))
    }

    pub fn dependencies(&self) -> anyhow::Result<Vec<Dependency>> {
        todo!()
    }
}

/* ---------------------------- Impl: Installable --------------------------- */

impl Installable for Addon {
    fn install_to(&self, target: impl AsRef<Path>) -> Result<(), std::io::Error> {
        if !target.as_ref().exists() || !target.as_ref().is_dir() {
            return Err(std::io::ErrorKind::InvalidInput.into());
        }

        super::clone_recursively(self.0.as_path(), target, |src, dst| {
            std::fs::hard_link(src, dst)
        })
    }
}

/* ----------------------- Impl: TryFrom<&Dependency> ----------------------- */

impl TryFrom<&Dependency> for Addon {
    type Error = Error;

    fn try_from(value: &Dependency) -> Result<Self, Self::Error> {
        match &value.source {
            Source::Path { path } => Addon::new(path),
            Source::Git(s) => {
                let checkout = crate::git::checkout(s).map_err(Error::Git)?;
                Addon::new(checkout.path)
            }
            Source::Release(release) => {
                release.download().map_err(Error::Git)?;
                Addon::new(release.get_path().map_err(Error::Git)?)
            }
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Enum: Error                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(std::io::Error),
    #[error(transparent)]
    Git(crate::git::Error),
}
