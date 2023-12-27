use anyhow::anyhow;
use std::path::PathBuf;

use super::Database;
use super::Reference;
use super::Source;

/* -------------------------------------------------------------------------- */
/*                             Function: checkout                             */
/* -------------------------------------------------------------------------- */

pub fn checkout(source: &Source) -> anyhow::Result<Checkout> {
    let db = Database::try_from(source)?;
    let checkout = db.checkout(&source.reference)?;

    Ok(checkout)
}

/* -------------------------------------------------------------------------- */
/*                              Struct: Checkout                              */
/* -------------------------------------------------------------------------- */

pub struct Checkout {
    pub path: PathBuf,
    pub reference: Reference,
}

/* ----------------------------- Impl: Checkout ----------------------------- */

impl Checkout {
    /* ----------------------------- Methods: Public ---------------------------- */

    pub fn get_path(repo: &git2::Repository, source: &Source) -> anyhow::Result<PathBuf> {
        let mut path = super::get_store_path()?;

        let obj = repo.revparse_single(&source.reference.to_string())?;

        let short_id = obj
            .short_id()
            .map(|id| id.as_str().map(|s| s.to_owned()))?
            .ok_or(anyhow!("couldn't parse revision"))?;

        path.extend(&[
            "git",
            "checkout",
            &super::Database::id(&source.repo)?,
            &short_id,
        ]);

        Ok(path)
    }
}
