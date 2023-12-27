use anyhow::anyhow;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::git;

use super::Plugin;
use super::Spec;

/* -------------------------------------------------------------------------- */
/*                                Struct: Addon                               */
/* -------------------------------------------------------------------------- */

#[derive(typed_builder::TypedBuilder)]
pub struct Addon {
    /// Addon name (used in multi-addon repositories).
    #[builder(default)]
    pub name: Option<String>,
    /// Name of an addon to replace during installs.
    #[builder(default)]
    pub replace: Option<String>,
    /// The source of the addon repository/directory.
    pub spec: Spec,
}

/* ------------------------------- Impl: Addon ------------------------------ */

impl Addon {
    pub fn install_to(&self, out: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = out.as_ref();
        if !path.exists() || !path.is_dir() || !path.join("project.godot").exists() {
            return Err(anyhow!("expected a project directory: {:?}", path));
        }

        let plugin = self.plugin()?;

        let subfolder = match plugin.as_ref() {
            Some(p) => p.subfolder().or(p.name()).map(&str::to_owned),
            None => self.package(),
        };

        if subfolder.is_none() {
            return Err(anyhow!("cannot determine addon path"));
        }

        let mut dest = PathBuf::from(path);
        dest.extend(&["addons", &subfolder.unwrap()]);

        let mut source = match &self.spec {
            crate::addon::Spec::Path(p) => p.clone(),
            crate::addon::Spec::Git(g) => git::checkout(g)?.path.clone(),
        };

        if let Some(plugin) = plugin.as_ref() {
            let plugin_source = plugin.source();
            let subdirectory = plugin_source.as_ref().strip_prefix(&source)?;

            source.push(subdirectory);
        }

        if dest.as_path().exists() {
            std::fs::remove_dir_all(dest.as_path())?;
        }

        // Recursively copy contents from source into destination.
        copy_recursively(source, dest, |s, d| {
            std::fs::hard_link(s, d).map_err(|e| anyhow!(e))
        })
    }

    pub fn dependencies(&self) -> anyhow::Result<Vec<Addon>> {
        todo!()
    }

    pub fn package(&self) -> Option<String> {
        match &self.spec {
            crate::addon::Spec::Git(g) => g.repo.owner(),
            crate::addon::Spec::Path(p) => p
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .map(&str::to_owned),
        }
    }
}

impl Addon {
    fn plugin(&self) -> anyhow::Result<Option<Plugin>> {
        // Determine the path to the addon directory.
        let path = match &self.spec {
            Spec::Path(p) => p.to_owned(),
            Spec::Git(g) => {
                let checkout = git::checkout(g)?;
                checkout.path
            }
        };

        if !path.exists() {
            return Err(anyhow!("addon not found"));
        }

        // First check if the addon has a plugin defined at the root.
        let root = path.join("plugin.cfg");
        if root.exists() {
            let plugin = self.try_get_plugin(root)?;
            if plugin.is_some() {
                return Ok(plugin);
            }
        }

        // Next, check each directory in 'addons'.
        for entry in WalkDir::new(&path.join("addons"))
            .follow_root_links(true)
            .follow_links(true)
            .same_file_system(true)
            .contents_first(false)
            .max_depth(1)
            .min_depth(1)
        {
            let entry = entry?;

            // Skip hidden directories.
            if entry.path().is_dir()
                && entry
                    .file_name()
                    .to_str()
                    .is_some_and(|n| n.starts_with("."))
            {
                continue;
            }

            let path = entry.path().join("plugin.cfg");
            if path.exists() {
                let plugin = self.try_get_plugin(path)?;
                if plugin.is_some() {
                    return Ok(plugin);
                }
            }
        }

        if let Some(name) = self.name.as_ref() {
            return Err(anyhow!("plugin not found: {}", name));
        }

        Ok(None)
    }

    fn try_get_plugin(&self, path: impl AsRef<Path>) -> anyhow::Result<Option<Plugin>> {
        let plugin = Plugin::from_file(path)?;
        if self.name.is_none()
            || self
                .name
                .as_ref()
                .is_some_and(|want| plugin.name().as_ref().is_some_and(|got| got == want))
        {
            return Ok(Some(plugin));
        }

        Ok(None)
    }
}

/// Hard links files recursively from source to destination.
/// See https://nick.groenen.me/notes/recursively-copy-files-in-rust/.
fn copy_recursively(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    copy_fn: fn(&Path, &Path) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(&dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            copy_recursively(entry.path(), dst.as_ref().join(entry.file_name()), copy_fn)?;
        } else {
            copy_fn(
                entry.path().as_ref(),
                dst.as_ref().join(entry.file_name()).as_ref(),
            )?;
        }
    }

    Ok(())
}
