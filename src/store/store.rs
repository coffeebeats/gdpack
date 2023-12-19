use anyhow::anyhow;
use std::path::PathBuf;

/* -------------------------------------------------------------------------- */
/*                                Struct: Addon                               */
/* -------------------------------------------------------------------------- */

pub struct Addon {
    pub spec: crate::addon::Spec,
    pub checkout: crate::git::Checkout,
}

/* ------------------------------- Impl: Addon ------------------------------ */

impl Addon {
    pub fn download(source: &crate::git::Source) -> anyhow::Result<Addon> {
        let remote = crate::git::Remote::from(source);

        let mut path = super::get_path()?;
        path.extend(&["git", "repo", &remote.name()?]);

        let repo = remote.fetch_to(path)?;

        let mut path = super::get_path()?;
        path.extend(&[
            "git",
            "checkout",
            &remote.name()?,
            &source.reference().rev(),
        ]);

        let checkout = repo.checkout_to(path, source.reference().clone())?;

        Ok(Addon {
            checkout,
            spec: crate::addon::Spec::Git(source.clone()),
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
