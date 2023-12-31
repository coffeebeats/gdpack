use anyhow::anyhow;
use ini::Ini;
use std::path::Path;
use std::path::PathBuf;

/* -------------------------------------------------------------------------- */
/*                              Struct: Extension                             */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug)]
pub struct Extension {
    #[allow(dead_code)]
    config: Ini,
    path: PathBuf,
}

/* ----------------------------- Impl: Extension ---------------------------- */

impl Extension {
    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Extension> {
        if !path.as_ref().exists() {
            return Err(anyhow!("filepath not found"));
        }

        if !path
            .as_ref()
            .extension()
            .is_some_and(|s| s != "gdextension")
        {
            return Err(anyhow!("expected '*.gdextension' file"));
        }

        Ini::load_from_file(path.as_ref())
            .map(|c| Extension {
                config: c,
                path: path.as_ref().to_owned(),
            })
            .map_err(|e| anyhow!(e))
    }

    pub fn name(&self) -> Option<&str> {
        self.path
            .file_name()
            .and_then(|s| s.to_str())
            .and_then(|s| s.strip_suffix(".gdextension"))
    }

    pub fn source(&self) -> impl AsRef<Path> + '_ {
        self.path
            .parent()
            .expect("can't determine extension directory")
    }
}
