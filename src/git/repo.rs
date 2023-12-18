use anyhow::anyhow;
use std::path::PathBuf;
use std::process::Command;

/* -------------------------------------------------------------------------- */
/*                             Struct: Repository                             */
/* -------------------------------------------------------------------------- */

#[allow(dead_code)]
pub struct Repository {
    pub(super) repo: git2::Repository,
    pub(super) remote: super::Remote,
    pub(super) path: PathBuf,
}

/* ---------------------------- Impl: Repository ---------------------------- */

impl Repository {
    pub fn checkout_to(&self, path: PathBuf, commit: super::Reference) -> anyhow::Result<Checkout> {
        let obj = self.repo.revparse_single(&commit.rev())?;

        let short_id = obj
            .short_id()
            .map(|id| id.as_str().map(|s| s.to_owned()))?
            .ok_or(anyhow!("couldn't parse revision"))?;

        if (&path).exists() {
            let repo = git2::Repository::open(&path)?;

            return Ok(Checkout {
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

        Ok(Checkout {
            repo: git2::Repository::open(&path)?,
            path,
            revision: commit,
        })
    }
}

/* -------------------------------------------------------------------------- */
/*                              Struct: Checkout                              */
/* -------------------------------------------------------------------------- */

pub struct Checkout {
    pub(super) repo: git2::Repository,
    pub(super) path: PathBuf,
    pub(super) revision: super::Reference,
}
