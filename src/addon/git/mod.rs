use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;
use url::Url;

/* -------------------------------------------------------------------------- */
/*                                Struct: GitSource                                */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
pub struct Spec {
    #[serde(flatten)]
    pub commit: GitReference,
    pub repo: Url,
}

/* ------------------------------- Impl: Spec ------------------------------- */

impl Spec {
    pub fn new(repo: Url, commit: GitReference) -> Spec {
        Spec { commit, repo }
    }
}

/* --------------------------- Enum: GitReference --------------------------- */

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum GitReference {
    Default,
    Branch(String),
    Rev(String),
    Tag(String),
}

/* --------------------------- Impl: GitReference --------------------------- */

impl GitReference {
    pub fn rev(&self) -> String {
        match self {
            GitReference::Default => "refs/remotes/origin/HEAD".to_owned(),
            GitReference::Branch(b) => format!("refs/remotes/origin/{0}", b),
            GitReference::Tag(t) => t.to_owned(),
            GitReference::Rev(r) => r.to_owned(),
        }
    }
}

/* ------------------------------ Impl: Display ----------------------------- */

impl std::fmt::Display for GitReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitReference::Default => f.write_str("HEAD"),
            GitReference::Branch(b) => f.write_str(&format!("branch={}", b)),
            GitReference::Rev(r) => f.write_str(&format!("rev={}", r)),
            GitReference::Tag(t) => f.write_str(&format!("tag={}", t)),
        }
    }
}
