use anyhow::anyhow;
use std::borrow::Borrow;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use toml_edit::Document;
use toml_edit::InlineTable;
use toml_edit::TableLike;
use typed_builder::TypedBuilder;

use crate::addon::Dependency;

const MANIFEST_SECTION_KEY_ADDONS: &str = "addons";
const MANIFEST_SECTION_KEY_ADDONS_DEV: &str = "dev-addons";
const MANIFEST_FILENAME: &str = "gdpack.toml";
const MANIFEST_SECTION_KEY_PROJECT: &str = "project";
const MANIFEST_SECTION_KEY_TARGET: &str = "target";

/* -------------------------------------------------------------------------- */
/*                                 Enum: Error                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid document structure")]
    Structure,
}

/* -------------------------------------------------------------------------- */
/*                              Struct: Manifest                              */
/* -------------------------------------------------------------------------- */

#[derive(Clone)]
pub struct Manifest(Document);

/* ----------------------------- Impl: Manifest ----------------------------- */

impl Manifest {
    /* --------------------------- Methods: Public -------------------------- */

    pub fn addons(&self, query: &Query) -> Option<Addons> {
        // let addons = match query.target.as_ref() {
        //     None => self.0.as_table(),
        //     Some(t) => self
        //         .0
        //         .get(MANIFEST_SECTION_KEY_TARGET)
        //         .and_then(|v| v.as_table())
        //         .and_then(|table| table.get(t))
        //         .and_then(|t| t.as_table())?,
        // };

        // addons
        //     .get(query.addons_key())
        //     .and_then(|v| v.as_table_like())
        //     .map(Addons)

        todo!()
    }

    pub fn addons_mut(&mut self, query: &Query) -> Option<AddonsMut> {
        // let addons = match query.target.as_ref() {
        //     None => self.0.as_table_mut(),
        //     Some(t) => self
        //         .0
        //         .get_mut(MANIFEST_SECTION_KEY_TARGET)
        //         .and_then(|v| v.as_table_mut())
        //         .and_then(|table| table.get_mut(t))
        //         .and_then(|t| t.as_table_mut())?,
        // };

        // addons
        //     .get_mut(query.addons_key())
        //     .and_then(|v| v.as_table_like_mut())
        //     .map(AddonsMut)

        todo!()
    }

    pub fn clean(&mut self) {
        todo!()
    }
}

/* ----------------------------- Impl: Parsable ----------------------------- */

impl super::Parsable for Manifest {
    fn parse(contents: &str) -> Result<Self, super::ParsableError> {
        let mut doc = contents
            .parse::<Document>()
            .map_err(|e| super::ParsableError::Parse(anyhow!(e)))?;

        if !doc.contains_key(MANIFEST_SECTION_KEY_ADDONS) {
            doc.insert(MANIFEST_SECTION_KEY_ADDONS, toml_edit::table());
        }

        Ok(Manifest(doc))
    }
}

/* ---------------------------- Impl: Persistable --------------------------- */

impl super::Persistable for Manifest {
    fn file_ext<'a>() -> Option<&'a str> {
        Some("toml")
    }

    fn file_name<'a>() -> Option<&'a str> {
        Some(MANIFEST_FILENAME)
    }
}

/* ------------------------------ Impl: Default ----------------------------- */

impl Default for Manifest {
    fn default() -> Self {
        let mut doc = Document::new();

        doc.insert(MANIFEST_SECTION_KEY_ADDONS, toml_edit::table());

        Manifest(doc)
    }
}

/* -------------------------------------------------------------------------- */
/*                             Trait: Dependencies                            */
/* -------------------------------------------------------------------------- */

pub trait Dependencies {
    fn get(&self, key: &Key) -> Option<Dependency>;

    fn has(&self, key: &Key) -> bool {
        self.get(key).is_some()
    }
}

/* -------------------------------------------------------------------------- */
/*                           Trait: DependenciesMut                           */
/* -------------------------------------------------------------------------- */

pub trait DependenciesMut
where
    Self: Dependencies,
{
    fn add(&mut self, key: &Key, dep: Dependency) -> Option<Dependency>;

    fn remove(&mut self, key: &Key) -> Option<Dependency>;
}

/* -------------------------------------------------------------------------- */
/*                               Struct: Addons                               */
/* -------------------------------------------------------------------------- */

pub struct Addons<'a>(&'a Document);

/* --------------------------- Impl: Dependencies --------------------------- */

impl<'a> Dependencies for Addons<'a> {
    fn get(&self, key: &Key) -> Option<Dependency> {
        self.0
            .get(&key.name)
            .and_then(|v| v.as_table_like())
            .and_then(|t| Dependency::try_from(t).ok())
    }
}

/* -------------------------------------------------------------------------- */
/*                              Struct: AddonsMut                             */
/* -------------------------------------------------------------------------- */

pub struct AddonsMut<'a>(&'a mut Document);

/* --------------------------- Impl: Dependencies --------------------------- */

impl<'a> Dependencies for AddonsMut<'a> {
    fn get(&self, key: &Key) -> Option<Dependency> {
        Addons(self.0).get(key)
    }
}

/* -------------------------- Impl: DependenciesMut ------------------------- */

impl<'a> DependenciesMut for AddonsMut<'a> {
    fn add(&mut self, key: &Key, dep: Dependency) -> Option<Dependency> {
        todo!()
    }

    fn remove(&mut self, key: &Key) -> Option<Dependency> {
        todo!()
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Struct: Key                                */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Key {
    name: String,
    query: Query,
}

/* -------------------------------- Impl: Key ------------------------------- */

impl Key {
    fn get(&self, doc: &Document) -> Option<&dyn toml_edit::TableLike> {
        todo!()
    }

    fn get_mut(&self, doc: &Document) -> Option<&dyn toml_edit::TableLike> {
        todo!()
    }
}

/* -------------------------------------------------------------------------- */
/*                                Struct: Query                               */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Query {
    dev: bool,
    #[builder(default)]
    target: Option<String>,
}

/* ------------------------------- Impl: Query ------------------------------ */

impl Query {
    /* -------------------------- Methods: Private -------------------------- */

    fn get<'a>(&self, doc: &'a Document) -> Option<&'a dyn toml_edit::TableLike> {
        match self.target.as_ref() {
            None => Some(doc.as_table() as &dyn toml_edit::TableLike),
            Some(t) => doc
                .get(MANIFEST_SECTION_KEY_TARGET)
                .and_then(|v| v.as_table_like())
                .and_then(|table| table.get(t))
                .and_then(|t| t.as_table_like()),
        }
    }

    fn get_mut<'a>(&self, doc: &'a Document) -> Option<&'a dyn toml_edit::TableLike> {
        todo!()
    }

    fn insert<'a>(&self, doc: &Document) -> Option<&dyn toml_edit::TableLike> {
        todo!()
    }

    fn path(&self) -> &'static str {
        match &self.dev {
            true => MANIFEST_SECTION_KEY_ADDONS_DEV,
            false => MANIFEST_SECTION_KEY_ADDONS,
        }
    }

    fn ensure_exists(&self, doc: &mut Document) {
        let key = self.path();

        match &self.target {
            None => {
                if !doc.contains_table(key) {
                    doc.insert(key, toml_edit::table());
                }
            }
            Some(t) => {
                if !doc.contains_table(MANIFEST_SECTION_KEY_TARGET) {
                    doc.insert(MANIFEST_SECTION_KEY_TARGET, toml_edit::table());
                }

                let targets = doc
                    .get_mut(MANIFEST_SECTION_KEY_TARGET)
                    .and_then(|v| v.as_table_like_mut())
                    .expect(&format!("missing table: {}", MANIFEST_SECTION_KEY_TARGET));

                if !targets.is_dotted() {
                    targets.set_dotted(true);
                }

                if !targets.contains_key(t) {
                    targets.insert(t, toml_edit::table());
                }

                let target = targets
                    .get_mut(t)
                    .and_then(|v| v.as_table_like_mut())
                    .expect(&format!(
                        "missing table: {}.{}",
                        MANIFEST_SECTION_KEY_TARGET, t
                    ));

                if !target.is_dotted() {
                    target.set_dotted(true);
                }

                if !target.contains_key(key) {
                    target.insert(key, toml_edit::table());
                }

                let section = target
                    .get_mut(key)
                    .and_then(|v| v.as_table_like_mut())
                    .expect(&format!(
                        "missing table: {}.{}.{}",
                        MANIFEST_SECTION_KEY_TARGET, t, key
                    ));

                if section.is_dotted() {
                    section.set_dotted(false);
                }
            }
        }
    }
}
