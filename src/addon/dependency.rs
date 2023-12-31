use anyhow::anyhow;
use typed_builder::TypedBuilder;

use crate::git;

use super::Addon;
use super::Spec;

/* -------------------------------------------------------------------------- */
/*                             Struct: Dependency                             */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Dependency {
    /// Addon name (used in multi-addon repositories).
    #[builder(default)]
    pub name: Option<String>,
    /// Name of an addon to replace during installs.
    #[builder(default)]
    pub replace: Option<String>,
    /// The source of the addon repository/directory.
    pub spec: Spec,
}

/* ---------------------------- Impl: Dependency ---------------------------- */

impl Dependency {
    pub fn install(&self) -> anyhow::Result<super::Addon> {
        match &self.spec {
            Spec::Path(p) => Addon::new(p.clone(), self.name.as_ref().map(|s| s.as_str())),
            Spec::Git(s) => {
                let checkout = git::checkout(&s)?;
                Addon::new(checkout.path, self.name.as_ref().map(|s| s.as_str()))
            }
            Spec::Release(release) => {
                let (tag, target_asset) = match &release.reference {
                    git::Reference::Release(tag, asset) => (tag, asset),
                    _ => Err(anyhow!("invalid specification"))?,
                };

                todo!()
            }
        }
    }

    pub fn package(&self) -> Option<String> {
        match &self.spec {
            Spec::Release(r) => r.repo.name(),
            Spec::Git(g) => g.repo.name(),
            Spec::Path(p) => p
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .map(str::to_owned),
        }
    }
}
