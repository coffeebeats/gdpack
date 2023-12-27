use anyhow::anyhow;
use ini::Ini;
use semver::Version;
use std::path::Path;
use std::path::PathBuf;

/* -------------------------------------------------------------------------- */
/*                               Struct: Plugin                               */
/* -------------------------------------------------------------------------- */

pub struct Plugin {
    config: Ini,
    path: PathBuf,
}

/* ------------------------------ Impl: Plugin ------------------------------ */

impl Plugin {
    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Plugin> {
        if !path.as_ref().exists() {
            return Err(anyhow!("filepath not found"));
        }

        if !path.as_ref().ends_with("plugin.cfg") {
            return Err(anyhow!("expected 'plugin.cfg' file"));
        }

        Ini::load_from_file(path.as_ref())
            .map(|c| Plugin {
                config: c,
                path: path.as_ref().to_owned(),
            })
            .map_err(|e| anyhow!(e))
    }

    pub fn name(&self) -> Option<&str> {
        self.config
            .section(Some("plugin"))
            .and_then(|s| s.get("name"))
    }

    pub fn source(&self) -> impl AsRef<Path> + '_ {
        self.path
            .parent()
            .expect("can't determine plugin directory")
    }

    pub fn subfolder(&self) -> Option<&str> {
        self.config
            .section(Some("plugin"))
            .and_then(|s| s.get("subfolder"))
    }

    #[allow(dead_code)]
    pub fn version(&self) -> Option<Version> {
        self.config
            .section(Some("plugin"))
            .and_then(|s| s.get("version"))
            .and_then(|s| Version::parse(s).ok())
    }
}
