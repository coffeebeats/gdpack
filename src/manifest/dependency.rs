/* -------------------------------------------------------------------------- */
/*                                 Enum: Spec                                 */
/* -------------------------------------------------------------------------- */

use anyhow::anyhow;
use std::path::PathBuf;
use std::str::FromStr;
use toml_edit::TableLike;

use crate::addon::Dependency;
use crate::addon::Spec;
use crate::git;

impl From<&Dependency> for toml_edit::InlineTable {
    fn from(value: &Dependency) -> Self {
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
            Spec::Git(s) => {
                table.insert(
                    "git",
                    toml_edit::value(s.repo.to_string()).into_value().unwrap(),
                );

                match &s.reference {
                    git::Reference::Branch(b) => {
                        table.insert("branch", toml_edit::value(b).into_value().unwrap());
                    }
                    git::Reference::Rev(r) => {
                        table.insert("rev", toml_edit::value(r).into_value().unwrap());
                    }
                    git::Reference::Tag(t) => {
                        table.insert("tag", toml_edit::value(t).into_value().unwrap());
                    }
                    _ => {}
                };
            }
            Spec::Release(r) => {
                table.insert(
                    "git",
                    toml_edit::value(r.repo.to_string()).into_value().unwrap(),
                );

                let asset = r.asset.replace(&r.tag.to_owned(), "{release}");

                table.insert("asset", toml_edit::value(asset).into_value().unwrap());
                table.insert("release", toml_edit::value(&r.tag).into_value().unwrap());
            }
        };

        if let Some(replace) = value.replace.as_ref() {
            table.insert("replace", toml_edit::value(replace).into_value().unwrap());
        }

        table
    }
}

impl TryFrom<&dyn TableLike> for Dependency {
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

            return Ok(Dependency::builder().spec(spec).replace(replace).build());
        }

        if let Some(repo) = table.get("git") {
            let repo = repo
                .as_str()
                .ok_or(anyhow!("expected string"))
                .and_then(|repo| url::Url::parse(repo).map_err(|e| anyhow!(e)))?;

            if table.len() == 1 + replace.iter().len() {
                let spec = Spec::Git(
                    git::Source::builder()
                        .repo(repo.into())
                        .reference(git::Reference::Default)
                        .build(),
                );

                return Ok(Dependency::builder().spec(spec).replace(replace).build());
            }

            if table.len() == 3 + replace.iter().len() {
                if let (Some(tag), Some(asset)) = (table.get("release"), table.get("asset")) {
                    let spec = Spec::Release(
                        git::GitHubRelease::builder()
                            .repo(repo.into())
                            .tag(tag.to_string())
                            .asset(asset.to_string())
                            .build(),
                    );

                    return Ok(Dependency::builder().spec(spec).replace(replace).build());
                }
            }

            if table.len() > 2 + replace.iter().len() {
                return Err(anyhow!("too many properties specified"));
            }

            if let Some(branch) = table.get("branch") {
                let spec = Spec::Git(
                    git::Source::builder()
                        .repo(repo.into())
                        .reference(git::Reference::Branch(branch.to_string()))
                        .build(),
                );
                return Ok(Dependency::builder().spec(spec).replace(replace).build());
            }

            if let Some(rev) = table.get("rev") {
                let spec = Spec::Git(
                    git::Source::builder()
                        .repo(repo.into())
                        .reference(git::Reference::Rev(rev.to_string()))
                        .build(),
                );
                return Ok(Dependency::builder().spec(spec).replace(replace).build());
            }

            if let Some(tag) = table.get("tag") {
                let spec = Spec::Git(
                    git::Source::builder()
                        .repo(repo.into())
                        .reference(git::Reference::Tag(tag.to_string()))
                        .build(),
                );
                return Ok(Dependency::builder().spec(spec).replace(replace).build());
            }
        }

        Err(anyhow!("incorrect properties specified"))
    }
}
