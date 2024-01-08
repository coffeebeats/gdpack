/* ------------------------------- Mod: addon ------------------------------- */

mod addon;

pub use addon::Addons;
pub use addon::AddonsMut;

/* ----------------------------- Mod: dependency ---------------------------- */

mod dependency;

pub use dependency::Dependency;
pub use dependency::Source;

/* -------------------------------- Mod: key -------------------------------- */

mod key;

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

/// A wrapper around a formatted [`toml_edit::Document`] that provides
/// operations to manage [`Dependency`] and configuration information for a
/// Godot project.
#[derive(Clone, Debug)]
pub struct Manifest(Document);

/* ----------------------------- Impl: Manifest ----------------------------- */

impl Manifest {
    /* --------------------------- Methods: Public -------------------------- */

    /// The file name associated with [`Manifest`] files.
    pub fn file_name() -> &'static str {
        MANIFEST_FILENAME
    }

    /// Returns an _immutable_ view of the addons recorded for the provided
    /// [`Query`].
    pub fn addons(&self, query: Query) -> Addons {
        Addons::builder().document(&self.0).query(query).build()
    }

    /// Returns a mutable view of the addons recorded for the provided [`Query`].
    pub fn addons_mut(&mut self, query: Query) -> AddonsMut {
        AddonsMut::builder()
            .document(&mut self.0)
            .query(query)
            .build()
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

/* -------------------------- Impl: From<Document> -------------------------- */

impl From<Document> for Manifest {
    fn from(value: Document) -> Self {
        Manifest(value)
    }
}

/* ------------------------------ Impl: Default ----------------------------- */

impl Default for Manifest {
    fn default() -> Self {
        let mut doc = Document::new();

        doc.insert(key::MANIFEST_SECTION_ADDONS, toml_edit::table());

        Manifest(doc)
    }
}
