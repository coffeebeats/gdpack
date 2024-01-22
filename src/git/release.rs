use serde::Deserialize;
use serde::Serialize;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;
use tempfile::tempdir;
use typed_builder::TypedBuilder;

use super::Database;
use super::Error;
use super::Remote;

/* -------------------------------------------------------------------------- */
/*                            Struct: GitHubRelease                           */
/* -------------------------------------------------------------------------- */

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, TypedBuilder,
)]
pub struct GitHubRelease {
    #[serde(rename = "git")]
    pub repo: Remote,
    #[serde(rename = "release")]
    pub tag: String,
    pub asset: String,
}

/* --------------------------- Impl: GitHubRelease -------------------------- */

impl GitHubRelease {
    /* --------------------------- Methods: Public -------------------------- */

    /// Returns a path to the release-specific directory for the specified
    /// [super::Remote] in the `gdpack` store.
    pub fn get_path(&self) -> Result<PathBuf, Error> {
        let mut path = super::get_store_path()?;
        path.extend(&["asset", &Database::id(&self.repo)?]);

        path.push(&self.tag);

        let mut asset = self.get_asset_name();

        let filename = PathBuf::from(&self.asset);
        if let Some(ext) = filename.extension().and_then(OsStr::to_str) {
            asset = asset
                .strip_suffix(&format!(".{}", ext))
                .unwrap_or(&asset)
                .to_owned()
        }

        path.push(asset);

        Ok(path)
    }

    pub fn download(&self) -> Result<(), Error> {
        // TODO: Save compressed archives instead of extracted contents so that
        // less space is consumed and checksum validation is easier.
        // If the asset exists already, skip the download.
        let path_store = self.get_path()?;
        if path_store.as_path().is_dir()
            && std::fs::read_dir(path_store).map_err(Error::Io)?.count() > 0
        {
            return Ok(());
        }

        let base = self.repo.assets()?;
        let asset_url = base
            .join(&format!("{}/{}", self.tag, self.get_asset_name()))
            .map_err(Error::Url)?;

        let tmp = tempdir().map_err(Error::Io)?;

        let path = tmp.path().join(self.get_asset_name());

        let mut file = File::create(path.as_path()).map_err(Error::Io)?;

        let res = reqwest::blocking::get(asset_url.clone()).map_err(Error::Request)?;

        let status = res.status();
        if status.is_client_error() || status.is_server_error() {
            return Err(Error::Response(status));
        }

        let mut content = Cursor::new(res.bytes().map_err(Error::Request)?);

        std::io::copy(&mut content, &mut file).map_err(Error::Io)?;

        let target = self.get_path()?;

        let mut archive =
            zip::ZipArchive::new(File::open(&path).map_err(Error::Io)?).map_err(Error::Zip)?;

        // See https://github.com/zip-rs/zip/blob/3e88fe66c941d411cff5cf49778ba08c2ed93801/examples/extract.rs
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(Error::Zip)?;

            let filepath = match file.enclosed_name() {
                Some(n) => n.to_owned(),
                None => continue, // Skip insecure filepaths.
            };

            let dst = target.join(filepath.as_path());

            if file.is_dir() {
                std::fs::create_dir_all(&dst).map_err(Error::Io)?;
            } else {
                if let Some(p) = dst.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p).map_err(Error::Io)?;
                    }
                }

                std::io::copy(
                    &mut file,
                    &mut std::fs::File::create(&dst).map_err(Error::Io)?,
                )
                .map_err(Error::Io)?;
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    std::fs::set_permissions(&dst, std::fs::Permissions::from_mode(mode))
                        .map_err(Error::Io)?;
                }
            }
        }

        Ok(())
    }

    /* -------------------------- Methods: Private -------------------------- */

    /// `get_asset_name` returns the name of the release asset after hydrating
    /// template variables.
    fn get_asset_name(&self) -> String {
        self.asset
            .replace("{tag}", &self.tag)
            .replace("{name}", self.repo.name().as_deref().unwrap_or("{name}"))
            .replace("{repo}", self.repo.name().as_deref().unwrap_or("{repo}"))
            .replace("{release}", &self.tag)
    }
}
