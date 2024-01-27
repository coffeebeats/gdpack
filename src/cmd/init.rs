use anyhow::anyhow;
use std::path::Path;

use crate::config::Configuration;
use crate::config::Manifest;
use crate::config::Parsable;
use crate::config::Persistable;

const STR_USAGE: &str = "
Thanks for using 'gdpack'!

Use `gdpack add` to add plugin dependencies to your project.

For example:
   gdpack add https://github.com/bitwes/Gut --tag 9.1.1 -d
";

/* -------------------------------------------------------------------------- */
/*                              Function: handle                              */
/* -------------------------------------------------------------------------- */

pub fn handle(project: Option<impl AsRef<Path>>) -> anyhow::Result<()> {
    let path_project = super::parse_project(project)?;

    let path_manifest = path_project.join(Manifest::file_name().unwrap());
    if path_manifest.is_file() && Manifest::parse_file(&path_manifest).is_ok() {
        return Err(anyhow!(
            "manifest already exists: {}",
            path_manifest.to_str().unwrap()
        ));
    }

    Manifest::persist(
        &Manifest::new(),
        path_project.join(Manifest::file_name().unwrap()),
    )?;

    println!("{}", STR_USAGE);

    Ok(())
}
