use anyhow::anyhow;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use typed_builder::TypedBuilder;

use crate::config::gdext::Extension;
use crate::config::manifest::Manifest;
use crate::config::plugin::Plugin;
use crate::config::Configuration;
use crate::config::FileQuery;
use crate::config::Parsable;
use crate::config::ParsableError;

use super::Dependency;
use super::Installable;

/* -------------------------------------------------------------------------- */
/*                                Struct: Addon                               */
/* -------------------------------------------------------------------------- */

/// A handle to a downloaded [`Dependency`], along with the necessary metadata
/// for installing it into a _Godot_ project.
#[derive(Clone, Debug, TypedBuilder)]
pub struct Addon {
    /// The addon's parsed [`Manifest`], if one exists.
    #[builder(default)]
    pub manifest: Option<Manifest>,

    /// The source directory that will be installed into the target _Godot_
    /// project's `addons` directory.
    #[builder(setter(into))]
    pub path: PathBuf,

    /// A list of paths to the [`Addon`]'s exported script templates. These will
    /// be installed in dependent project's template directory if this addon is
    /// a direct dependency of that project.
    #[builder(default)]
    pub script_templates: Vec<PathBuf>,

    /// The name of the directory under `addons` in which the [`Addon`] will be
    /// installed into in the target _Godot_ project.
    pub subfolder: String,

    /// The [`Version`] of the [`Addon`], if known.
    #[builder(default)]
    pub version: Option<semver::Version>,
}

/* ------------------------------- Impl: Addon ------------------------------ */

impl Addon {
    /* --------------------------- Method: Private -------------------------- */

    /// `find_in_dir` searches the specified directory `path` for the [`Addon`]
    /// source directory corresponding to the provided [`Addon`] `name`. An
    /// [`Addon`] must be uniquely defined within the directory, meaning `name`
    /// cannot be resolved into two locations or typed of [`Configuration`]
    /// files. The following locations are considered:
    ///     - `path/`, where `path/plugin.cfg` has `plugin.name == $name`
    ///     - `path/`, where `path/$name.gdextension` is present
    ///     - `path/addons/*/`, where `path/addons/*/plugin.cfg` has
    ///        `plugin.name == $name`
    ///     - `path/addons/*/` where `path/addons/*/$name.gdextension` is
    ///        present
    ///     - `path/addons/$name`, where no `plugin.cfg` or `*.gdextension`
    ///        file is present under `path/addons/$name`
    ///     - `path/`, but only if no `plugin.cfg`, `*.gdextension`, or `addons`
    ///        directory is found under `path/addons/$name`
    ///
    /// NOTE: This method returns a partially populated [`Addon`]. Only the `path`
    /// and [`Plugin`]-specific fields are populated.
    ///
    /// TODO: Substitute in a custom [`Addon`]-specific error.
    fn find_in_dir(path: impl AsRef<Path>, name: &str) -> anyhow::Result<Addon> {
        if !path.as_ref().is_dir() {
            return Addon::find_in_dir(
                path.as_ref().parent().ok_or(anyhow!(
                    "path missing parent directory: {}",
                    path.as_ref().to_str().unwrap_or("''")
                ))?,
                name,
            );
        }

        // Try parsing a [`Manifest`] from the "root" directory `path`.
        let mut m: Option<Manifest> =
            match Manifest::parse_file(path.as_ref().join(Manifest::file_name().unwrap())) {
                Ok(m) => Some(m),
                Err(e) => match e {
                    ParsableError::Io(e) => match e.kind() {
                        std::io::ErrorKind::NotFound => None,
                        _ => return Err(anyhow!(e)),
                    },
                    _ => return Err(anyhow!(e)),
                },
            };

        let plugins = FileQuery::<Plugin>::builder()
            .path(path.as_ref())
            .build()
            .into_iter()
            .collect::<Vec<(PathBuf, Result<Plugin, ParsableError>)>>();

        let extensions = FileQuery::<Extension>::builder()
            .path(path.as_ref())
            .build()
            .into_iter()
            .collect::<Vec<(PathBuf, Result<Extension, ParsableError>)>>();

        // This is an asset-only addon (or walking the project directory failed
        // for some reason). Either return `path/addons/$name/` or `path/`.
        if plugins.is_empty() && extensions.is_empty() {
            let mut path = path.as_ref().to_owned();

            let path_addon = path.as_path().join(format!("addons/{}", name));
            if path_addon.is_dir() {
                path = path_addon;
            }

            // Try parsing a [`Manifest`] defined within the addon path.
            let path_manifest = path.as_path().join(Manifest::file_name().unwrap());
            if path_manifest.exists() && path_manifest.is_file() {
                m.replace(Manifest::parse_file(path_manifest)?);
            }

            // TODO: Parse script templates from the manifest/addon.

            return Ok(Addon::builder()
                .manifest(m)
                .path(path)
                .subfolder(name.to_owned())
                .build());
        }

        // Some plugins and/or extensions were found. Because of this, an addon
        // with `name` *must* be found, otherwise the [`Dependency`] was
        // improperly configured.
        let plugins = plugins
            .into_iter()
            .filter(|(_, r)| {
                r.as_ref()
                    .is_ok_and(|p| p.name().is_some_and(|n| n == name))
            })
            .collect::<Vec<_>>();

        let extensions = extensions
            .into_iter()
            .filter(|(p, _)| {
                p.file_name()
                    .and_then(OsStr::to_str)
                    .is_some_and(|n| n == format!("{}.{}", name, Extension::extension()))
            })
            .collect::<Vec<_>>();

        if plugins.len() + extensions.len() != 1 {
            return Err(anyhow!(
                "could not find addon in directory: {}",
                path.as_ref().to_str().unwrap_or("''")
            ));
        }

        let mut path_addon: Option<PathBuf> = None;
        let mut subfolder: Option<String> = None;
        let mut version: Option<semver::Version> = None;

        for (p, plugin) in plugins {
            let plugin = plugin?;

            path_addon.replace(p.clone());

            // Try to determine the correct "addons" subfolder to install the
            // [`Addon`] into. If set, [`Plugin::subfolder()`] is the canconical
            // value. While [`Plugin::name()`] *should* be next, plugin authors
            // don't always get this right. So, if the [`Addon`] was found under
            // an "addons" directory, then use the actual directory name the
            // author intended.
            if let Some(s) = plugin.subfolder() {
                subfolder.replace(s.to_owned());
            } else if p.components().any(|c| c.as_os_str() == "addons") {
                let mut p = p;

                while p.parent().is_some_and(|p| !p.ends_with("addons")) {
                    p = p.parent().unwrap().to_owned();
                }

                if let Some(name) = p.file_name().and_then(OsStr::to_str) {
                    subfolder.replace(name.to_owned());
                }
            } else if let Some(name) = plugin.name() {
                subfolder.replace(name.to_owned());
            }

            if let Some(v) = plugin.version() {
                version.replace(v);
            }
        }

        for (p, extension) in extensions {
            extension?; // ensure extension parsed successfully.

            path_addon.replace(p);
        }

        // NOTE: This should never fail because `path_addon` was found and set
        // to the path of the configuration file.
        let path = path_addon.as_ref().and_then(|p| p.parent()).ok_or(anyhow!(
            "could not find addon in directory: {}",
            path.as_ref().to_str().unwrap_or("''")
        ))?;

        // Try parsing a [`Manifest`] defined within the addon path.
        let path_manifest = path.join(Manifest::file_name().unwrap());
        if path_manifest.is_file() {
            m.replace(Manifest::parse_file(path_manifest)?);
        }

        // TODO: Parse script templates from the manifest/addon.

        Ok(Addon::builder()
            .manifest(m)
            .path(path)
            .subfolder(subfolder.unwrap_or(name.to_owned()))
            .version(version)
            .build())
    }
}

/* ---------------------------- Impl: Installable --------------------------- */

impl Installable for Addon {
    fn install_to(&self, target: impl AsRef<Path>) -> Result<(), std::io::Error> {
        if !target.as_ref().is_dir() {
            std::fs::create_dir_all(target.as_ref())?;
        }

        let mut target = target.as_ref().canonicalize()?;

        // If the `target` directory doesn't include "addons", then assume its
        // a project root.
        if !target.components().any(|c| c.as_os_str() == "addons") {
            target = target.join("addons").join(&self.subfolder);
        } else {
            while !target.ends_with("addons") {
                if let Some(p) = target.parent() {
                    target = p.to_owned();
                    continue;
                }

                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!(
                        "couldn't determine install path for addon: {}",
                        self.subfolder
                    ),
                ));
            }

            target = target.join(&self.subfolder)
        }

        if target.is_dir() {
            std::fs::remove_dir_all(&target)?;
        }

        super::clone_recursively(self.path.as_path(), &target, |src, dst| {
            std::fs::hard_link(src, dst)
        })
    }
}

/* ----------------------- Impl: TryFrom<&Dependency> ----------------------- */

impl TryFrom<&Dependency> for Addon {
    type Error = anyhow::Error;

    fn try_from(value: &Dependency) -> Result<Self, Self::Error> {
        let root = value.source.fetch().map_err(|e| anyhow!(e))?;

        let name = value
            .addon
            .as_ref()
            .ok_or(anyhow!("cannot determine addon name"))?;

        let mut addon = Addon::find_in_dir(root, name)?;

        // In the event that this isn't a [`Plugin`], try to parse the version
        // from a 'git' tag.
        if addon.version.is_none() {
            match &value.source {
                super::Source::Release(r) => {
                    let tag = r.tag.as_str().strip_prefix('v').unwrap_or(&r.tag);
                    if let Ok(v) = semver::Version::parse(tag) {
                        addon.version.replace(v);
                    }
                }
                super::Source::Git(g) => {
                    if let Some(crate::git::Reference::Tag(tag)) = &g.reference {
                        if let Ok(v) = semver::Version::parse(tag) {
                            addon.version.replace(v);
                        }
                    }
                }
                _ => {}
            };
        }

        Ok(addon)
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Mod: Tests                                 */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use semver::Version;
    use std::fs::create_dir_all;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    use crate::core::Addon;

    macro_rules! write_file {
        ($path:expr, $content:expr$(,)?) => {
            create_dir_all($path.parent().expect("invalid path"))
                .expect("failed to create directory");

            File::create($path)
                .and_then(|mut f| f.write_all($content.as_bytes()))
                .expect("failed to create file");
        };
    }

    macro_rules! assert_eq_addon {
        ($left:expr, $right:expr$(,)?) => {
            assert_eq!($left.path, $right.path);
            assert_eq!($left.script_templates, $right.script_templates);
            assert_eq!($left.subfolder, $right.subfolder);
            assert_eq!($left.version, $right.version)
        };
    }

    #[test]
    fn test_addons_find_in_dir_fails_if_mismatching_plugin() {
        let tmp = tempdir().expect("failed to make temporary directory");

        let invalid = "[plugin]\nname='invalid'\nversion='1.2.3'";
        write_file!(&tmp.path().join("plugin.cfg"), invalid);

        assert!(Addon::find_in_dir(tmp.path(), "addon").is_err());
    }

    #[test]
    fn test_addons_find_in_dir_fails_if_mismatching_extension() {
        let tmp = tempdir().expect("failed to make temporary directory");

        write_file!(&tmp.path().join("invalid.gdextension"), "");

        assert!(Addon::find_in_dir(tmp.path(), "addon").is_err());
    }

    #[test]
    fn test_addons_find_in_dir_with_root_plugin() {
        let tmp = tempdir().expect("failed to make temporary directory");
        let path = tmp.path().join("plugin.cfg");

        let plugin = "[plugin]\nname='addon'\nversion='1.2.3'";
        write_file!(&path, plugin);

        assert_eq_addon!(
            Addon::find_in_dir(tmp.path(), "addon").expect("couldn't find addon"),
            Addon::builder()
                .path(path.parent().unwrap())
                .subfolder(String::from("addon"))
                .version(Some(Version::new(1, 2, 3)))
                .build()
        );
    }

    #[test]
    fn test_addons_find_in_dir_with_root_ext() {
        let tmp = tempdir().expect("failed to make temporary directory");
        let path = tmp.path().join("addon.ext");

        write_file!(&path, "");

        assert_eq_addon!(
            Addon::find_in_dir(tmp.path(), "addon").expect("couldn't find addon"),
            Addon::builder()
                .path(path.parent().unwrap())
                .subfolder(String::from("addon"))
                .build()
        );
    }

    #[test]
    fn test_addons_find_in_dir_with_plugin_in_addons() {
        let tmp = tempdir().expect("failed to make temporary directory");
        let path = tmp.path().join("addons/addon/plugin.cfg");

        let plugin = "[plugin]\nname='addon'\nversion='1.2.3'";
        write_file!(&path, plugin);

        assert_eq_addon!(
            Addon::find_in_dir(tmp.path(), "addon").expect("couldn't find addon"),
            Addon::builder()
                .path(path.parent().unwrap())
                .subfolder(String::from("addon"))
                .version(Some(Version::new(1, 2, 3)))
                .build()
        );
    }

    #[test]
    fn test_addons_find_in_dir_with_extension_in_addons() {
        let tmp = tempdir().expect("failed to make temporary directory");
        let path = tmp.path().join("addons/addon/addon.gdextension");

        write_file!(&path, "");

        assert_eq_addon!(
            Addon::find_in_dir(tmp.path(), "addon").expect("couldn't find addon"),
            Addon::builder()
                .path(path.parent().unwrap())
                .subfolder(String::from("addon"))
                .build()
        );
    }

    #[test]
    fn test_addons_find_in_dir_with_static_in_addons() {
        let tmp = tempdir().expect("failed to make temporary directory");
        let path = tmp.path().join("addons/addon/project.godot");

        write_file!(&path, "");

        assert_eq_addon!(
            Addon::find_in_dir(tmp.path(), "addon").expect("couldn't find addon"),
            Addon::builder()
                .path(path.parent().unwrap())
                .subfolder(String::from("addon"))
                .build()
        );
    }

    #[test]
    fn test_addons_find_in_dir_with_static_root() {
        let tmp = tempdir().expect("failed to make temporary directory");

        assert_eq_addon!(
            Addon::find_in_dir(tmp.path(), "addon").expect("couldn't find addon"),
            Addon::builder()
                .path(tmp.path())
                .subfolder(String::from("addon"))
                .build()
        );
    }
}
