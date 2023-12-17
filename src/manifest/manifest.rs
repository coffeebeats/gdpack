use anyhow::anyhow;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use toml_edit::Document;
use toml_edit::InlineTable;
use toml_edit::TableLike;

use crate::addon::Addon;

use super::key::MANIFEST_SECTION_KEY_ADDONS;

pub const MANIFEST_FILENAME: &str = "gdpack.toml";
pub const MANIFEST_SECTION_KEY_TARGET: &str = "target";

pub struct Manifest(Document);

impl Manifest {
    pub fn add(&mut self, key: &super::Key, addon: &Addon) -> anyhow::Result<()> {
        let section = self.get_section_mut(key)?;

        let next: InlineTable = addon.into();

        section.insert(&addon.name(), toml_edit::value(next));

        Ok(())
    }

    pub fn remove(&mut self, key: &super::Key, name: &str) -> anyhow::Result<()> {
        let section = self.get_section_mut(key)?;

        section.remove(name);

        if section.is_empty() {
            match &key.target {
                None => {
                    // Don't remove the production 'addons' table.
                    if key.dev {
                        self.0.remove(key.last());
                    }
                }
                Some(target_key) => {
                    if let Some(targets) = self
                        .0
                        .get_mut(MANIFEST_SECTION_KEY_TARGET)
                        .and_then(|v| v.as_table_like_mut())
                    {
                        if let Some(target) = targets
                            .get_mut(target_key)
                            .and_then(|v| v.as_table_like_mut())
                        {
                            target.remove(key.last());

                            if target.is_empty() {
                                targets.remove(target_key);
                            }
                        }

                        if targets.is_empty() {
                            self.0.remove(MANIFEST_SECTION_KEY_TARGET);
                        }
                    }
                }
            };
        }

        Ok(())
    }

    fn get_section_mut(&mut self, key: &super::Key) -> anyhow::Result<&mut dyn TableLike> {
        match &key.target {
            None => {
                if !self.0.contains_table(key.last()) {
                    self.0.insert(key.last(), toml_edit::table());
                }

                self.0
                    .get_mut(key.last())
                    .and_then(|v| v.as_table_like_mut())
                    .ok_or(anyhow!("missing table"))
            }
            Some(t) => {
                if !self.0.contains_table(MANIFEST_SECTION_KEY_TARGET) {
                    self.0
                        .insert(MANIFEST_SECTION_KEY_TARGET, toml_edit::table());
                }

                let targets = self
                    .0
                    .get_mut(MANIFEST_SECTION_KEY_TARGET)
                    .and_then(|v| v.as_table_like_mut())
                    .ok_or(anyhow!("missing table"))?;

                if !targets.is_dotted() {
                    targets.set_dotted(true);
                }

                if !targets.contains_key(t) {
                    targets.insert(t, toml_edit::table());
                }

                let target = targets
                    .get_mut(t)
                    .and_then(|v| v.as_table_like_mut())
                    .ok_or(anyhow!("missing table"))?;

                if !target.is_dotted() {
                    target.set_dotted(true);
                }

                if !target.contains_key(key.last()) {
                    target.insert(key.last(), toml_edit::table());
                }

                let section = target
                    .get_mut(key.last())
                    .and_then(|v| v.as_table_like_mut())
                    .ok_or(anyhow!("missing table"))?;

                if section.is_dotted() {
                    section.set_dotted(false);
                }

                Ok(section)
            }
        }
    }
}

impl Default for Manifest {
    fn default() -> Self {
        let mut doc = Document::new();

        doc.insert(MANIFEST_SECTION_KEY_ADDONS, toml_edit::table());

        Manifest(doc)
    }
}

pub fn init_from(path: &Path) -> anyhow::Result<Manifest> {
    if !path.exists() {
        return Ok(Manifest::default());
    }

    if !path.is_file() {
        return Err(anyhow!("expected path to a file"));
    }

    let contents = std::fs::read_to_string(path)?;
    let mut doc = contents.parse::<Document>()?;

    if !doc.contains_key(MANIFEST_SECTION_KEY_ADDONS) {
        doc.insert(MANIFEST_SECTION_KEY_ADDONS, toml_edit::table());
    }

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
