use url::Url;

/* -------------------------------------------------------------------------- */
/*                                Struct: Spec                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug, serde::Deserialize, PartialEq, Eq, serde::Serialize)]
pub struct Spec {
    #[serde(flatten)]
    pub commit: Commit,
    pub repo: Url,
}

/* ------------------------------- Impl: Spec ------------------------------- */

impl Spec {
    pub fn new(repo: Url, commit: Commit) -> Spec {
        Spec {
            commit: commit,
            repo: repo,
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                Enum: Commit                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug, serde::Deserialize, PartialEq, Eq, serde::Serialize)]
#[serde(untagged)]
pub enum Commit {
    Default,
    Branch(String),
    Rev(String),
    Tag(String),
}
