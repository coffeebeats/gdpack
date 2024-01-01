use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;
use tempfile::tempdir;
use typed_builder::TypedBuilder;

use super::Database;
use super::Remote;

/* -------------------------------------------------------------------------- */
/*                            Struct: GitHubRelease                           */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct GitHubRelease {
    pub repo: Remote,
    pub tag: String,
    pub asset: String,
}

/* --------------------------- Impl: GitHubRelease -------------------------- */

impl GitHubRelease {
    /// Returns a path to the release-specific directory for the specified
    /// [super::Remote] in the `gdpack` store.
    pub fn get_path(&self) -> anyhow::Result<PathBuf> {
        let mut path = super::get_store_path()?;
        path.extend(&["asset", &Database::id(&self.repo)?]);

        path.push(&self.tag);

        let filename = PathBuf::from(&self.asset);
        let extension = filename
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("");

        path.push(
            self.asset
                .strip_suffix(&format!(".{}", extension))
                .unwrap_or(&self.asset),
        );

        Ok(path)
    }

    pub fn download(&self) -> anyhow::Result<()> {
        let base = self.repo.assets()?;
        let asset_url = base.join(&format!("{}/{}", self.tag, self.asset))?;

        let tmp = tempdir()?;

        let path = tmp.path().join(&self.asset);

        let mut file = File::create(path.as_path())?;

        let res = reqwest::blocking::get(asset_url)?;

        let mut content = Cursor::new(res.bytes()?);

        std::io::copy(&mut content, &mut file)?;

        let target = self.get_path()?;

        let mut archive = zip::ZipArchive::new(File::open(path)?)?;

        // See https://github.com/zip-rs/zip/blob/3e88fe66c941d411cff5cf49778ba08c2ed93801/examples/extract.rs
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();

            let mut outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            outpath = target.join(outpath);

            if file.is_dir() {
                std::fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p).unwrap();
                    }
                }

                let mut outfile = std::fs::File::create(&outpath).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))
                        .unwrap();
                }
            }
        }

        Ok(())
    }
}
