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

[project]
# Merge additional script templates from these directories into the project.
include_script_templates = []

# Export non-imported script templates found in these directories.
export_script_templates = []

[dev-addons]
Gut = { git = "https://github.com/bitwes/Gut.git", tag = "v9.1.1" }
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
    /// NOTE: There are a few invariants upheld when gathering dependencies
    /// within a manifest. These are as follows:
    ///     1. The same addon cannot be specified 2+ times. However, a target-
    ///        specified addon may override the value declared in the default
    ///        target.
    ///     2. The same addon cannot be replaced by 2+ addons. Note that because
    ///        replacements can only be specified within a target, any collision
    ///        is guaranteed to be an invalid state.
    pub fn dependencies<'a>(
        &self,
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

            out.extend(self.addons(&query).into_iter().map(|d| (query.clone(), d)));
        }

        Manifest::check_for_duplicate(&out)?;
        Manifest::check_for_double_replace(&out)?;

        Ok(out.into_iter().map(|(_, d)| d).collect())
    }

    /// Returns an _immutable_ view of the project configuration within the
    /// [`Manifest`].
    pub fn project(&self) -> Project {
        Project::builder().document(&self.0).build()
    }

    /* -------------------------- Methods: Private -------------------------- */

    /// `check_for_duplicate` validates that the provided [`Dependency`] list
    /// does not contain duplicate sepcifications of an [`crate::core::Addon`].
    fn check_for_duplicate(deps: &[(Query, Dependency)]) -> Result<(), Error> {
        // Map from addon name to the target which specified it.
        let mut declared: HashMap<String, Option<&str>> = HashMap::new();

        for (query, dep) in deps {
            let name = dep
                .addon
                .as_ref()
                .map(String::to_owned)
                .ok_or(Error::MissingName)?;

            let target = query.target.as_deref();

            // Insert the addon as-is the first time it's encountered.
            if !declared.contains_key(&name) {
                declared.insert(name.to_owned(), target);
                continue;
            }

            match target {
                // Skip the default target because a specified target
                // declares this addon as a dependency.
                None => continue,
                Some(t) => match declared.remove(&name).unwrap() {
                    // Override the default target because this target
                    // declares this addon as a dependency.
                    None => declared.insert(name.to_owned(), Some(t)),
                    Some(t_duplicate) => {
                        return Err(Error::Duplicate(
                            name,
                            vec![t.to_owned(), t_duplicate.to_owned()],
                        ));
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
    #[error("duplicate addon found between targets {0}: {1:?}")]
    Duplicate(String, Vec<String>),
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
