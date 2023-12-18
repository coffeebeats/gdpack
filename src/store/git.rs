use anyhow::anyhow;
use std::path::PathBuf;
use std::process::Command;
use url::Url;

use crate::addon;

/* -------------------------------------------------------------------------- */
/*                              Struct: GitRemote                             */
/* -------------------------------------------------------------------------- */

#[derive(Clone)]
pub struct GitRemote(Url);

/* ----------------------------- Impl: GitRemote ---------------------------- */

impl GitRemote {
    pub fn fetch(&self) -> anyhow::Result<GitRepo> {
        let mut path = super::get_path()?;
        path.extend(&["git", "repo", &self.name()?]);

        if (&path).exists() {
            return Ok(GitRepo {
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

        Ok(GitRepo {
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

impl From<&crate::addon::git::Spec> for GitRemote {
    fn from(value: &crate::addon::git::Spec) -> Self {
        Self(value.repo.clone())
    }
}

/* -------------------------------------------------------------------------- */
/*                               Struct: GitRepo                              */
/* -------------------------------------------------------------------------- */

#[allow(dead_code)]
pub struct GitRepo {
    repo: git2::Repository,
    remote: GitRemote,
    path: PathBuf,
}

/* ------------------------------ Impl: GitRepo ----------------------------- */

impl GitRepo {
    pub fn checkout(&self, commit: addon::git::Commit) -> anyhow::Result<GitCheckout> {
        let obj = self.repo.revparse_single(&commit.rev())?;

        let short_id = obj
            .short_id()
            .map(|id| id.as_str().map(|s| s.to_owned()))?
            .ok_or(anyhow!("couldn't parse revision"))?;

        let mut path = super::get_path()?;
        path.extend(&["git", "checkout", &self.remote.name()?, &short_id]);

        if (&path).exists() {
            let repo = git2::Repository::open(&path)?;

            return Ok(GitCheckout {
                repo,
                path,
                revision: commit,
            });
        }

        std::fs::create_dir_all(&path)?;

        let output = Command::new("git")
            .current_dir(&path)
            .arg("clone")
            .arg(&self.path)
            .arg("--single-branch")
            .arg("--no-tags")
            .arg(path.to_str().ok_or(anyhow!("missing path"))?)
            .output()?;

        if !output.status.success() {
            if let Ok(s) = std::str::from_utf8(&output.stderr) {
                if !s.is_empty() {
                    println!("{}", s);
                }
            }
        }

        let output = Command::new("git")
            .current_dir(&path)
            .arg("checkout")
            .arg(short_id)
            .output()?;

        if !output.status.success() {
            if let Ok(s) = std::str::from_utf8(&output.stderr) {
                if !s.is_empty() {
                    println!("{}", s);
                }
            }
        }

        Ok(GitCheckout {
            repo: git2::Repository::open(&path)?,
            path,
            revision: commit,
        })
    }
}

/* -------------------------------------------------------------------------- */
/*                             Struct: GitCheckout                            */
/* -------------------------------------------------------------------------- */

#[allow(dead_code)]
pub struct GitCheckout {
    pub(super) repo: git2::Repository,
    pub(super) path: PathBuf,
    pub(super) revision: addon::git::Commit,
}
