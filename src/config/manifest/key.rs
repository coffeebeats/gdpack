use toml_edit::Document;
use toml_edit::Item;
use toml_edit::TableLike;
use typed_builder::TypedBuilder;

const MANIFEST_SECTION_KEY_ADDONS: &str = "addons";
const MANIFEST_SECTION_KEY_ADDONS_DEV: &str = "dev-addons";
const MANIFEST_SECTION_KEY_TARGET: &str = "target";

/* -------------------------------------------------------------------------- */
/*                                 Struct: Key                                */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Key {
    pub name: String,
    pub query: Query,
}

/* -------------------------------- Impl: Key ------------------------------- */

impl Key {
    /* --------------------------- Methods: Public -------------------------- */

    pub(super) fn get<'a>(&self, doc: &'a Document) -> Option<&'a dyn TableLike> {
        self.query
            .get(doc)
            .and_then(|t| t.get(&self.name))
            .and_then(|v| v.as_table_like())
    }

    pub(super) fn get_mut<'a>(&self, doc: &'a mut Document) -> Option<&'a mut dyn TableLike> {
        self.query
            .get_mut(doc)
            .and_then(|t| t.get_mut(&self.name))
            .and_then(|v| v.as_table_like_mut())
    }

    pub(super) fn insert<'a>(&self, doc: &'a mut Document, value: Item) -> Option<Item> {
        self.query
            .insert(doc)
            .and_then(|t| t.insert(&self.name, value))
    }
}

/* -------------------------------------------------------------------------- */
/*                                Struct: Query                               */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Query {
    pub dev: bool,
    #[builder(default)]
    pub target: Option<String>,
}

/* ------------------------------- Impl: Query ------------------------------ */

impl Query {
    /* -------------------------- Methods: Private -------------------------- */

    pub(super) fn get<'a>(&self, doc: &'a Document) -> Option<&'a dyn TableLike> {
        match self.target.as_ref() {
            None => doc.get(self.key_addons()).and_then(|t| t.as_table_like()),
            Some(t) => doc
                .get(MANIFEST_SECTION_KEY_TARGET)
                .and_then(|v| v.as_table_like())
                .and_then(|table| table.get(t))
                .and_then(|t| t.as_table_like())
                .and_then(|t| t.get(self.key_addons()))
                .and_then(|t| t.as_table_like()),
        }
    }

    pub(super) fn get_mut<'a>(&self, doc: &'a mut Document) -> Option<&'a mut dyn TableLike> {
        match self.target.as_ref() {
            None => doc
                .get_mut(self.key_addons())
                .and_then(|t| t.as_table_like_mut()),
            Some(t) => doc
                .get_mut(MANIFEST_SECTION_KEY_TARGET)
                .and_then(|v| v.as_table_like_mut())
                .and_then(|table| table.get_mut(t))
                .and_then(|t| t.as_table_like_mut())
                .and_then(|t| t.get_mut(self.key_addons()))
                .and_then(|t| t.as_table_like_mut()),
        }
    }

    pub(super) fn insert<'a>(&self, doc: &'a mut Document) -> Option<&'a mut dyn TableLike> {
        match self.target.as_ref() {
            None => doc
                .entry(self.key_addons())
                .or_insert(toml_edit::table())
                .as_table_like_mut(),
            Some(t) => {
                let targets = doc
                    .entry(MANIFEST_SECTION_KEY_TARGET)
                    .or_insert(toml_edit::table())
                    .as_table_like_mut()
                    .expect("missing table");

                if !targets.is_dotted() {
                    targets.set_dotted(true);
                }

                let target = targets
                    .entry(t)
                    .or_insert(toml_edit::table())
                    .as_table_like_mut()
                    .expect("missing table");

                if !target.is_dotted() {
                    target.set_dotted(true);
                }

                let addons = target
                    .entry(self.key_addons())
                    .or_insert(toml_edit::table())
                    .as_table_like_mut()
                    .expect("missing table");

                Some(addons)
            }
        }
    }

    fn key_addons(&self) -> &'static str {
        match &self.dev {
            true => MANIFEST_SECTION_KEY_ADDONS_DEV,
            false => MANIFEST_SECTION_KEY_ADDONS,
        }
    }
}
