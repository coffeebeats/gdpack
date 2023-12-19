use std::path::PathBuf;

use crate::git;

/* -------------------------------------------------------------------------- */
/*                                 Enum: Spec                                 */
/* -------------------------------------------------------------------------- */

#[derive(Debug, serde::Deserialize, PartialEq, Eq, serde::Serialize)]
#[serde(untagged)]
pub enum Spec {
    Path(PathBuf),
    Git(git::Source),
}
