/* ------------------------------- Mod: addon ------------------------------- */

mod addon;

pub use addon::Addons;
pub use addon::AddonsMut;

/* -------------------------------- Mod: key -------------------------------- */

mod key;

pub use key::Query;

/* ------------------------------ Mod: project ------------------------------ */

mod project;

pub use project::Project;

/* -------------------------------------------------------------------------- */
/*                              Struct: Manifest                              */
/* -------------------------------------------------------------------------- */

use anyhow::anyhow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use toml_edit::Document;

use crate::core::Dependency;

use super::Configuration;
use super::Parsable;
use super::ParsableError;

const MANIFEST_FILENAME: &str = "gdpack.toml";

/* -------------------------------------------------------------------------- */
/*                              Struct: Manifest                              */
/* -------------------------------------------------------------------------- */

/// A wrapper around a formatted [`toml_edit::Document`] that provides
/// operations to manage [`Dependency`] and configuration information for a
/// Godot project.
#[derive(Clone, Debug)]
pub struct Manifest(Document);

/* ----------------------------- Impl: Manifest ----------------------------- */

impl Manifest {
    /* --------------------------- Methods: Public -------------------------- */

    /// New creates a new [`Manifest`] using a template suitable for end users.
    /// This is in contrast to the [`Manifest::default`] implementation which
    /// creates an empty manifest.
    pub fn new() -> Manifest {
        let template = r#"# Use `gdpack add` to import addon dependencies into your project.
# 
# For example:
#    gdpack add https://github.com/bitwes/Gut --tag v9.1.1 -d

[project.export_files]
# A list of glob expressions which, when any match are matched, cause an addon
# file to *not* be installed into a Godot project. This option is particularly
# useful for omitting files and folders from an addon with a root 'plugin.cfg'.
# Note that hidden files and folders and the 'gdpack.toml' manifest are excluded
# by default.
exclude = []

# A list of glob expressions which, when any match are matched, cause an addon
# file to be installed into a Godot project when it otherwise wouldn't be. This
# option takes precedence over excluded files.
include = []

[project.script_templates]
# Include additional script templates from these directories into the project.
# The directory structure within the folder pointed to by each pattern will be
# merged into this project's 'script_templates' folder.
include = []

# Export non-imported script templates found in these directories.
export = []

[dev-addons]
Gut = { git = "https://github.com/bitwes/Gut.git", tag = "v9.2.1" }
"#;

        Manifest(template.parse::<Document>().unwrap())
    }

    /// Returns an _immutable_ view of the addons recorded for the provided
    /// [`Query`].
    pub fn addons<'a>(&'a self, query: &'a Query) -> Addons {
        Addons::builder().document(&self.0).query(query).build()
    }

    /// Returns a mutable view of the addons recorded for the provided [`Query`].
    pub fn addons_mut<'a>(&'a mut self, query: &'a Query) -> AddonsMut {
        AddonsMut::builder()
            .document(&mut self.0)
            .query(query)
            .build()
    }

    /// Returns the [`Dependency`] list found within the [`Manifest`] for the
    /// specified `target` list and environment. The default target will always
    /// be included for the returned dependencies.
    ///
    /// NOTE: A `path` to the directory from which dependencies are being
    /// included must be provided. This is so that a [`Dependency`] with a
    /// relative [`crate::core::Source::Path`] can be properly resolved.
    ///
    /// NOTE: There are a few invariants upheld when gathering dependencies
    /// within a manifest. These are as follows:
    ///     1. The same addon cannot be specified 2+ times within the same
    ///        environment or in multiple targets. However, an addon specified
    ///        within a target may override the default target.
    ///     2. The same addon cannot be replaced by 2+ addons. Note that because
    ///        replacements can only be specified within a target, any collision
    ///        is guaranteed to be an invalid state.
    pub fn dependencies<'a>(
        &self,
        path: impl AsRef<Path>,
        is_dev: bool,
        targets: impl IntoIterator<Item = Option<&'a str>>,
    ) -> Result<Vec<Dependency>, Error> {
        let mut out: Vec<(Query, Dependency)> = vec![];

        let mut targets = targets.into_iter().collect::<Vec<_>>();
        if !targets.contains(&None) {
            targets.push(None);
        }

        let mut queries: Vec<Query> = targets
            .into_iter()
            .map(|t| {
                Query::builder()
                    .dev(false)
                    .target(t.map(str::to_owned))
                    .build()
            })
            .collect();
        if is_dev {
            queries = queries
                .iter()
                .cloned()
                .chain(queries.iter().map(Query::invert_dev))
                .collect()
        }

        for query in queries {
            if query.target.as_ref().is_some_and(|t| t.is_empty()) {
                return Err(Error::MissingTarget);
            }

            out.extend(self.addons(&query).into_iter().map(|mut d| {
                let _ = d.included_from.insert(path.as_ref().to_owned());
                (query.clone(), d)
            }));
        }

        Manifest::check_for_duplicate(&out)?;
        Manifest::check_for_double_replace(&out)?;

        // Remove any production addons overriden by development addons.
        let dev_addons: HashSet<String> = out
            .iter()
            .filter(|(q, _)| q.dev)
            .filter(|(_, d)| d.addon.is_some())
            .map(|(_, d)| d.addon.as_ref().unwrap().to_owned())
            .collect();

        let out: Vec<(Query, Dependency)> = out
            .into_iter()
            .filter(|(q, d)| q.dev || d.addon.as_ref().is_some_and(|a| !dev_addons.contains(a)))
            .collect();

        Ok(out.into_iter().map(|(_, d)| d).collect())
    }

    /// Returns an _immutable_ view of the project configuration within the
    /// [`Manifest`].
    pub fn project(&self) -> Project {
        Project::builder().document(&self.0).build()
    }

    /* -------------------------- Methods: Private -------------------------- */

    /// `check_for_duplicate` validates that the provided [`Dependency`] list
    /// does not contain duplicate specifications of an [`crate::core::Addon`].
    fn check_for_duplicate(deps: &[(Query, Dependency)]) -> Result<(), Error> {
        // Map from addon name to the target which specified it.
        let mut declared: HashMap<String, Query> = HashMap::new();

        for (query, dep) in deps {
            let name = dep
                .addon
                .as_ref()
                .map(String::to_owned)
                .ok_or(Error::MissingName)?;

            // Insert the addon as-is the first time it's encountered.
            if !declared.contains_key(&name) {
                declared.insert(name.to_owned(), query.clone());
                continue;
            }

            let existing = declared.remove(&name).unwrap();

            // If the [`Query`] targets are equivalent then override a
            // production addon with a development one. If the environments are
            // the same, return an error.
            if query.target.as_ref() == existing.target.as_ref() {
                match &query.dev {
                    true => match &existing.dev {
                        true => {}
                        false => {
                            let _ = declared.insert(name.to_owned(), query.clone());
                            continue;
                        }
                    },
                    false => match &existing.dev {
                        false => {}
                        true => {
                            let _ = declared.insert(name.to_owned(), existing);
                            continue;
                        }
                    },
                }

                return Err(Error::Duplicate(
                    name,
                    vec![query.clone(), existing.to_owned()],
                ));
            }

            match query.target.as_deref() {
                None => match existing.target.as_ref() {
                    // Skip the default target because a specified target
                    // declares this addon as a dependency.
                    Some(_) => continue,
                    None => unreachable!(),
                },
                Some(_) => match &existing.target {
                    // Override the default target because this target
                    // declares this addon as a dependency.
                    None => declared.insert(name.to_owned(), existing),
                    // Equivalent targets have already been handled, so this
                    // will always be an error.
                    Some(_) => {
                        return Err(Error::Duplicate(
                            name,
                            vec![query.clone(), existing.to_owned()],
                        ))
                    }
                },
            };
        }

        Ok(())
    }

    /// `check_replace` validates that the provided [`Dependency`] list does not
    /// contain duplicate replacements of an [`crate::core::Addon`].
    fn check_for_double_replace(deps: &[(Query, Dependency)]) -> Result<(), Error> {
        // Map from replaced addon name to the target which specified it.
        let mut replaced: HashMap<String, String> = HashMap::new();

        for (query, dep) in deps {
            if dep.replace.is_none() {
                continue;
            }

            let name = dep
                .addon
                .as_ref()
                .map(String::to_owned)
                .ok_or(Error::MissingName)?;

            let target = match query.target.as_ref() {
                None => return Err(Error::InvalidReplace(name)),
                Some(t) => t.to_owned(),
            };

            let replace = dep.replace.as_ref().unwrap();

            if replaced.contains_key(replace) {
                return Err(Error::DoubleReplace(
                    name,
                    vec![target, replaced.get(replace).unwrap().to_owned()],
                ));
            }

            replaced.insert(replace.to_owned(), target);
        }

        Ok(())
    }
}

/* --------------------------- Impl: IntoIterator --------------------------- */

impl IntoIterator for &Manifest {
    type Item = (Query, Dependency);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let targets_self = self
            .0
            .get(key::MANIFEST_SECTION_TARGET)
            .and_then(|v| v.as_table_like())
            .map(|t| t.iter().map(|(a, _)| Some(a)).collect::<Vec<_>>())
            .unwrap_or_default();

        let queries_self = targets_self
            .iter()
            .chain(&None)
            .map(|t| Query::builder().target(t.map(str::to_owned)).build())
            .chain(targets_self.iter().chain(&None).map(|t| {
                Query::builder()
                    .dev(false)
                    .target(t.map(str::to_owned))
                    .build()
            }));

        queries_self
            .flat_map(|q| self.addons(&q).into_iter().map(move |d| (q.clone(), d)))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

/* --------------------------- Impl: Configuration -------------------------- */

impl Configuration for Manifest {
    fn file_name<'a>() -> Option<&'a str> {
        Some(MANIFEST_FILENAME)
    }

    fn matches(path: impl AsRef<std::path::Path>) -> bool {
        path.as_ref()
            .file_name()
            .is_some_and(|s| s == MANIFEST_FILENAME)
    }
}

/* ----------------------------- Impl: Parsable ----------------------------- */

impl Parsable for Manifest {
    fn parse(contents: &str) -> Result<Self, ParsableError> {
        let doc = contents
            .parse::<Document>()
            .map_err(|e| ParsableError::Parse(anyhow!(e)))?;

        // TODO: Add validation to ensure sections are correct.

        Ok(Manifest(doc))
    }
}

/* ------------------------------- Impl: Hash ------------------------------- */

impl std::hash::Hash for Manifest {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut deps = self.into_iter().collect::<Vec<_>>();
        deps.sort();

        deps.iter().for_each(|(q, d)| {
            q.hash(state);
            d.hash(state);
        });
    }
}

/* ----------------------------- Impl: PartialEq ---------------------------- */

impl Eq for Manifest {}

impl PartialEq for Manifest {
    fn eq(&self, other: &Self) -> bool {
        let deps_self = self
            .into_iter()
            .map(|(_, d)| d)
            .collect::<HashSet<Dependency>>();
        let deps_other = other
            .into_iter()
            .map(|(_, d)| d)
            .collect::<HashSet<Dependency>>();

        deps_self == deps_other
    }
}

/* --------------------------- Impl: Into<String> --------------------------- */

impl From<&Manifest> for String {
    fn from(value: &Manifest) -> Self {
        value.0.to_string()
    }
}

/* -------------------------- Impl: From<Document> -------------------------- */

impl From<Document> for Manifest {
    fn from(value: Document) -> Self {
        Manifest(value)
    }
}

/* ------------------------------ Impl: Default ----------------------------- */

impl Default for Manifest {
    fn default() -> Self {
        Manifest(Document::new())
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Enum: Error                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not determine addon name")]
    MissingName,
    #[error("missing target")]
    MissingTarget,
    #[error("duplicate addon found {0}: {1:?}")]
    Duplicate(String, Vec<Query>),
    #[error("duplicate replacement of addon found between targets {0}: {1:?}")]
    DoubleReplace(String, Vec<String>),
    #[error("cannot specify replacement without a target: {0}")]
    InvalidReplace(String),
}

/* -------------------------------------------------------------------------- */
/*                                 Mod: Tests                                 */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {}
