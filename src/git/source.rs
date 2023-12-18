use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;
use url::Url;

/* -------------------------------------------------------------------------- */
/*                               Struct: Source                               */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
pub struct Source {
    #[serde(flatten)]
    pub(super) commit: Reference,
    pub(super) repo: Url,
}

/* ------------------------------ Impl: Source ------------------------------ */

impl Source {
    pub fn new(repo: Url, commit: Reference) -> Source {
        Source { commit, repo }
    }
}

/* -------------------------------------------------------------------------- */
/*                               Enum: Reference                              */
/* -------------------------------------------------------------------------- */

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
    pub fn rev(&self) -> String {
        match self {
            Reference::Default => "refs/remotes/origin/HEAD".to_owned(),
            Reference::Branch(b) => format!("refs/remotes/origin/{0}", b),
            Reference::Tag(t) => t.to_owned(),
            Reference::Rev(r) => r.to_owned(),
        }
    }
}

/* ------------------------------ Impl: Display ----------------------------- */

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reference::Default => f.write_str("HEAD"),
            Reference::Branch(b) => f.write_str(&format!("branch={}", b)),
            Reference::Rev(r) => f.write_str(&format!("rev={}", r)),
            Reference::Tag(t) => f.write_str(&format!("tag={}", t)),
        }
    }
}
