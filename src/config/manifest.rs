use anyhow::anyhow;
use std::borrow::Borrow;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use toml_edit::Document;
use toml_edit::InlineTable;
use toml_edit::TableLike;

use crate::addon::Dependency;

const MANIFEST_SECTION_KEY_ADDONS: &str = "addons";
const MANIFEST_SECTION_KEY_ADDONS_DEV: &str = "dev-addons";
const MANIFEST_FILENAME: &str = "gdpack.toml";
const MANIFEST_SECTION_KEY_PROJECT: &str = "project";
const MANIFEST_SECTION_KEY_TARGET: &str = "target";

/* -------------------------------------------------------------------------- */
/*                              Struct: Manifest                              */
/* -------------------------------------------------------------------------- */

#[derive(Clone)]
pub struct Manifest(Document);

/* ----------------------------- Impl: Manifest ----------------------------- */

impl Manifest {
    pub fn addons(&self) -> Addons<'_, toml_edit::InlineTable> {
        todo!()
    }

    pub fn addons_mut(&mut self) -> AddonsMut<'_, toml_edit::InlineTable> {
        todo!()
    }

    pub fn target(&self) -> Addons<'_, toml_edit::Table> {
        todo!()
    }

    pub fn target_mut(&mut self) -> AddonsMut<'_, toml_edit::Table> {
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

/* --------------------------- Impl: Configurable --------------------------- */

impl super::Configurable for Manifest {
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
/*                               Struct: Addons                               */
/* -------------------------------------------------------------------------- */

pub struct Addons<'a, T>(&'a T)
where
    T: toml_edit::TableLike;

/* --------------------------- Impl: Dependencies --------------------------- */

impl<'a, T> Dependencies for Addons<'a, T>
where
    T: toml_edit::TableLike,
{
    fn has(&self, name: &str) -> bool {
        self.0.get(name).is_some()
    }

    fn get(&self, name: &str) -> Option<Dependency> {
        self.0
            .get(name)
            .and_then(|v| v.as_table_like())
            .and_then(|t| Dependency::try_from(t).ok())
    }
}

/* -------------------------------------------------------------------------- */
/*                              Struct: AddonsMut                             */
/* -------------------------------------------------------------------------- */

pub struct AddonsMut<'a, T>(&'a mut T)
where
    T: toml_edit::TableLike;

/* --------------------------- Impl: Dependencies --------------------------- */

impl<'a, T> Dependencies for AddonsMut<'a, T>
where
    T: toml_edit::TableLike,
{
    fn has(&self, name: &str) -> bool {
        Addons::<T>(&self.0).has(name)
    }

    fn get(&self, name: &str) -> Option<Dependency> {
        Addons::<T>(&self.0).get(name)
    }
}

/* -------------------------- Impl: DependenciesMut ------------------------- */

impl<'a, T> DependenciesMut for AddonsMut<'a, T>
where
    T: toml_edit::TableLike,
{
    fn add(&mut self, name: &str, dep: Dependency) -> Option<Dependency> {
        todo!()
    }

    fn remove(&mut self, name: &str) -> Option<Dependency> {
        todo!()
    }
}

/* -------------------------------------------------------------------------- */
/*                             Trait: Dependencies                            */
/* -------------------------------------------------------------------------- */

pub trait Dependencies {
    fn has(&self, name: &str) -> bool;

    fn get(&self, name: &str) -> Option<Dependency>;
}

/* -------------------------------------------------------------------------- */
/*                           Trait: DependenciesMut                           */
/* -------------------------------------------------------------------------- */

pub trait DependenciesMut
where
    Self: Dependencies,
{
    fn add(&mut self, name: &str, dep: Dependency) -> Option<Dependency>;

    fn remove(&mut self, name: &str) -> Option<Dependency>;
}
