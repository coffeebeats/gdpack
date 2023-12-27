/* ------------------------------ Mod: Checkout ----------------------------- */

mod checkout;

pub use checkout::checkout;
pub use checkout::Checkout;

/* ------------------------------ Mod: Database ----------------------------- */

mod database;

pub use database::Database;

/* ------------------------------- Mod: Source ------------------------------ */

mod source;

pub use source::Reference;
pub use source::Remote;
pub use source::Source;

/* -------------------------------------------------------------------------- */
/*                          Function: get_store_path                          */
/* -------------------------------------------------------------------------- */

use anyhow::anyhow;
use std::path::PathBuf;

const ENV_GDPACK_HOME: &str = "GDPACK_HOME";

/// `get_store_path` returns the path to the user's GDPack home directory,
/// specified by the `GDPACK_HOME` environment variable, or an `Err` if that
/// path cannot be determined.
fn get_store_path() -> anyhow::Result<PathBuf> {
    std::env::var(ENV_GDPACK_HOME)
        .map(|p| PathBuf::from(p))
        .map_err(|e| anyhow!(e))
}
