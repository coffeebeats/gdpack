//! This module is heavily inspired by [Cargo](https://doc.rust-lang.org/cargo/guide/cargo-home.html)'s
//! implementation of git dependencies and provides operations for managing git-
//! based Godot addons.

/* ------------------------------ Mod: Checkout ----------------------------- */

mod checkout;

pub use checkout::checkout;
pub use checkout::Checkout;

/* ------------------------------ Mod: Database ----------------------------- */

mod database;

pub use database::Database;

/* ------------------------------ Mod: Release ------------------------------ */

mod release;

pub use release::GitHubRelease;

/* ------------------------------- Mod: Source ------------------------------ */

mod source;

pub use source::Reference;
pub use source::Remote;
pub use source::Source;

/* -------------------------------------------------------------------------- */
/*                          Function: get_store_path                          */
/* -------------------------------------------------------------------------- */

use std::path::PathBuf;

const ENV_GDPACK_HOME: &str = "GDPACK_HOME";

/// `get_store_path` returns the path to the user's GDPack home directory,
/// specified by the `GDPACK_HOME` environment variable, or an `Err` if that
/// path cannot be determined.
fn get_store_path() -> Result<PathBuf, Error> {
    std::env::var(ENV_GDPACK_HOME)
        .map(PathBuf::from)
        .map_err(Error::Env)
}

/* -------------------------------------------------------------------------- */
/*                                 Enum: Error                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Env(std::env::VarError),
    #[error(transparent)]
    Git(git2::Error),
    #[error(transparent)]
    Io(std::io::Error),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("missing input: {0}")]
    MissingInput(String),
    #[error(transparent)]
    Request(reqwest::Error),
    #[error("response failed: {0}")]
    Response(reqwest::StatusCode),
    #[error(transparent)]
    Url(url::ParseError),
    #[error(transparent)]
    Zip(zip::result::ZipError),
}
