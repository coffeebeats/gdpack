/* ------------------------------- Mod: addon ------------------------------- */

mod addon;

pub use addon::Addons;
pub use addon::AddonsMut;

/* ----------------------------- Mod: dependency ---------------------------- */

mod dependency;

/* -------------------------------- Mod: key -------------------------------- */

mod key;

pub use key::Key;
pub use key::Query;

/* -------------------------------------------------------------------------- */
/*                              Struct: Manifest                              */
/* -------------------------------------------------------------------------- */

use anyhow::anyhow;
use toml_edit::Document;

use super::Configuration;
use super::Parsable;
use super::ParsableError;

const MANIFEST_FILENAME: &str = "gdpack.toml";

#[derive(Clone, Debug)]
pub struct Manifest(Document);

/* ----------------------------- Impl: Manifest ----------------------------- */

impl Manifest {
    /* --------------------------- Methods: Public -------------------------- */

    pub fn file_name() -> &'static str {
        MANIFEST_FILENAME
    }

    pub fn addons(&self) -> Addons {
        Addons(&self.0)
    }

    pub fn addons_mut(&mut self) -> AddonsMut {
        AddonsMut(&mut self.0)
    }
}

/* --------------------------- Impl: Configuration -------------------------- */

impl Configuration for Manifest {
    fn matches(path: impl AsRef<std::path::Path>) -> bool {
        path.as_ref()
            .file_name()
            .is_some_and(|s| s == MANIFEST_FILENAME)
    }
}

/* ----------------------------- Impl: Parsable ----------------------------- */

impl Parsable for Manifest {
    fn parse(contents: &str) -> Result<Self, ParsableError> {
        let doc = contents
            .parse::<Document>()
            .map_err(|e| ParsableError::Parse(anyhow!(e)))?;

        // TODO: Add validation to ensure sections are correct.

        Ok(Manifest(doc))
    }
}

/* --------------------------- Impl: Into<String> --------------------------- */

impl From<&Manifest> for String {
    fn from(value: &Manifest) -> Self {
        value.0.to_string()
    }
}

/* ------------------------------ Impl: Default ----------------------------- */

impl Default for Manifest {
    fn default() -> Self {
        let doc = Document::new();

        // TODO: Define what the default manifest looks like.

        Manifest(doc)
    }
}
