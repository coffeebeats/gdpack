use anyhow::anyhow;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use toml_edit::Document;
use toml_edit::InlineTable;

use crate::addon::Addon;

pub const MANIFEST_FILENAME: &str = "gdpack.toml";
pub const MANIFEST_SECTION_KEY_ADDONS: &str = "addons";
pub const MANIFEST_SECTION_KEY_ADDONS_DEV: &str = "dev-addons";

const MANIFEST_TEMPLATE: &str = r#"[addons]
"#;

pub struct Manifest(Document);

impl Manifest {
    pub fn add(&mut self, key: &str, addon: &Addon) -> Option<Addon> {
        if !self.0.contains_table(key) {
            self.0.insert(key, toml_edit::table());
        }

        let next: InlineTable = addon.into();

        self.0
            .get_mut(key)
            .and_then(|v| v.as_table_mut())
            .and_then(|t| t.insert(&addon.name(), toml_edit::value(next)))
            .and_then(|v| v.as_table_like().and_then(|t| t.try_into().ok()))
    }

    pub fn remove(&mut self, key: &str, name: &str) -> Option<Addon> {
        let existing = self
            .0
            .get_mut(key)
            .and_then(|v| v.as_table_like_mut())
            .and_then(|t| t.remove(name))
            .and_then(|v| v.as_table_like().and_then(|t| t.try_into().ok()));

        if self
            .0
            .get(key)
            .and_then(|v| v.as_table_like())
            .is_some_and(|t| t.is_empty())
        {
            self.0.remove(key);
        }

        existing
    }
}

impl Default for Manifest {
    fn default() -> Self {
        let doc = MANIFEST_TEMPLATE.parse::<Document>().unwrap();
        Manifest(doc)
    }
}

pub fn parse_from(path: &Path) -> anyhow::Result<Manifest> {
    if !path.exists() {
        return Err(anyhow!("missing input file"));
    }

    if !path.is_file() {
        return Err(anyhow!("expected path to a file"));
    }

    let contents = std::fs::read_to_string(path)?;
    let doc = contents.parse::<Document>()?;

    Ok(Manifest(doc))
}

pub fn write_to(doc: &Manifest, path: &Path) -> anyhow::Result<()> {
    if path.exists() && (!path.is_file() || !path.ends_with(MANIFEST_FILENAME)) {
        return Err(anyhow!("invalid filepath"));
    }

    File::create(path)?
        .write_all(doc.0.to_string().as_bytes())
        .map_err(|e| anyhow!(e))
}
