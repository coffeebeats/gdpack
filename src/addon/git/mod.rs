use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;
use url::Url;

/* -------------------------------------------------------------------------- */
/*                                Struct: Spec                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
pub struct Spec {
    #[serde(flatten)]
    pub commit: Commit,
    pub repo: Url,
}

/* ------------------------------- Impl: Spec ------------------------------- */

impl Spec {
    pub fn new(repo: Url, commit: Commit) -> Spec {
        Spec { commit, repo }
    }
}

/* -------------------------------------------------------------------------- */
/*                                Enum: Commit                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum Commit {
    Default,
    Branch(String),
    Rev(String),
    Tag(String),
}
