use serde::Deserialize;
use serde::Serialize;
use toml_edit::de::ValueDeserializer;
use toml_edit::ser::ValueSerializer;
use toml_edit::Document;
use typed_builder::TypedBuilder;

use super::key::Key;
use super::Dependency;
use super::Query;

/* -------------------------------------------------------------------------- */
/*                               Struct: Addons                               */
/* -------------------------------------------------------------------------- */

/// [`Addons`] is used to immutably view the collection of addons within the
/// provided [`toml_edit::Document`] for the specified [`Query`].
#[derive(Debug, TypedBuilder)]
pub struct Addons<'a> {
    document: &'a Document,
    query: Query,
}

/* ------------------------------ Impl: Addons ------------------------------ */

impl<'a> Addons<'a> {
    /* --------------------------- Methods: Public -------------------------- */

    /// Immutably look up the [`Dependency`] with the specified `name`.
    pub fn get(&self, name: &str) -> Option<Dependency> {
        let key = Key::builder().query(&self.query).name(name).build();

        key.get(&self.document).and_then(|t| {
            t.to_string()
                .parse::<ValueDeserializer>()
                .and_then(Dependency::deserialize)
                .ok()
        })
    }
}

/* --------------------------- Impl: IntoIterator --------------------------- */

impl<'a> IntoIterator for Addons<'a> {
    type Item = Dependency;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.query
            .get(self.document)
            .and_then(|v| v.as_table_like())
            .map(|t| t.iter())
            .unwrap_or(Box::new(std::iter::empty()))
            .filter_map(|(_, v)| {
                v.to_string()
                    .parse::<ValueDeserializer>()
                    .and_then(Dependency::deserialize)
                    .ok()
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

/* -------------------------------------------------------------------------- */
/*                              Struct: AddonsMut                             */
/* -------------------------------------------------------------------------- */

/// [`AddonsMut`] is used to manage the collection of addons within the provided
/// [`toml_edit::Document`] for the specified [`Query`].
#[derive(Debug, TypedBuilder)]
pub struct AddonsMut<'a> {
    document: &'a mut Document,
    query: Query,
}

/* ----------------------------- Impl: AddonsMut ---------------------------- */

impl<'a> AddonsMut<'a> {
    /* --------------------------- Methods: Public -------------------------- */

    /// Immutably look up the [`Dependency`] with the specified `name`.
    pub fn get(&self, name: &str) -> Option<Dependency> {
        Addons::builder()
            .document(&self.document)
            .query(self.query.clone())
            .build()
            .get(name)
    }

    /// Insert a new [`Dependency`] with the specified `name`; overrides any
    /// existing values and returns the previously stored value, if any.
    pub fn insert(&mut self, name: &str, dep: &Dependency) -> Option<Dependency> {
        let key = Key::builder().query(&self.query).name(name).build();

        let prev = dep
            .serialize(ValueSerializer::new())
            .ok()
            .and_then(|v| key.insert(&mut self.document, toml_edit::value(v)))?;

        prev.to_string()
            .parse::<ValueDeserializer>()
            .and_then(Dependency::deserialize)
            .ok()
    }

    /// Remove the [`Dependency`] with the specified `name`; returns the
    /// previously stored value, if any.
    pub fn remove(&mut self, name: &str) -> Option<Dependency> {
        let key = Key::builder().query(&self.query).name(name).build();

        let out = key
            .remove(&mut self.document)
            .map(|v| v.to_string())
            .and_then(|s| s.parse::<ValueDeserializer>().ok())
            .and_then(|de| Dependency::deserialize(de).ok());

        if key.query.is_empty(&self.document) {
            key.query.remove(&mut self.document);
        }

        out
    }
}

/* --------------------------- Impl: IntoIterator --------------------------- */

impl<'a> IntoIterator for AddonsMut<'a> {
    type Item = Dependency;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.query
            .get(self.document)
            .and_then(|v| v.as_table_like())
            .map(|t| t.iter())
            .unwrap_or(Box::new(std::iter::empty()))
            .filter_map(|(_, v)| {
                v.to_string()
                    .parse::<ValueDeserializer>()
                    .and_then(Dependency::deserialize)
                    .ok()
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Mod: Tests                                 */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde::Serialize;
    use std::path::PathBuf;
    use toml_edit::de::ValueDeserializer;
    use toml_edit::ser::ValueSerializer;

    use crate::config::manifest::Dependency;
    use crate::config::manifest::Manifest;
    use crate::config::manifest::Query;
    use crate::config::manifest::Source;
    use crate::config::Parsable;
    use crate::git;

    /* ---------------------------- Test: Addons ---------------------------- */

    #[test]
    fn test_addons_insert() {
        let mut got = Manifest::default();

        let prev = got.addons_mut(Query::default()).insert(
            "abc",
            &Dependency::builder().source(PathBuf::from("a/b/c")).build(),
        );

        assert_eq!(
            String::from(&got),
            String::from(&Manifest::parse("[addons]\nabc = { path = \"a/b/c\" }").unwrap()),
        );
        assert_eq!(prev, None);
    }

    #[test]
    fn test_addons_remove() {
        let mut got = Manifest::default();

        got.addons_mut(Query::default()).insert(
            "abc",
            &Dependency::builder().source(PathBuf::from("a/b/c")).build(),
        );

        let prev = got.addons_mut(Query::default()).remove("abc");

        assert_eq!(String::from(&got), String::from(&Manifest::default()));
        assert_eq!(
            prev,
            Some(Dependency::builder().source(PathBuf::from("a/b/c")).build())
        );
    }

    /* ------------------------- Test: Serialization ------------------------ */

    macro_rules! test_ser_spec {
        ($name:ident, $input:expr, $want:expr$(,)?) => {
            #[test]
            fn $name() {
                assert_eq!(
                    $input
                        .serialize(ValueSerializer::new())
                        .map(|v| v.to_string()),
                    Ok($want.to_owned()),
                );
            }
        };
    }

    test_ser_spec!(
        test_ser_spec_local_path,
        PathBuf::from("a/b/c"),
        r#"{ path = "a/b/c" }"#
    );

    test_ser_spec!(
        test_ser_spec_repo_with_default_branch,
        Source::Git(
            git::Source::builder()
                .repo(
                    url::Url::parse("https://github.com")
                        .map(git::Remote::from)
                        .unwrap()
                )
                .build()
        ),
        r#"{ git = "https://github.com/" }"#,
    );

    test_ser_spec!(
        test_ser_spec_repo_with_branch,
        Source::Git(
            git::Source::builder()
                .reference(Some(git::Reference::Branch(String::from("branch"))))
                .repo(
                    url::Url::parse("https://github.com")
                        .map(git::Remote::from)
                        .unwrap()
                )
                .build()
        ),
        r#"{ branch = "branch", git = "https://github.com/" }"#,
    );

    test_ser_spec!(
        test_ser_spec_repo_with_tag,
        Source::Git(
            git::Source::builder()
                .reference(Some(git::Reference::Tag(String::from("tag"))))
                .repo(
                    url::Url::parse("https://github.com")
                        .map(git::Remote::from)
                        .unwrap()
                )
                .build()
        ),
        r#"{ tag = "tag", git = "https://github.com/" }"#,
    );

    test_ser_spec!(
        test_ser_spec_repo_with_rev,
        Source::Git(
            git::Source::builder()
                .reference(Some(git::Reference::Rev(String::from("rev"))))
                .repo(
                    url::Url::parse("https://github.com")
                        .map(git::Remote::from)
                        .unwrap()
                )
                .build()
        ),
        r#"{ rev = "rev", git = "https://github.com/" }"#
    );

    test_ser_spec!(
        test_ser_spec_repo_with_release,
        Source::Release(
            git::GitHubRelease::builder()
                .tag(String::from("tag"))
                .asset(String::from("asset"))
                .repo(
                    url::Url::parse("https://github.com")
                        .map(git::Remote::from)
                        .unwrap()
                )
                .build()
        ),
        r#"{ git = "https://github.com/", release = "tag", asset = "asset" }"#
    );

    #[test]
    fn test_dependency_serializes_with_attrs_to_table() -> Result<(), toml_edit::ser::Error> {
        let dep = Dependency::builder()
            .addon(Some(String::from("abc")))
            .replace(Some(String::from("def")))
            .source(PathBuf::from("a/b/c"))
            .build();

        assert_eq!(
            dep.serialize(ValueSerializer::new())?.to_string(),
            r#"{ name = "abc", replace = "def", path = "a/b/c" }"#
        );

        Ok(())
    }

    /* ------------------------ Test: Deserialization ----------------------- */

    macro_rules! test_de_spec {
        ($name:ident, $input:expr, $want:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    $input
                        .parse::<ValueDeserializer>()
                        .and_then(Source::deserialize),
                    Ok($want),
                );
            }
        };
    }

    test_de_spec!(
        test_de_spec_local_path,
        r#"{ path = "a/b/c" }"#,
        Source::from(PathBuf::from("a/b/c"))
    );

    test_de_spec!(
        test_de_spec_repo_with_default_branch,
        r#"{ git = "https://github.com" }"#,
        Source::Git(
            git::Source::builder()
                .repo(
                    url::Url::parse("https://github.com")
                        .map(git::Remote::from)
                        .unwrap()
                )
                .build()
        )
    );

    test_de_spec!(
        test_de_spec_repo_with_branch,
        r#"{ git = "https://github.com", branch = "branch" }"#,
        Source::Git(
            git::Source::builder()
                .repo(
                    url::Url::parse("https://github.com")
                        .map(git::Remote::from)
                        .unwrap()
                )
                .reference(Some(git::Reference::Branch(String::from("branch"))))
                .build()
        )
    );

    test_de_spec!(
        test_de_spec_repo_with_tag,
        r#"{ git = "https://github.com", tag = "tag" }"#,
        Source::Git(
            git::Source::builder()
                .repo(
                    url::Url::parse("https://github.com")
                        .map(git::Remote::from)
                        .unwrap()
                )
                .reference(Some(git::Reference::Tag(String::from("tag"))))
                .build()
        )
    );

    test_de_spec!(
        test_de_spec_repo_with_rev,
        r#"{ git = "https://github.com", rev = "rev" }"#,
        Source::Git(
            git::Source::builder()
                .repo(
                    url::Url::parse("https://github.com")
                        .map(git::Remote::from)
                        .unwrap()
                )
                .reference(Some(git::Reference::Rev(String::from("rev"))))
                .build()
        )
    );

    test_de_spec!(
        test_de_spec_repo_with_release,
        r#"{ git = "https://github.com", release = "tag", asset = "asset" }"#,
        Source::Release(
            git::GitHubRelease::builder()
                .repo(
                    url::Url::parse("https://github.com")
                        .map(git::Remote::from)
                        .unwrap()
                )
                .tag(String::from("tag"))
                .asset(String::from("asset"))
                .build()
        )
    );

    #[test]
    fn test_dependency_deserializes_with_attrs_to_table() -> Result<(), toml_edit::de::Error> {
        assert_eq!(
            r#"{ git = "https://github.com", name = "abc", replace = "def" }"#
                .parse::<ValueDeserializer>()
                .and_then(Dependency::deserialize)?,
            Dependency::builder()
                .addon(Some(String::from("abc")))
                .replace(Some(String::from("def")))
                .source(Source::Git(
                    git::Source::builder()
                        .repo(
                            url::Url::parse("https://github.com")
                                .map(git::Remote::from)
                                .unwrap()
                        )
                        .build()
                ))
                .build()
        );

        Ok(())
    }
}
