use serde::Serialize;
use toml_edit::ser::ValueSerializer;
use toml_edit::Document;
use toml_edit::InlineTable;

use crate::addon::Dependency;

use super::Key;
use super::Query;

/* -------------------------------------------------------------------------- */
/*                               Struct: Addons                               */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Addons<'a>(pub &'a Document);

/* ------------------------------ Impl: Addons ------------------------------ */

impl<'a> Addons<'a> {
    /* --------------------------- Methods: Public -------------------------- */

    pub fn get(&self, key: &Key) -> Option<Dependency> {
        key.get(&self.0).and_then(|t| Dependency::try_from(t).ok())
    }

    pub fn list(&self, query: &Query) -> Vec<Dependency> {
        query
            .get(&self.0)
            .map(|t| t.iter())
            .unwrap_or(Box::new(std::iter::empty()))
            .filter_map(|(_, v)| v.as_table_like())
            .map(Dependency::try_from)
            .filter_map(Result::ok)
            .collect()
    }
}

/* -------------------------------------------------------------------------- */
/*                              Struct: AddonsMut                             */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct AddonsMut<'a>(pub &'a mut Document);

/* ----------------------------- Impl: AddonsMut ---------------------------- */

impl<'a> AddonsMut<'a> {
    /* --------------------------- Methods: Public -------------------------- */

    pub fn get(&self, key: &Key) -> Option<Dependency> {
        Addons(self.0).get(key)
    }

    pub fn list(&self, query: &Query) -> Vec<Dependency> {
        Addons(self.0).list(query)
    }

    pub fn insert(&mut self, key: &Key, dep: &Dependency) -> Option<Dependency> {
        dep.serialize(ValueSerializer::new())
            .ok()
            .and_then(|v| v.as_inline_table().map(InlineTable::to_owned))
            .and_then(|t| key.insert(&mut self.0, toml_edit::value(t)))
            .and_then(|prev| {
                prev.as_table_like()
                    .map(Dependency::try_from)
                    .and_then(Result::ok)
            })
    }

    pub fn remove(&mut self, key: &Key) -> Option<Dependency> {
        let prev = key.get_mut(&mut self.0).and_then(|t| t.remove(&key.name))?;

        prev.as_table_like()
            .map(Dependency::try_from)
            .and_then(Result::ok)
    }
}
