use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use toml_edit::de::ValueDeserializer;
use toml_edit::Item;
use typed_builder::TypedBuilder;

/* -------------------------------------------------------------------------- */
/*                           Struct: ScriptTemplates                          */
/* -------------------------------------------------------------------------- */

/// `ScriptTemplates` defines project configuration pertaining to the GDScript
/// templates used during development.
#[derive(Clone, Debug, Default, Eq, Deserialize, PartialEq, Serialize, TypedBuilder)]
pub struct ScriptTemplates {
    #[builder(default)]
    #[serde(default, rename = "include_script_templates")]
    pub include: Vec<PathBuf>,
    #[builder(default)]
    #[serde(default, rename = "export_script_templates")]
    pub export: Vec<PathBuf>,
}

/* -------------------------- Impl: TryFrom<&Item> -------------------------- */

impl TryFrom<&Item> for ScriptTemplates {
    type Error = toml_edit::de::Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        value
            .to_string()
            .trim()
            .parse::<ValueDeserializer>()
            .and_then(ScriptTemplates::deserialize)
    }
}
