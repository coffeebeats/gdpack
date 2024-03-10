use serde::Deserialize;
use serde::Serialize;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use typed_builder::TypedBuilder;

/* -------------------------------------------------------------------------- */
/*                                Struct: Hook                                */
/* -------------------------------------------------------------------------- */

/// Hook contains pre- and post-install scripts.
#[derive(
    Clone,
    Debug,
    Default,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    TypedBuilder,
)]
pub struct Hook {
    #[serde(rename = "preinstall")]
    pub pre: Option<String>,
    #[serde(rename = "postinstall")]
    pub post: Option<String>,
}

/* ------------------------------- Impl: Hook ------------------------------- */

impl Hook {
    /* --------------------------- Methods: Public -------------------------- */

    /// `run_pre` runs the pre-install script, if one is defined.
    pub fn run_pre(&self, path_working_dir: impl AsRef<Path>) -> std::io::Result<()> {
        if let Some(script) = self.pre.as_ref() {
            let status = Hook::run(script, path_working_dir)?;
            if !status.success() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("command failed with status {:?}", status.code()),
                ));
            }
        }

        Ok(())
    }

    /// `run_post` runs the post-install script, if one is defined.
    pub fn run_post(&self, path_working_dir: impl AsRef<Path>) -> std::io::Result<()> {
        if let Some(script) = self.post.as_ref() {
            let status = Hook::run(script, path_working_dir)?;
            if !status.success() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("command failed with status {:?}", status.code()),
                ));
            }
        }

        Ok(())
    }

    /* -------------------------- Methods: Private -------------------------- */

    fn run(
        script: &str,
        path_working_dir: impl AsRef<Path>,
    ) -> std::io::Result<std::process::ExitStatus> {
        let output = Command::new("sh")
            .current_dir(path_working_dir)
            .arg("-c")
            .arg(script)
            .output()?;

        std::io::stdout().write_all(&output.stdout).unwrap();
        std::io::stderr().write_all(&output.stderr).unwrap();

        Ok(output.status)
    }
}
