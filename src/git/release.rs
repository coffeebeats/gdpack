use anyhow::anyhow;
use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;
use tempfile::tempdir;

use super::Database;
use super::Reference;
use super::Remote;
use super::Source;

/* -------------------------------------------------------------------------- */
/*                            Struct: GitHubRelease                           */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubRelease {
    repo: Remote,
    tag: String,
}

/* --------------------------- Impl: GitHubRelease -------------------------- */

impl GitHubRelease {
    /// Returns a path to the release-specific directory for the specified
    /// [super::Remote] in the `gdpack` store.
    pub fn get_path(remote: &Remote, source: &Source) -> anyhow::Result<PathBuf> {
        let mut path = super::get_store_path()?;
        path.extend(&["asset", &Database::id(remote)?]);

        let (tag, _) = match &source.reference {
            Reference::Release(r, a) => (r, a),
            _ => return Err(anyhow!("invalid 'Reference'; expected a release!")),
        };

        path.push(tag);

        Ok(path)
    }

    pub fn download_asset(&self, asset: String) -> anyhow::Result<()> {
        println!("Installing from release: {} {}", self.tag, asset);
        let base = self.repo.assets()?;
        println!("Base URL: {}", base.as_str());
        let asset_url = base.join(&format!("{}/{}", self.tag, asset))?;
        println!("Asset URL: {}", asset_url.as_str());

        let path = GitHubRelease::get_path(&self.repo, self.source);

        let path = tmp.path().join(asset);
        println!("Path: {}", path.to_str().unwrap());

        let mut file = File::create(path.as_path())?;
        println!("Downloading to: {}", &path.to_str().unwrap_or("''"));

        let res = reqwest::blocking::get(asset_url)?;
        println!("Downloaded: {}", res.status());

        let mut content = Cursor::new(res.bytes()?);

        std::io::copy(&mut content, &mut file)?;

        println!("downloaded: {}", path.to_str().unwrap_or(""));

        todo!()
    }
}
