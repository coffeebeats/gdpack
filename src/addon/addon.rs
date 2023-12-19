use anyhow::anyhow;
use semver::Version;
use std::path::Path;
use std::path::PathBuf;

use crate::git;

use super::Plugin;
use super::Spec;

/* -------------------------------------------------------------------------- */
/*                                Struct: Addon                               */
/* -------------------------------------------------------------------------- */

#[derive(typed_builder::TypedBuilder)]
pub struct Addon {
    /// Addon name (used in multi-addon repositories).
    #[builder(default)]
    pub name: Option<String>,
    /// Name of an addon to replace during installs.
    #[builder(default)]
    pub replace: Option<String>,
    /// The source of the addon repository/directory.
    pub spec: Spec,
}

/* ------------------------------- Impl: Addon ------------------------------ */

impl Addon {
    pub fn copy_to(&self, out: impl AsRef<Path>) -> anyhow::Result<()> {
        if out.as_ref().exists() && !out.as_ref().is_dir() {
            return Err(anyhow!("expected a directory: {:?}", out.as_ref()));
        }

        Ok(())
    }

    pub fn dependencies(&self) -> anyhow::Result<Vec<Addon>> {
        todo!()
    }

    pub fn package(&self) -> Option<String> {
        match &self.spec {
            crate::addon::Spec::Git(g) => g.name(),
            crate::addon::Spec::Path(p) => {
                p.file_name().and_then(|s| s.to_str()).map(|s| s.to_owned())
            }
        }
    }

    pub fn subpath(&self) -> Option<PathBuf> {
        todo!()
    }

    pub fn version(&self) -> Option<Version> {
        let package = self.package()?;

        // Determine the path to the addon namespace directory.
        let mut path = match &self.spec {
            Spec::Path(p) => p.to_owned(),
            Spec::Git(g) => {
                let checkout = git::download(g).ok()?;
                checkout.path
            }
        };

        // First, check the directory root.
        path.push(PathBuf::from("plugin.cfg".to_owned()));
        if path.exists() && path.is_file() {
            // TODO: Parse version from 'plugin.cfg' file.
        }

        // Next, check the addon directory.
        path.pop();
        path.push(format!("addons/{}", self.name.as_ref().unwrap_or(&package)));
        if path.exists() && path.is_file() {
            // TODO: Parse version from 'plugin.cfg' file.
        }

        // TODO: Support other paths or search for the file.

        None
    }
}

impl Addon {
    fn plugin(&self) -> Plugin {
        todo!()
    }
}
