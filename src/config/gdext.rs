use anyhow::anyhow;
use ini::Ini;

use super::Configuration;
use super::Parsable;
use super::ParsableError;

/* -------------------------------------------------------------------------- */
/*                              Struct: Extension                             */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug, Default)]
pub struct Extension(Ini);

/* --------------------------- Impl: Configuration -------------------------- */

impl Configuration for Extension {
    fn matches(path: impl AsRef<std::path::Path>) -> bool {
        path.as_ref()
            .extension()
            .is_some_and(|s| s == "gdextension")
    }
}

/* ----------------------------- Impl: Parsable ----------------------------- */

impl Parsable for Extension {
    fn parse(contents: &str) -> Result<Self, ParsableError> {
        Ini::load_from_str(contents)
            .map(Extension)
            .map_err(|e| ParsableError::Parse(anyhow!(e)))
    }
}
