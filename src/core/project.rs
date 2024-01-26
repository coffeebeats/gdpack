use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use toml_edit::de::ValueDeserializer;
use toml_edit::Item;
use typed_builder::TypedBuilder;
use walkdir::DirEntry;
use walkdir::WalkDir;

/* -------------------------------------------------------------------------- */
/*                                 Enum: Error                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("insecure path (escapes addon): {0:?}")]
    Insecure(PathBuf),
    #[error("invalid path: {0}")]
    Invalid(PathBuf),
    #[error("directory not found: {0}")]
    MissingDir(PathBuf),
    #[error(transparent)]
    NotRelative(std::path::StripPrefixError),
}

/* -------------------------------------------------------------------------- */
/*                           Struct: ScriptTemplates                          */
/* -------------------------------------------------------------------------- */

/// `ScriptTemplates` defines project configuration pertaining to the GDScript
/// templates used during development.
#[derive(Clone, Debug, Default, Eq, Deserialize, PartialEq, Serialize, TypedBuilder)]
pub struct ScriptTemplates {
    #[builder(default)]
    #[serde(default, rename = "include_script_templates")]
    pub include: Vec<PathBuf>,
    #[builder(default)]
    #[serde(default, rename = "export_script_templates")]
    pub export: Vec<PathBuf>,
}

/* -------------------------- Impl: ScriptTemplates ------------------------- */

impl ScriptTemplates {
    /* --------------------------- Methods: Public -------------------------- */

    // `find_scripts_in_dir` returns a list of paths, relative the the provided
    // `path`, which point to non-imported script templates.
    pub fn find_scripts_in_dir(path: impl AsRef<Path>) -> Result<Vec<PathBuf>, Error> {
        let path = path.as_ref();
        if !path.is_dir() {
            return Err(Error::MissingDir(path.to_owned()));
        }

        let templates = ScriptTemplateScan::builder()
            .path(path)
            .skip_imported(true)
            .skip_nonimported(false)
            .build()
            .into_iter()
            .collect();

        Ok(templates)
    }

    /// `included_from` returns a set of script templates, relative to the
    /// provided path, which should be installed into a Godot project.
    pub fn included_from(&self, path: impl AsRef<Path>) -> Result<HashSet<PathBuf>, Error> {
        let path = path.as_ref();

        let mut out = HashSet::new();

        for pattern in &self.include {
            let path = if pattern.is_absolute() {
                pattern.to_owned()
            } else {
                path.join(pattern)
            };

            let path = path
                .canonicalize()
                .map_err(|_| Error::Invalid(pattern.into()))?;

            let templates = ScriptTemplates::find_scripts_in_dir(path)?;
            out.extend(templates);
        }

        Ok(out)
    }

    /// `exported_from` returns a set of script templates, relative to the
    /// provided path, which should be installed into a Godot project that
    /// depends on an addon providing these [`ScriptTemplates`].
    ///
    /// NOTE: Addons are only permitted to export script templates which are
    /// located underneath the addon's root directory (i.e. where its installed
    /// from). This is to prevent a malicious addon from improperly accessing
    /// a user's file system.
    pub fn exported_from(&self, path: impl AsRef<Path>) -> Result<HashSet<PathBuf>, Error> {
        let path = path.as_ref();

        let mut out = HashSet::new();

        for pattern in &self.export {
            let root = path;

            let path = if pattern.is_absolute() {
                pattern.clone()
            } else {
                path.join(pattern)
            };

            let path = path.canonicalize().map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => Error::MissingDir(pattern.into()),
                _ => Error::Insecure(pattern.into()),
            })?;

            // NOTE: Any exported script templates must be defined within the
            // same directory that the search is rooted from (typically the
            // addon's directory).
            if let Err(_) = path.strip_prefix(root) {
                return Err(Error::Insecure(pattern.into()));
            }

            let templates = ScriptTemplates::find_scripts_in_dir(path)?;
            out.extend(templates);
        }

        // NOTE: Return an error if any returned paths are symlinks pointing to
        // locations outside of the provided directory.
        for p in &out {
            if !p.canonicalize().is_ok_and(|p| p.strip_prefix(path).is_ok()) {
                return Err(Error::Insecure(p.to_owned()));
            }
        }

        Ok(out)
    }
}

/* -------------------------- Impl: TryFrom<&Item> -------------------------- */

impl TryFrom<&Item> for ScriptTemplates {
    type Error = toml_edit::de::Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        value
            .to_string()
            .trim()
            .parse::<ValueDeserializer>()
            .and_then(ScriptTemplates::deserialize)
    }
}

/* -------------------------------------------------------------------------- */
/*                         Struct: ScriptTemplateScan                         */
/* -------------------------------------------------------------------------- */

/// `ScriptTemplateScan` defines a scan of a directory for GDScript templates
/// files. Different parameters can be set to customize the results of the
/// query.
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct ScriptTemplateScan {
    #[builder(setter(into))]
    pub path: PathBuf,

    #[builder(default = true)]
    pub contents_first: bool,
    #[builder(default = false)]
    pub skip_nonimported: bool,
    #[builder(default = false)]
    pub skip_imported: bool,
}

/* --------------------------- Impl: IntoIterator --------------------------- */

impl IntoIterator for ScriptTemplateScan {
    type Item = PathBuf;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let script_templates = WalkDir::new(&self.path)
            .min_depth(1)
            .follow_root_links(true)
            .follow_links(true)
            .contents_first(self.contents_first)
            .sort_by_file_name()
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|s| s == "gd"))
            .filter(|entry| {
                let is_imported = entry
                    .path()
                    .file_stem()
                    .and_then(OsStr::to_str)
                    .is_some_and(|s| s.starts_with("gdpack__"));

                if is_imported {
                    !self.skip_imported
                } else {
                    !self.skip_nonimported
                }
            })
            .map(DirEntry::into_path)
            .collect::<Vec<_>>();

        script_templates.into_iter()
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Mod: Tests                                 */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use rstest::fixture;
    use rstest::rstest;
    use std::collections::HashSet;
    use std::path::PathBuf;
    use tempfile::TempDir;

    use super::Error;
    use super::ScriptTemplates;

    /* ---------------------- Test: find_scripts_in_dir --------------------- */

    macro_rules! write_file {
        ($input:expr) => {
            let p = PathBuf::from($input);
            std::fs::create_dir_all(p.as_path().parent().unwrap()).unwrap();
            std::fs::File::create(p).unwrap();
        };
    }

    #[fixture]
    fn once_tmp() -> TempDir {
        // Given: A temporary directory to create test files in.
        tempfile::tempdir().unwrap()
    }

    #[rstest]
    fn test_find_scripts_in_dir_handle_invalid_inputs(once_tmp: TempDir) {
        assert_eq!(
            ScriptTemplates::find_scripts_in_dir(PathBuf::from("")),
            Err(Error::MissingDir("".into()))
        );
        assert_eq!(
            ScriptTemplates::find_scripts_in_dir(PathBuf::from("missing")),
            Err(Error::MissingDir("missing".into()))
        );

        let path = once_tmp.path().join("a.txt");
        write_file!(&path);
        assert_eq!(
            ScriptTemplates::find_scripts_in_dir(&path),
            Err(Error::MissingDir(path))
        );
    }

    #[rstest]
    fn test_find_scripts_in_empty_dir_returns_empty_vec(
        once_tmp: TempDir,
        #[values(".", "./a", "./a/b/c")] path_search: PathBuf,
    ) {
        // Given: A path to search for scripts in.
        let path = once_tmp.path().join(&path_search);

        // When: An empty root directory is searched.
        let got = ScriptTemplates::find_scripts_in_dir(&path);

        assert_eq!(
            got,
            if path_search.to_str() == Some(".") {
                Ok(vec![])
            } else {
                Err(Error::MissingDir(path))
            }
        )
    }

    #[rstest]
    fn test_find_scripts_in_dir_returns_vec_with_correct_files(once_tmp: TempDir) {
        // Given: A path to search for scripts in.
        let path = once_tmp.path();

        // Given: A set of template scripts mixed in with other files.
        write_file!(path.join("./a.gd"));
        write_file!(path.join("./a/b.gd"));
        write_file!(path.join("./a/gdpack__b.gd")); // ignored
        write_file!(path.join("./a/b/c.gd"));
        write_file!(path.join("./a/b/c.txt")); // ignored

        // When: An empty root directory is searched.
        let got = ScriptTemplates::find_scripts_in_dir(&path);

        assert_eq!(
            got,
            Ok(vec!["a/b/c.gd", "a/b.gd", "a.gd",]
                .into_iter()
                .map(PathBuf::from)
                .map(|p| path.join(p))
                .collect())
        )
    }

    /* ------------------------- Test: exported_from ------------------------ */

    #[rstest]
    #[case(|p: PathBuf| p.join("root"), |_: PathBuf| Ok(HashSet::default()))]
    #[case(|p: PathBuf| p, |p: PathBuf| Err(Error::Insecure(p)))]
    #[case(|p: PathBuf| p.join("root/.."), |p: PathBuf| Err(Error::Insecure(p.join("root/.."))))]
    #[case(|p: PathBuf| p.join("missing"), |p: PathBuf| Err(Error::MissingDir(p.join("missing"))))]
    // NOTE: On macos, the temporary directory is a symlink to `/private/...`,
    // meaning the case below will fail.
    #[case(|p: PathBuf| p.join("root"), |p: PathBuf| if cfg!(target_os = "macos") { Err(Error::Insecure(p.join("root"))) } else { Ok(HashSet::default())})]
    fn test_exported_from_prevents_insecure_patterns(
        once_tmp: TempDir,
        #[case] pattern: fn(PathBuf) -> PathBuf,
        #[case] result_fn: fn(PathBuf) -> Result<HashSet<PathBuf>, Error>,
    ) {
        // Given: A temporary directory to write test files to.
        let path_tmp = once_tmp.path().to_owned();

        // Given: A path to search for scripts in.
        let path_root = once_tmp.path().join("root");
        std::fs::create_dir(&path_root).unwrap();

        // Given: A [`ScriptTemplates`] with the exported pattern.
        let templates = ScriptTemplates::builder()
            .export(vec![pattern(path_tmp.clone())])
            .build();

        // When: The exported script template paths are collected.
        let got = templates.exported_from(&path_root);

        // Then: The results match expectations.
        assert_eq!(got, result_fn(path_tmp));
    }

    #[rstest]
    fn test_exported_from_prevents_insecure_template(once_tmp: TempDir) {
        // Given: A path to search for scripts in.
        let path = once_tmp.path().join("a");
        std::fs::create_dir_all(&path).unwrap();

        // Given: A [`ScriptTemplates`] with the exported pattern.
        let templates = ScriptTemplates::builder()
            .export(vec![path.clone()])
            .build();

        // Given: A file outside of the search path.
        let path_ext = path.parent().unwrap().join("secret.txt");
        write_file!(&path_ext);

        // Given: An included symlink to the external file.
        let path_file = path.join("template.gd");
        std::os::unix::fs::symlink(&path_ext, &path_file).unwrap();

        // When: The exported script template paths are collected.
        let got = templates.exported_from(path);

        // Then: An error is returned warning about the insecure path.
        assert_eq!(got, Err(Error::Insecure(path_file)));
    }
}
