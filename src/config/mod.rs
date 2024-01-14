/* ------------------------------- Mod: gdext ------------------------------- */

pub mod gdext;

/* ------------------------------ Mod: manifest ----------------------------- */

pub mod manifest;

pub use manifest::Manifest;
use typed_builder::TypedBuilder;

/* ------------------------------- Mod: plugin ------------------------------ */

pub mod plugin;

/* -------------------------------------------------------------------------- */
/*                            Trait: Configuration                            */
/* -------------------------------------------------------------------------- */

pub trait Configuration {
    fn matches(path: impl AsRef<Path>) -> bool;

    fn file_name<'a>() -> Option<&'a str> {
        None
    }
}

/* -------------------------------------------------------------------------- */
/*                               Trait: Parsable                              */
/* -------------------------------------------------------------------------- */

use std::marker::PhantomData;
use std::path::Path;

pub trait Parsable
where
    Self: Configuration + Default + Sized,
{
    fn parse(contents: &str) -> Result<Self, ParsableError>;

    fn parse_file(path: impl AsRef<Path>) -> Result<Self, ParsableError> {
        let path = path.as_ref();

        if !<Self as Configuration>::matches(path) {
            return Err(ParsableError::InvalidName(
                path.to_str()
                    .map(str::to_owned)
                    .unwrap_or(String::default()),
            ));
        }

        let contents = std::fs::read_to_string(path).map_err(ParsableError::Io)?;

        Self::parse(&contents)
    }
}

/* -------------------------- Error: ParsableError -------------------------- */

#[derive(Debug, thiserror::Error)]
pub enum ParsableError {
    #[error(transparent)]
    Io(std::io::Error),
    #[error("invalid filename: {0}")]
    InvalidName(String),
    #[error(transparent)]
    Parse(anyhow::Error),
}

/* -------------------------------------------------------------------------- */
/*                             Trait: Persistable                             */
/* -------------------------------------------------------------------------- */

use std::io::Write;
use std::path::PathBuf;

pub trait Persistable<'a>
where
    Self: Configuration + 'a,
    &'a Self: TryInto<String>,
    <&'a Self as TryInto<String>>::Error: Into<anyhow::Error>,
{
    fn persist<'b>(&'b self, path: impl AsRef<Path>) -> Result<(), PersistableError>
    where
        'b: 'a,
    {
        {
            let path = path.as_ref();

            if path.exists() && !path.is_file() {
                return Err(PersistableError::InvalidPath(
                    path.to_str()
                        .map(str::to_owned)
                        .unwrap_or(String::default()),
                ));
            }

            let contents: String = self
                .try_into()
                .map_err(|e| e.into())
                .map_err(PersistableError::Serialize)?;

            std::fs::File::create(path)
                .and_then(|mut f| f.write_all(contents.as_bytes()))
                .map_err(PersistableError::Io)
        }
    }
}

/* ---------------------------- Impl: Persistable --------------------------- */

impl<'a, T> Persistable<'a> for T
where
    T: Configuration + 'a,
    &'a T: TryInto<String>,
    <&'a T as TryInto<String>>::Error: Into<anyhow::Error>,
{
}

/* ------------------------- Error: PersistableError ------------------------ */

#[derive(Debug, thiserror::Error)]
pub enum PersistableError {
    #[error(transparent)]
    Io(std::io::Error),
    #[error("invalid filepath: {0}")]
    InvalidPath(String),
    #[error(transparent)]
    Serialize(anyhow::Error),
}

/* -------------------------------------------------------------------------- */
/*                               Trait: Findable                              */
/* -------------------------------------------------------------------------- */

use walkdir::WalkDir;

#[derive(Debug, TypedBuilder)]
pub struct FileQuery<T>
where
    T: Configuration + Parsable,
{
    #[builder(setter(into))]
    path: PathBuf,

    #[builder(default = 1, setter(into))]
    min_depth: usize,

    #[builder(default = std::usize::MAX, setter(into))]
    max_depth: usize,

    #[builder(default)]
    _config: PhantomData<T>,
}

impl<T> IntoIterator for FileQuery<T>
where
    T: Configuration + Parsable,
{
    type Item = (PathBuf, Result<T, ParsableError>);

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            WalkDir::new(self.path)
                .follow_root_links(true)
                .follow_links(true)
                .max_depth(self.max_depth) // Only look at files directly in the folder.
                .min_depth(self.min_depth) // Skip the walked directory itself.
                .into_iter()
                .filter_map(Result::ok) // Skip unreadable or invalid files.
                .filter(|e| e.path().is_file()) // Skip directories.
                .filter_map(|e| match T::matches(e.path()) {
                    false => None,
                    true => Some((e.path().to_owned(), T::parse_file(e.path()))),
                }),
        )
    }
}
