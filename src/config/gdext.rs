use anyhow::anyhow;
use ini::Ini;

use super::Configuration;
use super::Parsable;
use super::ParsableError;

const GDEXTENSION_FILE_EXTENSION: &str = "gdextension";

/* -------------------------------------------------------------------------- */
/*                              Struct: Extension                             */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug, Default)]
pub struct Extension(Ini);

/* ----------------------------- Impl: Extension ---------------------------- */

impl Extension {
    pub fn extension() -> &'static str {
        GDEXTENSION_FILE_EXTENSION
    }
}

/* --------------------------- Impl: Configuration -------------------------- */

impl Configuration for Extension {
    fn matches(path: impl AsRef<std::path::Path>) -> bool {
        path.as_ref()
            .extension()
            .is_some_and(|s| s == GDEXTENSION_FILE_EXTENSION)
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
