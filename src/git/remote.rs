use anyhow::anyhow;
use std::path::PathBuf;
use std::process::Command;
use url::Url;

use super::Repository;

/* -------------------------------------------------------------------------- */
/*                               Struct: Remote                               */
/* -------------------------------------------------------------------------- */

#[derive(Clone)]
pub struct Remote(pub(super) Url);

/* ------------------------------ Impl: Remote ------------------------------ */

impl Remote {
    pub fn fetch_to(&self, path: PathBuf) -> anyhow::Result<Repository> {
        if (&path).exists() {
            return Ok(Repository {
                repo: git2::Repository::open(&path)?,
                remote: self.clone(),
                path: path,
            });
        }

        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        let output = Command::new("git")
            .current_dir(&path)
            .arg("clone")
            .arg(self.0.as_str())
            .arg("--tags")
            .arg("--no-checkout")
            .args(&["--bare", path.to_str().ok_or(anyhow!("missing path"))?])
            .output()?;

        if !output.status.success() {
            if let Ok(s) = std::str::from_utf8(&output.stderr) {
                if !s.is_empty() {
                    println!("{}", s);
                }
            }
        }

        Ok(super::Repository {
            repo: git2::Repository::open(&path)?,
            remote: self.clone(),
            path: path,
        })
    }

    pub fn name(&self) -> anyhow::Result<String> {
        let host = self
            .0
            .host()
            .ok_or(anyhow!("missing host"))?
            .to_string()
            .to_lowercase()
            .replace(".", "_");

        let name = self
            .0
            .path()
            .to_lowercase()
            .trim_matches('/')
            .replace("/", "_");

        name.strip_suffix(".git")
            .map(|name| format!("{}_{}", host, name))
            .ok_or(anyhow!("invalid path segments"))
    }
}

/* ------------------- Impl: From<crate::addon::git::Spec> ------------------ */

impl From<&crate::addon::git::Spec> for Remote {
    fn from(value: &crate::addon::git::Spec) -> Self {
        Self(value.repo.clone())
    }
}
