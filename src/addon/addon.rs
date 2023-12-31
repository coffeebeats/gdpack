use anyhow::anyhow;
use std::path::Path;
use std::path::PathBuf;
use typed_builder::TypedBuilder;
use walkdir::WalkDir;

use super::Extension;
use super::Plugin;

/* -------------------------------------------------------------------------- */
/*                                Struct: Addon                               */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug, TypedBuilder)]
pub struct Addon {
    #[builder(default)]
    extension: Option<Extension>,
    path: PathBuf,
    #[builder(default)]
    plugin: Option<Plugin>,
}

/* ------------------------------- Impl: Addon ------------------------------ */

impl Addon {
    pub fn new(path: impl AsRef<Path>, name: Option<&str>) -> anyhow::Result<Addon> {
        let path = path.as_ref().to_owned();
        if !path.exists() || !path.is_dir() {
            return Err(anyhow!(
                "invalid path to addon; expected a directory: {}",
                path.to_str().unwrap_or("''")
            ));
        }

        // First check if the addon has a plugin defined at the root.
        let path_plugin = path.join("plugin.cfg");
        if path_plugin.exists() {
            let plugin = Addon::try_get_plugin(path_plugin.clone(), name)?;
            if plugin.is_some() {
                return Ok(Addon::builder().path(path_plugin).plugin(plugin).build());
            }
        }

        // Next check if the addon has a gdextension defined at the root.
        for entry in WalkDir::new(path.clone())
            .follow_root_links(true)
            .follow_links(false)
            .contents_first(true)
            .max_depth(1)
            .min_depth(1)
        {
            let entry = entry?;
            let path = entry.path();

            if !path.exists()
                || !path.is_file()
                || !path.extension().is_some_and(|s| s == "gdextension")
            {
                continue;
            }

            let extension = Addon::try_get_extension(path, name)?;
            if extension.is_some() {
                return Ok(Addon::builder()
                    .path(path.to_owned())
                    .extension(extension)
                    .build());
            }
        }

        // Next, check each directory in 'addons'.
        for entry in WalkDir::new(path.join("addons"))
            .follow_root_links(true)
            .follow_links(true)
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
                    .is_some_and(|n| n.starts_with('.'))
            {
                continue;
            }

            let path = entry.path();

            // First check if the addon has a plugin defined.
            let path_plugin = path.join("plugin.cfg");
            if path_plugin.exists() {
                let plugin = Addon::try_get_plugin(path_plugin.as_path(), name)?;
                if plugin.is_some() {
                    return Ok(Addon::builder().path(path_plugin).plugin(plugin).build());
                }
            }

            // Next check if the addon has a gdextension defined.
            for entry in WalkDir::new(path)
                .follow_root_links(true)
                .follow_links(false)
                .contents_first(true)
                .max_depth(1)
                .min_depth(1)
            {
                let entry = entry?;
                let path = entry.path();

                if !path.exists()
                    || !path.is_file()
                    || !path.extension().is_some_and(|s| s == "gdextension")
                {
                    continue;
                }

                let extension = Addon::try_get_extension(path, name)?;
                if extension.is_some() {
                    return Ok(Addon::builder()
                        .path(path.to_owned())
                        .extension(extension)
                        .build());
                }
            }
        }

        Ok(Addon::builder().path(path).build())
    }

    pub fn install_to(&self, out: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = out.as_ref();
        if !path.exists() || !path.is_dir() || !path.join("project.godot").exists() {
            return Err(anyhow!("expected a project directory: {:?}", path));
        }

        let subfolder = if let Some(plugin) = self.plugin.as_ref() {
            plugin.subfolder().or(plugin.name())
        } else if let Some(extension) = self.extension.as_ref() {
            extension.name()
        } else {
            // TODO: Determine whether this is correct.
            self.path.file_name().and_then(|s| s.to_str())
        };

        if subfolder.is_none() {
            return Err(anyhow!("cannot determine addon path"));
        }

        let mut dest = PathBuf::from(path);
        dest.extend(&["addons", &subfolder.unwrap()]);

        let source = if let Some(plugin) = self.plugin.as_ref() {
            plugin.source().as_ref().to_owned()
        } else if let Some(extension) = self.extension.as_ref() {
            extension.source().as_ref().to_owned()
        } else {
            self.path.to_owned()
        };

        println!("source: {}", &source.to_str().unwrap_or(""));

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
}

impl Addon {
    fn try_get_extension(
        path: impl AsRef<Path>,
        name: Option<&str>,
    ) -> anyhow::Result<Option<Extension>> {
        let plugin = Extension::from_file(path)?;
        if name.is_none()
            || name
                .as_ref()
                .is_some_and(|want| plugin.name().as_ref().is_some_and(|got| got == want))
        {
            return Ok(Some(plugin));
        }

        Ok(None)
    }

    fn try_get_plugin(
        path: impl AsRef<Path>,
        name: Option<&str>,
    ) -> anyhow::Result<Option<Plugin>> {
        let plugin = Plugin::from_file(path)?;
        if name.is_none()
            || name
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
