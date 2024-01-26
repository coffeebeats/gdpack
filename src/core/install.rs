use semver::Version;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::path::Path;
use std::path::PathBuf;
use typed_builder::TypedBuilder;

use crate::config::Manifest;

use super::Addon;
use super::Dependency;
use super::Installable;
use super::ScriptTemplateScan;

/* -------------------------------------------------------------------------- */
/*                               Struct: Install                              */
/* -------------------------------------------------------------------------- */

#[derive(Debug, TypedBuilder)]
pub struct Install<'a> {
    #[builder(default = true)]
    pub dev: bool,
    pub manifest: &'a Manifest,
    #[builder(default)]
    pub targets: Vec<Option<&'a str>>,
}

/* ------------------------------ Impl: Install ----------------------------- */

impl<'a> Install<'a> {
    /// `install_to` runs the install procedure for the contained [`Manifest`]
    /// on the Godot project rooted at `path`.
    pub fn install_to(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let mut path = path.as_ref();

        // First, validate that the provided path is a project.

        if path.ends_with("addons") {
            path = path.parent().unwrap();
        }

        if !path.join("project.godot").is_file() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "missing Godot project config: {}",
                    path.join("project.godot").to_str().unwrap()
                ),
            )));
        }

        // Next, determine the set of addons to install. This is done before
        // any destructive operations are performed to ensure the operation is
        // valid.
        let addons = self.resolve_addons(path)?;

        // Next, install each addon into the project's "addons" folder.
        let path_addons = path.join("addons");
        if path_addons.is_dir() {
            std::fs::remove_dir_all(&path_addons).map_err(Error::Io)?;
        }

        for addon in addons {
            addon
                .install_to(&path)
                .map_err(|e| Error::Install(addon.subfolder, e))?;
        }

        // Next, clean any imported GDScript templates from the project.
        Install::clean_script_templates(path)?;

        // Finally, import all of the script templates defined for the project.
        let target = path.join("script_templates");
        if let Some(st) = self.manifest.project().get_script_templates() {
            match st.included_from(path) {
                Err(e) => return Err(Error::Project(e)),
                Ok(templates) => {
                    for t in templates {
                        let target = target.join(&t);
                        std::fs::create_dir_all(target.parent().unwrap()).map_err(Error::Io)?;
                        std::fs::hard_link(path.join(&t), target).map_err(Error::Io)?;
                    }
                }
            }
        }

        Ok(())
    }

    /* -------------------------- Methods: Private -------------------------- */

    /// `clean_script_templates` removes all imported GDScript files found
    /// within the directory `path`.
    fn clean_script_templates(path: impl AsRef<Path>) -> Result<(), Error> {
        let path = path.as_ref();

        let mut to_remove: HashSet<PathBuf> = HashSet::new();

        for path_template in ScriptTemplateScan::builder()
            .path(&path)
            .contents_first(true)
            .skip_imported(false)
            .skip_nonimported(true)
            .build()
        {
            let path_template = path.join(path_template);

            if path_template.is_file() {
                std::fs::remove_file(&path_template).map_err(Error::Io)?;
                to_remove.insert(path_template.parent().unwrap().to_owned());
            }

            if path_template.is_dir()
                && to_remove.contains(&path_template)
                && std::fs::read_dir(&path_template).is_ok_and(|d| d.count() == 0)
            {
                std::fs::remove_dir(path_template).map_err(Error::Io)?;
            }
        }

        Ok(())
    }

    /// `resolve_addons` determines the set of [`Addon`] dependencies to install
    /// into the project specified by `path`.
    fn resolve_addons(&self, path: impl AsRef<Path>) -> Result<Vec<Addon>, Error> {
        let mut to_install: HashMap<String, (Addon, Dependency)> = HashMap::new(); // addon name -> ...
        let mut subfolders = HashMap::<String, String>::new(); // subfolder name -> addon name

        let mut to_visit = self
            .manifest
            .dependencies(path, self.dev, self.targets.iter().copied())
            .map(VecDeque::from)
            .map_err(Error::Config)?;

        'dep: while let Some(dep) = to_visit.pop_front() {
            // Download the [`Dependency`] to the `gdpack` store.
            let path_dep = dep.download().map_err(Error::Dependency)?;

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
                let targets = self.targets.iter().copied();
                let dev = false; // Don't install transitive dev-only addons.

                let deps = m
                    .dependencies(path_dep, dev, targets)
                    .map_err(Error::Config)?;

                to_visit.extend(deps);
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
    Dependency(super::dependency::Error),
    #[error(transparent)]
    Io(std::io::Error),
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
    #[error(transparent)]
    Project(super::project::Error),
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
    use std::path::PathBuf;
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
        let got = Install::builder()
            .manifest(&m)
            .build()
            .resolve_addons(PathBuf::from("."));

        // Then: An empty list is returned.
        assert_addons_eq!(got.unwrap(), vec![]);
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
            .init(tmp.path(), "1")
            .unwrap();
        let dep2 = TestDep::builder()
            .name("2")
            .addon(dep_type_2)
            .build()
            .init(tmp.path(), "2")
            .unwrap();

        // Given: A root manifest with direct dependencies.
        let mut m = Manifest::default();
        m.addons_mut(&Query::prod()).insert(&dep1);
        m.addons_mut(&Query::prod()).insert(&dep2);

        // When: An installation is run for the default target/environment.
        let got = Install::builder()
            .manifest(&m)
            .build()
            .resolve_addons(tmp.path());

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
        // NOTE: Cannot use 'DepType::Assets' as that would violate the rules of
        // [`Addon::find_in_dir()`].
        #[values(
            DepType::Extension,
            DepType::Plugin(Version::new(1, 0, 0)),
            DepType::RootExtension,
            DepType::RootPlugin(Version::new(1, 0, 0))
        )]
        dep_type_2: DepType,
        // NOTE: Cannot use 'DepType::Assets' as that would violate the rules of
        // [`Addon::find_in_dir()`].
        #[values(
            DepType::Extension,
            DepType::Plugin(Version::new(1, 0, 0)),
            DepType::RootExtension,
            DepType::RootPlugin(Version::new(1, 0, 0))
        )]
        dep_type_3: DepType,
    ) {
        // Given: A temporary test directory for creating dependencies.
        let tmp = tempfile::tempdir().unwrap();

        // Given: A directory containing the test project.
        let path_project = tmp.path();

        // Given: Three simple dependencies that exist on disk.
        let dep1 = TestDep::builder()
            .name("1")
            .addon(dep_type_1)
            .build()
            .init(&path_project, "./3/2/1")
            .unwrap()
            .rooted_at(tmp.path());
        let dep2 = TestDep::builder()
            .name("2")
            .addon(dep_type_2)
            .deps(vec![dep1.clone()])
            .build()
            .init(&path_project, "./3/2")
            .unwrap();
        let dep3 = TestDep::builder()
            .name("3")
            .addon(dep_type_3)
            .deps(vec![dep2.clone()])
            .build()
            .init(&path_project, "./3")
            .unwrap();

        // Given: A root manifest with direct dependencies.
        let mut m = Manifest::default();
        m.addons_mut(&Query::prod()).insert(&dep3);

        // When: An installation is run for the default target/environment.
        let got = Install::builder()
            .manifest(&m)
            .build()
            .resolve_addons(&path_project);

        // Then: The resolved addons match expectations.
        assert_addons_eq!(
            got.unwrap(),
            vec![
                Addon::try_from(&dep1).unwrap(),
                Addon::try_from(&dep2).unwrap(),
                Addon::try_from(&dep3).unwrap(),
            ]
        );
    }

    /* ---------------------------------------------------------------------- */
    /*                           Struct: Dependency                           */
    /* ---------------------------------------------------------------------- */

    /* -------------------------- Impl: Dependency -------------------------- */

    impl Dependency {
        fn rooted_at(&self, path: impl AsRef<Path>) -> Dependency {
            let mut dep = self.clone();
            dep.included_from.replace(path.as_ref().to_owned());

            dep
        }
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
        fn init(
            &self,
            tmp: impl AsRef<Path>,
            root: impl AsRef<Path>,
        ) -> anyhow::Result<Dependency> {
            let mut root = root.as_ref().to_owned();

            if !root.is_absolute() {
                root = tmp.as_ref().join(root);
            }

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

                // NOTE: While it would be a more robust test if Manifest were
                // placed in `path`, that complicates the placement of the
                // addons.
                m.persist(root.join(Manifest::file_name().unwrap()))?;
            }

            Ok(Dependency::builder()
                .addon(Some(self.name.to_owned()))
                .replace(None)
                .included_from(Some(tmp.as_ref().to_owned()))
                .source(Source::Path {
                    path: root.to_owned(),
                })
                .build())
        }
    }
}
