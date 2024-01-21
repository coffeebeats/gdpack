use anyhow::anyhow;
use std::collections::HashMap;
use std::path::PathBuf;
use typed_builder::TypedBuilder;

use crate::addon::Addon;
use crate::addon::Installable;
use crate::config::manifest::Dependency;
use crate::config::manifest::Manifest;
use crate::config::manifest::Query;
use crate::config::Configuration;
use crate::config::Parsable;

/* -------------------------------------------------------------------------- */
/*                                Struct: Args                                */
/* -------------------------------------------------------------------------- */

#[derive(clap::Args, Debug, TypedBuilder)]
pub struct Args {
    /// Add a development-only dependency (will not be propagated to dependents'
    /// installs).
    #[arg(long, alias = "prod")]
    pub production: bool,

    /// A `PATH` to the Godot project containing the manifest.
    #[arg(short, long, value_name = "PATH")]
    #[builder(default)]
    pub project: Option<PathBuf>,

    /// Add the dependency only for `TARGET` (can be specified more than once
    /// and accepts multiple values delimited by `,`).
    #[arg(short, long, value_name = "TARGET", value_delimiter = ',', num_args = 1..)]
    #[builder(default)]
    pub target: Vec<String>,
}

/* -------------------------------------------------------------------------- */
/*                              Function: handle                              */
/* -------------------------------------------------------------------------- */

pub fn handle(args: Args) -> anyhow::Result<()> {
    let path_project = super::parse_project(args.project)?;

    let path_manifest = path_project.join(Manifest::file_name().unwrap());
    let m = Manifest::parse_file(path_manifest)
        .map_err(|_| anyhow!("missing 'gdpack.toml' manifest; try calling 'gdpack init'"))?;

    let targets = match args.target.is_empty() {
        true => vec![None],
        false => std::iter::once(None)
            .chain(args.target.iter().map(Some))
            .collect(),
    };

    let path_project_addons = path_project.as_path().join("addons");
    if path_project_addons.is_dir() {
        std::fs::remove_dir_all(&path_project_addons)
            .map_err(|e| anyhow!("failed to remove existing 'addons' directory: {}", e))?;
    }

    // NOTE: The following installation process will need to be reworked once
    // support for transitive dependencies is added.

    // Create a per-target list of Dependencies; this will be used to validate
    // various properties of the dependency set.
    let mut deps: HashMap<Option<&str>, Vec<Dependency>> = HashMap::new();

    for target in targets.iter().map(|t| t.map(String::as_str)) {
        if target.as_ref().is_some_and(|t| t.is_empty()) {
            return Err(anyhow!("missing target"));
        }

        let dev = m.addons(Query::builder().dev(true).target(target.clone()).build());
        let prod = m.addons(Query::builder().dev(false).target(target.clone()).build());

        let target_deps: Vec<_> = if args.production {
            prod.into_iter().collect()
        } else {
            prod.into_iter().chain(dev).collect()
        };

        for dep in target_deps {
            if !deps.contains_key(&target) {
                deps.insert(target, vec![]);
            }

            if let Some(list) = deps.get_mut(&target) {
                list.push(dep);
            }
        }
    }

    // Validate the following invariants of the dependency set:

    // 1. The same addon cannot be specified 2+ times. However, a target-
    //    specified addon may override the value declared in the default target.
    let mut declared: HashMap<String, (Option<&str>, &Dependency)> = HashMap::new(); // addon name -> (target, dep)
    for (target, target_deps) in &deps {
        for dep in target_deps {
            let name = dep
                .addon
                .as_ref()
                .map(String::to_owned)
                .ok_or(anyhow!("could not determine addon name",))?;

            // Insert the addon as-is the first time it's encountered.
            if !declared.contains_key(&name) {
                declared.insert(name.to_owned(), (target.to_owned(), dep));
                continue;
            }

            match target {
                // Skip the default target because a specified target
                // declares this addon as a dependency.
                None => continue,
                Some(t) => match declared.get(&name).unwrap() {
                    // Override the default target because this target
                    // declares this addon as a dependency.
                    (None, _) => declared.insert(name.to_owned(), (Some(t), dep)),
                    (Some(t_duplicate), _) => {
                        return Err(anyhow!(
                            "duplicate addon found between targets '{}' and '{}': {}",
                            t,
                            t_duplicate,
                            name
                        ));
                    }
                },
            };
        }
    }

    // 2. The same addon cannot be replaced by 2+ addons. Note that because
    //    replacements can only be specified within a target, any collision is
    //    guaranteed to be an invalid state.
    let mut replaced: HashMap<String, String> = HashMap::new(); // replaced addon name -> target
    for (target, target_deps) in &deps {
        for dep in target_deps {
            if dep.replace.is_none() {
                continue;
            }

            let name = dep
                .addon
                .as_ref()
                .map(String::to_owned)
                .ok_or(anyhow!("could not determine addon name",))?;

            if target.is_none() {
                return Err(anyhow!(
                    "cannot specify replacement without a target: {}",
                    &name
                ));
            }

            let replace = dep.replace.as_ref().unwrap();

            if replaced.contains_key(replace) {
                return Err(anyhow!(
                    "duplicate replacement of addon found between targets '{}' and '{}': {}",
                    target.as_ref().unwrap(),
                    replaced.get(replace).unwrap(),
                    name
                ));
            }

            replaced.insert(replace.to_owned(), target.unwrap().to_owned());
        }
    }

    // Now, determine the final set of addons to install.

    let deps: Vec<&Dependency> = declared
        .into_iter()
        .filter(|(name, _)| !replaced.contains_key(name))
        .filter_map(|(_, (_, dep))| Some(dep))
        .collect::<Vec<_>>();

    let mut addons: Vec<Addon> = vec![];

    // 3. Multiple addons cannot be installed to the same subfolder within the
    //    _Godot_ project's "addons" folder.
    let mut subfolders = HashMap::<String, String>::new(); // subfolder name -> addon name

    for dep in deps {
        // Download the [`Dependency`] to the `gdpack` store.
        dep.source.fetch()?;

        // Load the [`Addon`] from the fetched [`Dependency`].
        let addon =
            Addon::try_from(dep).map_err(|e| anyhow!("failed to load addon from disk: {:}", e))?;

        let subfolder = addon.subfolder.to_owned();

        let name = dep
            .addon
            .as_ref()
            .map(String::to_owned)
            .or(dep.source.name())
            .ok_or(anyhow!("missing dependency name: {:?}", dep.source))?;

        if subfolders.contains_key(&subfolder) {
            let other = subfolders.get(&subfolder).unwrap();

            return Err(anyhow!(
                "duplicate subfolder found between addons '{}' and '{}': {}",
                &name,
                other,
                &subfolder
            ));
        }

        addons.push(addon);

        subfolders.insert(subfolder, name);
    }

    // Finally, install the validated set of addons.

    for addon in addons {
        addon
            .install_to(&path_project_addons)
            .map_err(|e| anyhow!("failed to install addon: {:}", e))?;
    }

    Ok(())
}
