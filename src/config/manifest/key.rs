use toml_edit::Document;
use toml_edit::Item;
use typed_builder::TypedBuilder;

pub(super) const MANIFEST_SECTION_ADDONS: &str = "addons";
pub(super) const MANIFEST_SECTION_ADDONS_DEV: &str = "dev-addons";
pub(super) const MANIFEST_SECTION_TARGET: &str = "target";

/* -------------------------------------------------------------------------- */
/*                                 Struct: Key                                */
/* -------------------------------------------------------------------------- */

/// [`Key`] defines a key for accessing a particular addon within a specific
/// target and environment collection. It can be used to retrieve the
/// corresponding [`crate::addon::Dependency`] within the [`super::Manifest`].
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub(super) struct Key<'a> {
    #[builder(setter(into))]
    pub name: &'a str,
    pub query: &'a Query<'a>,
}

/* -------------------------------- Impl: Key ------------------------------- */

impl<'a> Key<'a> {
    /* --------------------------- Methods: Public -------------------------- */

    /// Retrieve the [`toml_edit::Item`] for the addon specified by the [`Key`],
    /// if it exists.
    pub(super) fn get<'b>(&self, doc: &'b Document) -> Option<&'b Item> {
        self.query.get(doc).and_then(|t| t.get(self.name))
    }

    /// Retrieve a mutable [`toml_edit::Item`] for the addon specified by the
    /// [`Key`], if it exists.
    #[allow(dead_code)]
    pub(super) fn get_mut<'b>(&self, doc: &'b mut Document) -> Option<&'b mut Item> {
        self.query.get_mut(doc).and_then(|t| t.get_mut(self.name))
    }

    /// Insert the provided [`toml_edit::Item`] into the provided
    /// [`toml_edit::Document`] under the path corresponding to the [`Key`].
    pub(super) fn insert(&self, doc: &mut Document, value: Item) -> Option<Item> {
        self.query
            .insert(doc)
            .and_then(|t| t.as_table_like_mut())
            .and_then(|t| t.insert(self.name, value))
    }

    /// Remove the [`toml_edit::Item`] at the path corresponding to [`Key`] from
    /// the provided [`toml_edit::Document`] and return it, if it exists.
    pub(super) fn remove(&self, doc: &mut Document) -> Option<Item> {
        self.query
            .get_mut(doc)
            .and_then(|t| t.as_table_like_mut())
            .and_then(|t| t.remove(self.name))
    }
}

/* -------------------------------------------------------------------------- */
/*                                Struct: Query                               */
/* -------------------------------------------------------------------------- */

/// [`Query`] defines a target- and environment-specific collection of addons
/// registered in the [`super::Manifest`]. The provided methods allow for
/// viewing and managing these addons as [`toml_edit::Item`] instances.
#[derive(Clone, Debug, Default, Eq, PartialEq, TypedBuilder)]
pub struct Query<'a> {
    pub dev: bool,
    #[builder(default)]
    pub target: Option<&'a str>,
}

/* ------------------------------- Impl: Query ------------------------------ */

impl<'a> Query<'a> {
    /* --------------------------- Methods: Public -------------------------- */

    /// `invert_dev` turns `self` into a new [`Query`] instance for the opposing
    /// addon environment (i.e. 'dev' vs. non-'dev').
    pub fn invert_dev(&self) -> Query<'a> {
        Query::builder().dev(!self.dev).target(self.target).build()
    }

    /* -------------------------- Methods: Private -------------------------- */

    pub(super) fn get<'b>(&self, doc: &'b Document) -> Option<&'b Item> {
        match self.target.as_ref() {
            None => doc.get(self.key_addons()),
            Some(target) => doc
                .get(MANIFEST_SECTION_TARGET)
                .and_then(|v| v.as_table_like())
                .and_then(|t| t.get(target))
                .and_then(|v| v.as_table_like())
                .and_then(|t| t.get(self.key_addons())),
        }
    }

    pub(super) fn get_mut<'b>(&self, doc: &'b mut Document) -> Option<&'b mut Item> {
        match self.target.as_ref() {
            None => doc.get_mut(self.key_addons()),
            Some(target) => doc
                .get_mut(MANIFEST_SECTION_TARGET)
                .and_then(|v| v.as_table_like_mut())
                .and_then(|t| t.get_mut(target))
                .and_then(|v| v.as_table_like_mut())
                .and_then(|t| t.get_mut(self.key_addons())),
        }
    }

    pub(super) fn is_empty(&self, doc: &Document) -> bool {
        self.get(doc)
            .and_then(|v| v.as_table_like())
            .map(|t| t.is_empty())
            .unwrap_or(true)
    }

    pub(super) fn insert<'b>(&self, doc: &'b mut Document) -> Option<&'b mut Item> {
        match self.target.as_ref() {
            None => Some(doc.entry(self.key_addons()).or_insert(toml_edit::table())),
            Some(t) => {
                let targets = doc
                    .entry(MANIFEST_SECTION_TARGET)
                    .or_insert(toml_edit::table())
                    .as_table_like_mut()
                    .expect("missing table"); // Document is assumed to be valid.

                if !targets.is_dotted() {
                    targets.set_dotted(true);
                }

                let target = targets
                    .entry(t)
                    .or_insert(toml_edit::table())
                    .as_table_like_mut()
                    .expect("missing table"); // Document is assumed to be valid.

                if !target.is_dotted() {
                    target.set_dotted(true);
                }

                let addons = target
                    .entry(self.key_addons())
                    .or_insert(toml_edit::table());

                Some(addons)
            }
        }
    }

    pub(super) fn remove(&self, doc: &mut Document) -> Option<Item> {
        match self.target.as_ref() {
            None => {
                let prev = doc.remove(self.key_addons());

                // NOTE: The default (i.e. "production") addons sections should
                // never be completely removed.
                if !self.dev {
                    self.insert(doc);
                }

                prev
            }
            Some(target) => {
                // Remove the target- and environment-specific section.
                let prev = doc
                    .get_mut(MANIFEST_SECTION_TARGET)
                    .and_then(|v| v.as_table_like_mut())
                    .and_then(|t| t.get_mut(target))
                    .and_then(|v| v.as_table_like_mut())
                    .and_then(|t| t.remove(self.key_addons()));

                // If this leaves the target-specific section empty, remove it.
                if doc
                    .get(MANIFEST_SECTION_TARGET)
                    .and_then(|v| v.as_table_like())
                    .and_then(|t| t.get(target))
                    .and_then(|v| v.as_table_like())
                    .is_some_and(|t| t.is_empty())
                {
                    doc.get_mut(MANIFEST_SECTION_TARGET)
                        .and_then(|v| v.as_table_like_mut())
                        .and_then(|t| t.remove(target));
                }

                // If this leaves the top-level 'target' key empty, remove it.
                if doc
                    .get(MANIFEST_SECTION_TARGET)
                    .and_then(|v| v.as_table_like())
                    .is_some_and(|t| t.is_empty())
                {
                    doc.remove(MANIFEST_SECTION_TARGET);
                }

                prev
            }
        }
    }

    fn key_addons(&self) -> &'static str {
        match &self.dev {
            true => MANIFEST_SECTION_ADDONS_DEV,
            false => MANIFEST_SECTION_ADDONS,
        }
    }
}
