use std::path::PathBuf;

use super::git;

/* -------------------------------------------------------------------------- */
/*                                Struct: Addon                               */
/* -------------------------------------------------------------------------- */

pub struct Addon {
    pub replace: Option<String>,
    pub spec: Spec,
}

/* ------------------------------- Impl: Addon ------------------------------ */

impl Addon {
    pub fn new(spec: Spec, replace: Option<String>) -> Addon {
        Addon {
            spec: spec,
            replace: replace,
        }
    }

    pub fn replace(other: &str, spec: Spec) -> Addon {
        Addon {
            spec: spec,
            replace: Some(other.to_owned()),
        }
    }

    // TODO: Remove this.
    pub fn name(&self) -> String {
        match &self.spec {
            Spec::Path(p) => p
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .expect("missing filename")
                .to_owned(),
            Spec::Git(g) => g
                .repo
                .path_segments()
                .expect("missing path segments")
                .last()
                .expect("missing path segment")
                .to_owned(),
        }
    }
}

impl From<Spec> for Addon {
    fn from(value: Spec) -> Self {
        return Addon::new(value, None);
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Enum: Spec                                 */
/* -------------------------------------------------------------------------- */

#[derive(Debug, serde::Deserialize, PartialEq, Eq, serde::Serialize)]
#[serde(untagged)]
pub enum Spec {
    Path(PathBuf),
    Git(git::Spec),
}
