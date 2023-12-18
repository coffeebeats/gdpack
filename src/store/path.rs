use anyhow::anyhow;
use std::path::PathBuf;

const ENV_GDPACK_HOME: &str = "GDPACK_HOME";

/// `get_path` returns the path to the user's GDPack home directory, specified
/// by the `GDPACK_HOME` environment variable, or an `Err` if that path cannot
/// be determined.
pub fn get_path() -> anyhow::Result<PathBuf> {
    std::env::var(ENV_GDPACK_HOME)
        .map(|p| PathBuf::from(p))
        .map_err(|e| anyhow!(e))
}
