use anyhow::anyhow;
use std::path::PathBuf;

use super::Checkout;
use super::Remote;
use super::Source;

const ENV_GDPACK_HOME: &str = "GDPACK_HOME";

/* -------------------------------------------------------------------------- */
/*                             Function: get_path                             */
/* -------------------------------------------------------------------------- */

/// `get_path` returns the path to the user's GDPack home directory, specified
/// by the `GDPACK_HOME` environment variable, or an `Err` if that path cannot
/// be determined.
pub(super) fn get_path() -> anyhow::Result<PathBuf> {
    std::env::var(ENV_GDPACK_HOME)
        .map(|p| PathBuf::from(p))
        .map_err(|e| anyhow!(e))
}

/* -------------------------------------------------------------------------- */
/*                             Function: download                             */
/* -------------------------------------------------------------------------- */

pub fn download(source: &Source) -> anyhow::Result<Checkout> {
    let remote = Remote::from(source);

    let mut path = get_path()?;
    path.extend(&["git", "repo", &remote.name()?]);

    let repo = remote.fetch_to(path)?;

    let mut path = get_path()?;
    path.extend(&[
        "git",
        "checkout",
        &remote.name()?,
        &source.reference().rev(),
    ]);

    repo.checkout_to(path, source.reference().clone())
}
