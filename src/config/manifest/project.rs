use toml_edit::DocumentMut;
use typed_builder::TypedBuilder;

use crate::core::ExportFiles;
use crate::core::ScriptTemplates;

pub(super) const MANIFEST_SECTION_PROJECT: &str = "project";
pub(super) const MANIFEST_SECTION_PROJECT_EXPORT_FILES: &str = "export_files";
pub(super) const MANIFEST_SECTION_PROJECT_SCRIPT_TEMPLATES: &str = "script_templates";

/* -------------------------------------------------------------------------- */
/*                               Struct: Project                              */
/* -------------------------------------------------------------------------- */

/// [`Project`] is used to immutably view the project configuration within the
/// provided [`toml_edit::DocumentMut`].
#[derive(Debug, TypedBuilder)]
pub struct Project<'a> {
    document: &'a DocumentMut,
}

/* ------------------------------ Impl: Project ----------------------------- */

impl<'a> Project<'a> {
    /// `get_export_files` returns the exported file-related configuration
    /// within the `project` table of the [`super::Manifest`].
    pub fn get_export_files(&self) -> Option<ExportFiles> {
        self.document
            .as_table()
            .get(MANIFEST_SECTION_PROJECT)
            .and_then(|v| v.as_table())
            .and_then(|t| t.get(MANIFEST_SECTION_PROJECT_EXPORT_FILES))
            .and_then(|v| v.as_table())
            .map(|t| t.clone().into_inline_table())
            .and_then(|t| ExportFiles::try_from(&toml_edit::value(t)).ok())
    }

    /// `get_script_templates` returns the script template-related configuration
    /// within the `project` table of the [`super::Manifest`].
    pub fn get_script_templates(&self) -> Option<ScriptTemplates> {
        self.document
            .as_table()
            .get(MANIFEST_SECTION_PROJECT)
            .and_then(|v| v.as_table())
            .and_then(|t| t.get(MANIFEST_SECTION_PROJECT_SCRIPT_TEMPLATES))
            .and_then(|v| v.as_table())
            .map(|t| t.clone().into_inline_table())
            .and_then(|t| ScriptTemplates::try_from(&toml_edit::value(t)).ok())
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Mod: Tests                                 */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use serde::de::Deserialize;
    use std::path::PathBuf;
    use toml_edit::de::ValueDeserializer;

    use crate::core::ScriptTemplates;

    /* ------------------------ Test: Deserialization ----------------------- */

    macro_rules! test_de_templates {
        ($name:ident, $input:expr, $want:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    $input
                        .parse::<ValueDeserializer>()
                        .and_then(ScriptTemplates::deserialize),
                    Ok($want),
                );
            }
        };
    }

    test_de_templates!(
        test_de_templates_include_succeeds,
        r#"{ include = ["./a.gd", "../a.gd", "a.gd", "/a.gd"] }"#,
        ScriptTemplates::builder()
            .include(
                vec!["./a.gd", "../a.gd", "a.gd", "/a.gd"]
                    .into_iter()
                    .map(PathBuf::from)
                    .collect::<Vec<_>>()
            )
            .build()
    );

    test_de_templates!(
        test_de_templates_export_from_inline_table_succeeds,
        r#"{ export = ["./a.gd", "../a.gd", "a.gd", "/a.gd"] }"#,
        ScriptTemplates::builder()
            .export(
                vec!["./a.gd", "../a.gd", "a.gd", "/a.gd"]
                    .into_iter()
                    .map(PathBuf::from)
                    .collect::<Vec<_>>()
            )
            .build()
    );

    test_de_templates!(
        test_de_templates_include_and_export_succeeds,
        r#"{ include = ["./a.gd", "../a.gd", "a.gd", "/a.gd"], export = ["./a.gd", "../a.gd", "a.gd", "/a.gd"] }"#,
        ScriptTemplates::builder()
            .include(
                vec!["./a.gd", "../a.gd", "a.gd", "/a.gd"]
                    .into_iter()
                    .map(PathBuf::from)
                    .collect::<Vec<_>>()
            )
            .export(
                vec!["./a.gd", "../a.gd", "a.gd", "/a.gd"]
                    .into_iter()
                    .map(PathBuf::from)
                    .collect::<Vec<_>>()
            )
            .build()
    );
}
