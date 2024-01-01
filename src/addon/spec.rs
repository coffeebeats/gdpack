use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

use crate::git;

/* -------------------------------------------------------------------------- */
/*                                 Enum: Spec                                 */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum Spec {
    Path(PathBuf),
    Git(git::Source),
    Release(git::GitHubRelease),
}
