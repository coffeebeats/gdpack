/* ------------------------------- Mod: gdext ------------------------------- */

pub mod gdext;

/* ------------------------------ Mod: manifest ----------------------------- */

pub mod manifest;

pub use manifest::Manifest;

/* ------------------------------- Mod: plugin ------------------------------ */

pub mod plugin;

/* -------------------------------------------------------------------------- */
/*                            Trait: Configuration                            */
/* -------------------------------------------------------------------------- */

pub trait Configuration {
    fn matches(path: impl AsRef<Path>) -> bool;
}

/* -------------------------------------------------------------------------- */
/*                               Trait: Parsable                              */
/* -------------------------------------------------------------------------- */

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

        let contents = std::fs::read_to_string(path).map_err(ParsableError::IO)?;

        Self::parse(&contents)
    }
}

/* -------------------------- Error: ParsableError -------------------------- */

#[derive(Debug, thiserror::Error)]
pub enum ParsableError {
    #[error(transparent)]
    IO(std::io::Error),
    #[error("invalid filename: {0}")]
    InvalidName(String),
    #[error(transparent)]
    Parse(anyhow::Error),
}

/* -------------------------------------------------------------------------- */
/*                             Trait: Persistable                             */
/* -------------------------------------------------------------------------- */

use std::fs::File;
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

            File::create(path)
                .and_then(|mut f| f.write_all(contents.as_bytes()))
                .map_err(PersistableError::IO)
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
    IO(std::io::Error),
    #[error("invalid filepath: {0}")]
    InvalidPath(String),
    #[error(transparent)]
    Serialize(anyhow::Error),
}

/* -------------------------------------------------------------------------- */
/*                               Trait: Findable                              */
/* -------------------------------------------------------------------------- */

use walkdir::WalkDir;

pub trait Findable
where
    Self: Configuration + Parsable,
{
    fn find_in_ancestors(path: impl AsRef<Path>) -> Result<Self, FindableError> {
        let mut path = path.as_ref();

        let mut paths: Vec<PathBuf> = vec![];

        if path.is_file() {
            paths.push(path.to_owned());
            path = path.parent().unwrap() // This is safe because it's not empty.
        }

        // Beginning with the nearest directory of the path that was passed in,
        // attempt to find the configuration in that directory or any ancestor
        // directory.
        loop {
            if path.exists() {
                paths.push(path.to_owned());
            }

            match path.parent() {
                None => return Self::find_in_paths(paths.iter()),
                Some(p) => {
                    path = p;
                }
            }
        }
    }

    fn find_in_folder(path: impl AsRef<Path>) -> Result<Self, FindableError> {
        let mut path = path.as_ref();

        if path.is_file() {
            // Shortcut the directory walk if the passed-in path is a match.
            if Self::matches(path) {
                return Self::parse_file(path).map_err(FindableError::Parse);
            }

            path = path.parent().unwrap();
        }

        match WalkDir::new(path)
            .follow_root_links(true)
            .follow_links(true)
            .max_depth(1) // Only look at files directly in the folder.
            .min_depth(1) // Skip the walked directory itself.
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().is_file()) // Skip directories.
            .take_while(|e| Self::matches(e.path()))
            .next()
        {
            None => Err(FindableError::NotFound),
            Some(e) => Self::parse_file(e.path()).map_err(FindableError::Parse),
        }
    }

    fn find_in_paths(paths: impl Iterator<Item = impl AsRef<Path>>) -> Result<Self, FindableError> {
        paths
            .filter_map(|p| match Self::find_in_folder(p) {
                Err(FindableError::NotFound) => None,
                r => Some(r),
            })
            .next()
            .unwrap_or(Err(FindableError::NotFound))
    }
}

/* --------------------------- Enum: FindableError -------------------------- */

#[derive(Debug, thiserror::Error)]
pub enum FindableError {
    #[allow(dead_code)]
    #[error("not found")]
    NotFound,
    #[allow(dead_code)]
    #[error(transparent)]
    Parse(ParsableError),
}
