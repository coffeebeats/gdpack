use anyhow::anyhow;
use std::path::PathBuf;

/* -------------------------------------------------------------------------- */
/*                                Struct: Addon                               */
/* -------------------------------------------------------------------------- */

pub struct Addon {
    pub spec: crate::addon::Spec,
    pub checkout: super::GitCheckout,
}

/* ------------------------------- Impl: Addon ------------------------------ */

impl Addon {
    pub fn checkout(spec: &crate::addon::git::Spec) -> anyhow::Result<Addon> {
        let remote = super::GitRemote::from(spec);
        let repo = remote.fetch()?;

        let checkout = repo.checkout(spec.commit.clone())?;

        Ok(Addon {
            checkout,
            spec: crate::addon::Spec::Git(spec.clone()),
        })
    }

    pub fn copy_to(&self, name: String, out: PathBuf) -> anyhow::Result<()> {
        if out.exists() && !out.is_dir() {
            return Err(anyhow!("expected a directory: {:?}", &out));
        }

        // TODO: Implement copy.

        Ok(())
    }
}

impl Addon {
    pub fn name(&self) -> anyhow::Result<String> {
        unimplemented!()
    }

    pub fn dependencies(&self) -> anyhow::Result<Vec<crate::addon::Spec>> {
        unimplemented!()
    }
}
