/* ------------------------------- Mod: gdext ------------------------------- */

mod gdext;

/* ------------------------------ Mod: manifest ----------------------------- */

mod manifest;

/* ------------------------------- Mod: plugin ------------------------------ */

mod plugin;

/* -------------------------------------------------------------------------- */
/*                             Trait: Configurable                            */
/* -------------------------------------------------------------------------- */

pub trait Configurable {
    /// `file_ext` is the expected extension of the configuration file.
    fn file_ext<'a>() -> Option<&'a str>;

    /// `file_name` is the expected name of the configuration file.
    fn file_name<'a>() -> Option<&'a str>;
}

/* -------------------------------------------------------------------------- */
/*                               Trait: Parsable                              */
/* -------------------------------------------------------------------------- */

use std::path::Path;

pub trait Parsable
where
    Self: Configurable + Default + Sized,
{
    fn parse(contents: &str) -> Result<Self, ParsableError>;

    fn parse_file(path: impl AsRef<Path>) -> Result<Self, ParsableError> {
        let path = path.as_ref();

        if let Some(want) = <Self as Configurable>::file_ext() {
            let file_ext = path.extension().and_then(|s| s.to_str());
            if !file_ext.is_some_and(|s| s == want.strip_prefix('.').unwrap_or(want)) {
                return Err(ParsableError::InvalidName(
                    file_ext.map(str::to_owned).unwrap_or(String::default()),
                ));
            }
        }

        if let Some(want) = <Self as Configurable>::file_name() {
            let file_name = path.file_name().and_then(|s| s.to_str());
            if !file_name.is_some_and(|s| s == want) {
                return Err(ParsableError::InvalidName(
                    file_name.map(str::to_owned).unwrap_or(String::default()),
                ));
            }
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
