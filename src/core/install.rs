use semver::Version;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::Path;
use typed_builder::TypedBuilder;

use crate::config::Manifest;

use super::Addon;
use super::Dependency;
use super::Installable;

/* -------------------------------------------------------------------------- */
/*                               Struct: Install                              */
/* -------------------------------------------------------------------------- */

#[derive(Debug, TypedBuilder)]
pub struct Install<'a> {
    #[builder(default = true)]
    pub dev: bool,
    pub root: &'a Manifest,
    #[builder(default)]
    pub targets: Vec<Option<&'a str>>,
}

/* ------------------------------ Impl: Install ----------------------------- */

impl<'a> Install<'a> {
    pub fn install_to(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let addons = self.resolve_addons()?;

        for addon in addons {
            addon
                .install_to(&path)
                .map_err(|e| Error::Install(addon.subfolder, e))?;
        }

        Ok(())
    }

    pub fn resolve_addons(&self) -> Result<Vec<Addon>, Error> {
        let mut to_install: HashMap<String, (Addon, Dependency)> = HashMap::new(); // addon name -> ...
        let mut subfolders = HashMap::<String, String>::new(); // subfolder name -> addon name

        let mut to_visit = self
            .root
            .dependencies(self.dev, self.targets.iter().copied())
            .map(VecDeque::from)
            .map_err(Error::Config)?;

        'dep: while let Some(dep) = to_visit.pop_front() {
            // Download the [`Dependency`] to the `gdpack` store.
            dep.source.fetch().map_err(Error::Fetch)?;

            // Load the [`Addon`] from the fetched [`Dependency`].
            let addon = Addon::try_from(&dep).map_err(Error::Load)?;

            let name = dep
                .addon
                .as_ref()
                .map(String::to_owned)
                .ok_or(Error::MissingName)?;

            // Validate the following invariants of the dependency set:

            // 1. The same addon cannot be specified 2+ times. The exception to
            //    this are versioned plugins; the latest major-compatible
            //    release of a plugin will be selected (under the assumption
            //    that semantic versioning rules are upheld). if competing major
            //    versions are found, an error will be returned.
            //
            //    NOTE: For non-plugins, there isn't a canonical way to
            //    determine the version of the dependency - it depends on the
            //    project. One common scenario that *is* supported are tags on
            //    'git'-based dependencies: if the conflicting dependencies are
            //    both specified by semver-compatible tags, then those values
            //    will be used. Otherwise any duplicated addons will result in
            //    an error.
            if let Some((existing, dep_prev)) = to_install.get(&name) {
                match existing.version.as_ref() {
                    None => {
                        // At this point, there's no safe way to verify that the
                        // "existing" and "next" addons are compatible. The only
                        // possibility is if the [`Dependency`] specifications
                        // are equivalent.
                        if &dep != dep_prev {
                            return Err(Error::Incompatible(
                                name,
                                None::<Version>,
                                None::<Version>,
                            ));
                        }
                    }
                    Some(v_prev) => match addon.version.as_ref() {
                        None => {
                            return Err(Error::Incompatible(
                                name,
                                None::<Version>,
                                Some(v_prev.clone()),
                            ))
                        }
                        Some(v_next) => {
                            if v_next.major != v_prev.major {
                                return Err(Error::Incompatible(
                                    name,
                                    Some(v_prev.clone()),
                                    Some(v_next.clone()),
                                ));
                            }

                            // Only update the addon version if "current" is
                            // newer (after checking compatible major versions).
                            if v_prev.cmp_precedence(v_next) != std::cmp::Ordering::Less {
                                continue 'dep;
                            }
                        }
                    },
                }

                // Remove the existing addon's subfolder entry, if present,
                // since this can safely change between versions.
                subfolders.remove(&existing.subfolder);
            }

            // 2. Multiple addons cannot be installed to the same subfolder
            //    within the Godot project's "addons" folder.
            let subfolder = addon.subfolder.to_owned();

            if subfolders.contains_key(&subfolder) {
                let other = subfolders.get(&subfolder).unwrap();
                return Err(Error::Subfolder(subfolder, name, other.to_owned()));
            }

            subfolders.insert(subfolder, name.clone());

            // Now, register the valid addon as to-be-installed and add any
            // dependencies it has to the queue.

            if let Some(m) = addon.manifest.as_ref() {
                // TODO: Allow more robust selection of transitive dependencies
                // which are specific to targets specified in the dependency.
                let targets = self.targets.iter().copied(); // TODO: This just
                let dev = false; // Don't install transitive dev-only addons.

                to_visit.extend(m.dependencies(dev, targets).map_err(Error::Config)?);
            }

            to_install.insert(name, (addon, dep.clone()));
        }

        Ok(to_install.into_values().map(|(a, _)| a).collect())
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Enum: Error                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Config(crate::config::manifest::Error),
    #[error(transparent)]
    Fetch(anyhow::Error),
    // TODO: Provide additional information about the dependency chain that
    // resulted in each addon being included.
    #[error("incompatible versions found for addon '{0}': {1:?}, {2:?}")]
    Incompatible(String, Option<Version>, Option<Version>),
    #[error("failed to install addon {0}: {1:?}")]
    Install(String, std::io::Error),
    #[error(transparent)]
    Load(anyhow::Error),
    #[error("could not determine addon name")]
    MissingName,
    #[error("duplicate subfolder found between addons '{1}' and '{2}': {0}")]
    Subfolder(String, String, String),
}

/* -------------------------------------------------------------------------- */
/*                                 Mod: Tests                                 */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use semver::Version;
    use std::collections::HashSet;
    use std::path::Path;
    use typed_builder::TypedBuilder;

    use crate::config::gdext::Extension;
    use crate::config::manifest::Query;
    use crate::config::plugin::Plugin;
    use crate::config::Configuration;
    use crate::config::Manifest;
    use crate::config::Persistable;
    use crate::core::Addon;
    use crate::core::Dependency;
    use crate::core::Source;

    use super::Install;

    /* ------------------------------ Test: run ----------------------------- */

    macro_rules! assert_addons_eq {
        ($got:expr, $want:expr$(,)?) => {
            assert_eq!(
                $got.into_iter().collect::<HashSet<Addon>>(),
                $want.into_iter().collect::<HashSet<Addon>>(),
            )
        };
    }

    #[rstest]
    fn test_installer_run_empty_succeeds() {
        // Given: A root manifest with no dependencies.
        let m = Manifest::default();

        // When: An installation is run for the default target/environment.
        let got = Install::builder().root(&m).build().resolve_addons();

        // Then: An empty list is returned.
        assert_addons_eq!(got.unwrap(), vec![])
    }

    #[rstest]
    fn test_installer_run_direct_deps_succeeds(
        #[values(
            DepType::Assets,
            DepType::Extension,
            DepType::Plugin(Version::new(1, 0, 0)),
            DepType::RootExtension,
            DepType::RootPlugin(Version::new(1, 0, 0))
        )]
        dep_type_1: DepType,
        #[values(
            DepType::Assets,
            DepType::Extension,
            DepType::Plugin(Version::new(1, 0, 0)),
            DepType::RootExtension,
            DepType::RootPlugin(Version::new(1, 0, 0))
        )]
        dep_type_2: DepType,
    ) {
        // Given: A temporary test directory for creating dependencies.
        let tmp = tempfile::tempdir().unwrap();

        // Given: Two simple de pendencies that exist on disk.
        let dep1 = TestDep::builder()
            .name("1")
            .addon(dep_type_1)
            .build()
            .init(tmp.path().join("1"))
            .unwrap();
        let dep2 = TestDep::builder()
            .name("2")
            .addon(dep_type_2)
            .build()
            .init(tmp.path().join("2"))
            .unwrap();

        // Given: A root manifest with direct dependencies.
        let mut m = Manifest::default();
        m.addons_mut(&Query::prod()).insert(&dep1);
        m.addons_mut(&Query::prod()).insert(&dep2);

        // When: An installation is run for the default target/environment.
        let got = Install::builder().root(&m).build().resolve_addons();

        // Then: An empty list is returned.
        assert_addons_eq!(
            got.unwrap(),
            vec![
                Addon::try_from(&dep1).unwrap(),
                Addon::try_from(&dep2).unwrap()
            ]
        )
    }

    #[rstest]
    fn test_installer_run_transitive_deps_succeeds(
        #[values(
            DepType::Assets,
            DepType::Extension,
            DepType::Plugin(Version::new(1, 0, 0)),
            DepType::RootExtension,
            DepType::RootPlugin(Version::new(1, 0, 0))
        )]
        dep_type_1: DepType,
        #[values(
            DepType::Assets,
            DepType::Extension,
            DepType::Plugin(Version::new(1, 0, 0)),
            DepType::RootExtension,
            DepType::RootPlugin(Version::new(1, 0, 0))
        )]
        dep_type_2: DepType,
        #[values(
            DepType::Assets,
            DepType::Extension,
            DepType::Plugin(Version::new(1, 0, 0)),
            DepType::RootExtension,
            DepType::RootPlugin(Version::new(1, 0, 0))
        )]
        dep_type_3: DepType,
    ) {
        // Given: A temporary test directory for creating dependencies.
        let tmp = tempfile::tempdir().unwrap();

        // Given: Two simple dependencies that exist on disk.
        let dep1 = TestDep::builder()
            .name("1")
            .addon(dep_type_1)
            .build()
            .init(tmp.path().join("1"))
            .unwrap();
        let dep2 = TestDep::builder()
            .name("2")
            .addon(dep_type_2)
            .deps(vec![dep1.clone()])
            .build()
            .init(tmp.path().join("2"))
            .unwrap();
        let dep3 = TestDep::builder()
            .name("3")
            .addon(dep_type_3)
            .deps(vec![dep2.clone()])
            .build()
            .init(tmp.path().join("3"))
            .unwrap();

        // Given: A root manifest with direct dependencies.
        let mut m = Manifest::default();
        m.addons_mut(&Query::prod()).insert(&dep3);

        // When: An installation is run for the default target/environment.
        let got = Install::builder().root(&m).build().resolve_addons();

        // Then: An empty list is returned.
        assert_addons_eq!(
            got.unwrap(),
            vec![
                Addon::try_from(&dep1).unwrap(),
                Addon::try_from(&dep2).unwrap(),
                Addon::try_from(&dep3).unwrap(),
            ]
        )
    }

    /* ---------------------------------------------------------------------- */
    /*                             Struct: TestDep                            */
    /* ---------------------------------------------------------------------- */

    #[derive(Clone, Debug, TypedBuilder)]
    struct TestDep {
        #[builder(default = DepType::Assets)]
        addon: DepType,
        #[builder(default)]
        deps: Vec<Dependency>,
        #[builder(setter(into))]
        name: String,
    }

    /* ---------------------------- Enum: DepType --------------------------- */

    #[derive(Clone, Debug)]
    enum DepType {
        Assets,
        Extension,
        Plugin(Version),
        RootExtension,
        RootPlugin(Version),
    }

    /* ---------------------------- Impl: TestDep --------------------------- */

    impl TestDep {
        #[allow(dead_code)]
        fn init(&self, root: impl AsRef<Path>) -> anyhow::Result<Dependency> {
            let root = root.as_ref();

            let path = match &self.addon {
                DepType::Assets => root.to_owned(),
                DepType::Extension => root.join("addons").join(&self.name),
                DepType::Plugin(_) => root.join("addons").join(&self.name),
                DepType::RootExtension => root.to_owned(),
                DepType::RootPlugin(_) => root.to_owned(),
            };

            match &self.addon {
                DepType::Assets => {
                    std::fs::create_dir_all(&path)?;
                    std::fs::write(path.join("asset.txt"), "")?;
                }
                DepType::Extension => {
                    std::fs::create_dir_all(&path)?;
                    std::fs::write(
                        path.join(format!("{}.{}", self.name, Extension::extension())),
                        "",
                    )?;
                }
                DepType::Plugin(v) => {
                    std::fs::create_dir_all(&path)?;
                    std::fs::write(
                        path.join(Plugin::file_name().unwrap()),
                        format!("[plugin]\nname={}\nversion={}", self.name, v),
                    )?;
                }
                DepType::RootExtension => {
                    std::fs::create_dir_all(&path)?;
                    std::fs::write(
                        path.join(format!("{}.{}", self.name, Extension::extension())),
                        "",
                    )?;
                }
                DepType::RootPlugin(v) => {
                    std::fs::create_dir_all(&path)?;
                    std::fs::write(
                        path.join(Plugin::file_name().unwrap()),
                        format!("[plugin]\nname={}\nversion={}", self.name, v),
                    )?;
                }
            };

            if !self.deps.is_empty() {
                let mut m = Manifest::default();

                for dep in &self.deps {
                    m.addons_mut(&Query::prod()).insert(dep);
                }

                std::fs::create_dir_all(root)?;
                m.persist(root.join(Manifest::file_name().unwrap()))?;
            }

            Ok(Dependency::builder()
                .addon(Some(self.name.to_owned()))
                .replace(None)
                .source(Source::Path {
                    path: root.to_owned(),
                })
                .build())
        }
    }
}
