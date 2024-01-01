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
        let name = self.name.as_deref();

        match &self.spec {
            Spec::Path(p) => Addon::new(p.clone(), name),
            Spec::Git(s) => {
                let checkout = git::checkout(s)?;
                Addon::new(checkout.path, name)
            }
            Spec::Release(release) => {
                release.download()?;
                Addon::new(release.get_path()?, name)
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
