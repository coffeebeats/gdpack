/* -------------------------------------------------------------------------- */
/*                                 Enum: Spec                                 */
/* -------------------------------------------------------------------------- */

use anyhow::anyhow;
use std::path::PathBuf;
use std::str::FromStr;
use toml_edit::TableLike;

use crate::addon::git;
use crate::addon::Addon;
use crate::addon::Spec;

impl From<&Addon> for toml_edit::InlineTable {
    fn from(value: &Addon) -> Self {
        let mut table = toml_edit::InlineTable::new();

        match &value.spec {
            Spec::Path(p) => {
                table.insert(
                    "path",
                    toml_edit::value(p.to_str().expect("invalid path"))
                        .into_value()
                        .unwrap(),
                );
            }
            Spec::Git(g) => {
                table.insert(
                    "git",
                    toml_edit::value(g.repo.to_string()).into_value().unwrap(),
                );

                match &g.commit {
                    git::Commit::Branch(b) => {
                        table.insert("branch", toml_edit::value(b).into_value().unwrap());
                    }
                    git::Commit::Rev(r) => {
                        table.insert("rev", toml_edit::value(r).into_value().unwrap());
                    }
                    git::Commit::Tag(t) => {
                        table.insert("tag", toml_edit::value(t).into_value().unwrap());
                    }
                    _ => {}
                };
            }
        };

        if let Some(replace) = value.replace.as_ref() {
            table.insert("replace", toml_edit::value(replace).into_value().unwrap());
        }

        table
    }
}

impl TryFrom<&dyn TableLike> for Addon {
    type Error = anyhow::Error;

    fn try_from(table: &dyn TableLike) -> Result<Self, Self::Error> {
        let replace = table
            .get("replace")
            .and_then(|v| v.as_str())
            .map(|r| r.to_owned());

        if let Some(path) = table.get("path") {
            if table.len() > 1 + replace.iter().len() {
                return Err(anyhow!("too many properties specified"));
            }

            let spec = path
                .as_str()
                .ok_or(anyhow!("expected a string"))
                .and_then(|p| PathBuf::from_str(p).map_err(|e| anyhow!(e)))
                .map(Spec::Path)?;

            return Ok(Addon::builder().spec(spec).replace(replace).build());
        }

        if let Some(repo) = table.get("git") {
            let repo = repo
                .as_str()
                .ok_or(anyhow!("expected string"))
                .and_then(|repo| url::Url::parse(repo).map_err(|e| anyhow!(e)))?;

            if table.len() == 1 + replace.iter().len() {
                let spec = Spec::Git(git::Spec::new(repo, git::Commit::Default));
                return Ok(Addon::builder().spec(spec).replace(replace).build());
            }

            if table.len() > 2 + replace.iter().len() {
                return Err(anyhow!("too many properties specified"));
            }

            if let Some(branch) = table.get("branch") {
                let spec = Spec::Git(git::Spec::new(
                    repo,
                    git::Commit::Branch(branch.to_string()),
                ));
                return Ok(Addon::builder().spec(spec).replace(replace).build());
            }

            if let Some(rev) = table.get("rev") {
                let spec = Spec::Git(git::Spec::new(repo, git::Commit::Rev(rev.to_string())));
                return Ok(Addon::builder().spec(spec).replace(replace).build());
            }

            if let Some(tag) = table.get("tag") {
                let spec = Spec::Git(git::Spec::new(repo, git::Commit::Tag(tag.to_string())));
                return Ok(Addon::builder().spec(spec).replace(replace).build());
            }
        }

        Err(anyhow!("incorrect properties specified"))
    }
}
