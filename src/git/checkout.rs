use std::path::PathBuf;

use super::Database;
use super::Reference;
use super::Source;

/* -------------------------------------------------------------------------- */
/*                             Function: checkout                             */
/* -------------------------------------------------------------------------- */

/// A helper function for downloading a git-based addon dependency; returns a
/// reference to the version-specific repository.
pub fn checkout(source: &Source) -> Result<Checkout, super::Error> {
    let db = Database::try_from(source)?;
    let checkout = db.checkout(source.reference.as_ref())?;

    Ok(checkout)
}

/* -------------------------------------------------------------------------- */
/*                              Struct: Checkout                              */
/* -------------------------------------------------------------------------- */

/// A handle to a version-specific checkout of a git-based Godot addon.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Checkout {
    pub path: PathBuf,
    pub reference: Option<Reference>,
}

/* ----------------------------- Impl: Checkout ----------------------------- */

impl Checkout {
    /* --------------------------- Methods: Public -------------------------- */

    /// Returns a path to the version-specific checkout for the specified
    /// [super::Remote] in the `gdpack` store.
    pub fn get_path(repo: &git2::Repository, source: &Source) -> Result<PathBuf, super::Error> {
        let mut path = super::get_store_path()?;

        let obj = repo
            .revparse_single(
                &source
                    .reference
                    .as_ref()
                    .map(Reference::to_string)
                    .unwrap_or(String::from("HEAD")),
            )
            .map_err(super::Error::Git)?;

        let short_id = obj
            .short_id()
            .map(|id| id.as_str().map(|s| s.to_owned()))
            .map_err(super::Error::Git)?
            .ok_or(super::Error::InvalidInput("couldn't parse revision".into()))?;

        path.extend(&[
            "git",
            "checkout",
            &super::Database::id(&source.repo)?,
            &short_id,
        ]);

        Ok(path)
    }
}
