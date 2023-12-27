use anyhow::anyhow;
use git2::Oid;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use super::Checkout;
use super::Reference;
use super::Remote;
use super::Source;

/* -------------------------------------------------------------------------- */
/*                              Struct: Database                              */
/* -------------------------------------------------------------------------- */

pub struct Database(Remote);

/* ----------------------------- Impl: Database ----------------------------- */

impl Database {
    /* ----------------------------- Methods: Public ---------------------------- */

    pub fn checkout(&self, reference: &Reference) -> anyhow::Result<Checkout> {
        let path_db = Database::get_path(&self.0)?;

        let repo = git2::Repository::open(&path_db)?;

        let obj = repo.revparse_single(&reference.to_string())?;

        let source = &Source::builder()
            .reference(Reference::Rev(obj.id().to_string()))
            .repo(self.0.clone())
            .build();

        let path_checkout = Checkout::get_path(&repo, &source)?;
        if !path_checkout.exists() {
            if let Err(e) = self.fetch_latest(&reference) {
                println!("failed to fetch latest: {}; skipping...", e);
            }

            println!(
                "installing revision {} for dependency: {}",
                path_checkout
                    .file_name()
                    .and_then(|s| s.to_str())
                    .expect("missing short id"),
                self.0.name().expect("missing remote name")
            );

            let repo =
                git2::Repository::clone(path_db.to_str().expect("invalid path"), &path_checkout)?;

            repo.checkout_tree(
                &repo
                    .find_commit(Oid::from_str(&obj.id().to_string())?)?
                    .into_object(),
                Some(git2::build::CheckoutBuilder::new().force().refresh(true)),
            )?;

            repo.set_head_detached(obj.id()).unwrap();
        }

        let checkout = Checkout {
            path: path_checkout,
            reference: source.reference.clone(),
        };

        return Ok(checkout);
    }

    pub fn fetch_latest(&self, reference: &Reference) -> anyhow::Result<()> {
        println!(
            "fetching latest for dependency: {}",
            self.0.name().expect("missing remote name")
        );

        let path = Database::get_path(&self.0)?;

        let repo = git2::Repository::open(path)?;

        let mut remote = repo.remote_anonymous(&self.0.to_string())?;

        remote.fetch(
            &reference.refspecs(),
            Some(
                git2::FetchOptions::default()
                    .prune(git2::FetchPrune::On)
                    .update_fetchhead(true),
            ),
            None,
        )?;

        Ok(())
    }

    /* ---------------------------- Methods: Private ---------------------------- */

    pub(super) fn get_path(remote: &Remote) -> anyhow::Result<PathBuf> {
        let mut path = super::get_store_path()?;
        path.extend(&["git", "repo", &Database::id(remote)?]);

        Ok(path)
    }

    pub(super) fn id(remote: &Remote) -> anyhow::Result<String> {
        let host = remote
            .host()
            .map(|s| s.replace(".", "_"))
            .map(|s| s.to_lowercase())
            .ok_or(anyhow!("missing repository host: {}", remote))?;

        let owner = remote
            .owner()
            .map(|s| s.replace("/", "_"))
            .map(|s| s.to_lowercase())
            .ok_or(anyhow!("missing repository owner: {}", remote))?;

        let name = remote
            .name()
            .map(|s| s.to_lowercase())
            .ok_or(anyhow!("missing repository name: {}", remote))?;

        Ok(format!("{}_{}_{}", host, owner, name))
    }
}

/* ------------------------- Impl: TryFrom<&Source> ------------------------- */

impl TryFrom<&Source> for Database {
    type Error = anyhow::Error;

    fn try_from(value: &Source) -> Result<Self, Self::Error> {
        let db = Database(value.repo.clone());

        let path = Database::get_path(&value.repo)?;
        if !path.exists() {
            clone_bare(value, path.as_path())?;
        }

        return Ok(db);
    }
}

/* -------------------------- Function: clone_bare -------------------------- */

fn clone_bare(source: &Source, path: impl AsRef<Path>) -> anyhow::Result<()> {
    println!("downloading dependency: {}", source.repo.to_string());

    std::fs::create_dir_all(&path)?;

    let mut clone_cmd = Command::new("git");

    clone_cmd
        .current_dir(path.as_ref())
        .arg("clone")
        .arg(source.repo.to_string())
        .arg("--no-checkout");

    if let Reference::Tag(_) = source.reference {
        clone_cmd.arg("--tags");
    }

    let output = clone_cmd
        .args(&["--bare", path.as_ref().to_str().unwrap()])
        .output()?;

    if !output.status.success() {
        if let Ok(s) = std::str::from_utf8(&output.stderr) {
            if !s.is_empty() {
                println!("{}", s);
            }
        }
    }

    Ok(())
}
