use anyhow::anyhow;
use ini::Ini;
use semver::Version;

use super::Configuration;
use super::Parsable;
use super::ParsableError;

/* -------------------------------------------------------------------------- */
/*                               Struct: Plugin                               */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug, Default)]
pub struct Plugin(Ini);

/* ------------------------------ Impl: Plugin ------------------------------ */

impl Plugin {
    #[allow(dead_code)]
    pub fn name(&self) -> Option<&str> {
        self.0.section(Some("plugin")).and_then(|s| s.get("name"))
    }

    #[allow(dead_code)]
    pub fn subfolder(&self) -> Option<&str> {
        self.0
            .section(Some("plugin"))
            .and_then(|s| s.get("subfolder"))
    }

    #[allow(dead_code)]
    pub fn version(&self) -> Option<Version> {
        self.0
            .section(Some("plugin"))
            .and_then(|s| s.get("version"))
            .and_then(|s| Version::parse(s).ok())
    }
}

/* --------------------------- Impl: Configuration -------------------------- */

impl Configuration for Plugin {
    fn matches(path: impl AsRef<std::path::Path>) -> bool {
        path.as_ref().file_name().is_some_and(|s| s == "plugin.cfg")
    }
}

/* ----------------------------- Impl: Parsable ----------------------------- */

impl Parsable for Plugin {
    fn parse(contents: &str) -> Result<Self, ParsableError> {
        Ini::load_from_str(contents)
            .map(Plugin)
            .map_err(|e| ParsableError::Parse(anyhow!(e)))
    }
}
