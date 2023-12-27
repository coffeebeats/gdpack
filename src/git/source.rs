use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;
use url::Host;
use url::Url;

/* -------------------------------------------------------------------------- */
/*                               Struct: Remote                               */
/* -------------------------------------------------------------------------- */

/// Remote is a newtype wrapper around [Url] which adds helpful methods for
/// extracting parts of a remotely-hosted git repository.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Remote(Url);

/* ------------------------------ Impl: Remote ------------------------------ */

impl Remote {
    /// Extracts and returns the host of the remote repository [Url].
    pub fn host(&self) -> Option<String> {
        self.0.host().as_ref().map(Host::<&str>::to_string)
    }

    /// Extracts and returns the owner of the remote repository.
    pub fn owner(&self) -> Option<String> {
        self.0
            .path()
            .trim_matches('/')
            .split("/")
            .next()
            .map(&str::to_owned)
    }

    /// Extracts and returns the name of the remote repository.
    pub fn name(&self) -> Option<String> {
        self.0
            .path()
            .trim_matches('/')
            .split("/")
            .skip(1)
            .next()
            .and_then(|s| s.strip_suffix(".git").or(Some(s)))
            .map(&str::to_owned)
    }

    /// Returns a reference to the underlying [Url].
    pub fn url(&self) -> &Url {
        &self.0
    }
}

/* ------------------------------ Impl: Display ----------------------------- */

impl std::fmt::Display for Remote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

/* ----------------------------- Impl: From<Url> ---------------------------- */

impl From<Url> for Remote {
    fn from(value: Url) -> Self {
        Remote(value)
    }
}

/* -------------------------------------------------------------------------- */
/*                               Struct: Source                               */
/* -------------------------------------------------------------------------- */

/// Captures a specific version of a remotely hosted git repository.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
pub struct Source {
    #[serde(flatten)]
    pub reference: Reference,
    pub repo: Remote,
}

/* -------------------------------------------------------------------------- */
/*                               Enum: Reference                              */
/* -------------------------------------------------------------------------- */

/// Specifies a particular revision in a git repository.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum Reference {
    Default,
    Branch(String),
    Rev(String),
    Tag(String),
}

/* ----------------------------- Impl: Reference ---------------------------- */

impl Reference {
    /// Returns a list of git [refspecs](https://git-scm.com/book/en/v2/Git-Internals-The-Refspec)
    /// to fetch when checking out a specific git [Reference].
    ///
    /// NOTE: This implementation is more or less copied from [Cargo's implementation](https://github.com/rust-lang/cargo/blob/rust-1.76.0/src/cargo/sources/git/utils.rs#L968-L1006).
    pub fn refspecs(&self) -> Vec<String> {
        match self {
            Reference::Default => vec![String::from("+HEAD:refs/remotes/origin/HEAD")],
            Reference::Branch(b) => vec![format!("+refs/heads/{0}:refs/remotes/origin/{0}", b)],
            Reference::Tag(t) => vec![format!("+refs/tags/{0}:refs/remotes/origin/tags/{0}", t)],
            Reference::Rev(r) => {
                if r.starts_with("refs/") {
                    vec![format!("+{0}:{0}", r)]
                } else if is_commit_hash_like(r) {
                    vec![format!("+{0}:refs/commit/{0}", r)]
                } else {
                    // Just fetch everything and hope it's found.
                    vec![
                        String::from("+refs/heads/*:refs/remotes/origin/*"),
                        String::from("+HEAD:refs/remotes/origin/HEAD"),
                    ]
                }
            }
        }
    }
}

/* ------------------------------ Impl: Display ----------------------------- */

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reference::Default => f.write_str("HEAD"),
            Reference::Branch(b) => f.write_str(b),
            Reference::Rev(r) => f.write_str(r),
            Reference::Tag(t) => f.write_str(t),
        }
    }
}

/* ---------------------- Function: is_commit_hash_like --------------------- */

fn is_commit_hash_like(id: &str) -> bool {
    id.len() >= 7 && id.chars().all(|ch| ch.is_ascii_hexdigit())
}
