//! Core of cargo-remove command

mod dependency;
mod manifest;
mod metadata;

use cargo::core::Package;
use cargo::CargoResult;
use cargo::Config;

pub use self::dependency::Dependency;
pub use self::dependency::RegistrySource;
pub use self::manifest::DepTable;
pub use self::manifest::LocalManifest;
pub use self::manifest::Manifest;

/// Remove a dependency from a Cargo.toml manifest file.
#[derive(Debug)]
pub struct RemoveOptions<'a> {
    /// Configuration information for Cargo operations
    pub config: &'a Config,
    /// Package to remove dependencies from
    pub spec: &'a Package,
    /// Dependencies to remove
    pub dependencies: Vec<String>,
    /// Which dependency section to remove these from
    pub section: DepTable,
    /// Whether or not to actually write the manifest
    pub dry_run: bool,
}

/// Remove dependencies from a manifest
pub fn remove(options: &RemoveOptions<'_>) -> CargoResult<()> {
    let dep_table = options
        .section
        .to_table()
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();

    let manifest_path = options.spec.manifest_path().to_path_buf();
    let mut manifest = LocalManifest::try_new(&manifest_path)?;

    options
        .dependencies
        .iter()
        .map(|dep| {
            let section = {
                let table_name = options.section.kind_table_name();
                if let Some(target) = options.section.target() {
                    format!("{table_name} for target `{target}`")
                } else {
                    table_name.to_owned()
                }
            };
            options
                .config
                .shell()
                .status("Removing", format!("{dep} from {section}"))?;

            let result = manifest
                .remove_from_table(&dep_table, dep)
                .map_err(Into::into);

            // Now that we have removed the crate, if that was the last reference to that
            // crate, then we need to drop any explicitly activated features on
            // that crate.
            manifest.gc_dep(dep);

            result
        })
        .collect::<CargoResult<Vec<_>>>()?;

    if options.dry_run {
        options
            .config
            .shell()
            .warn("aborting remove due to dry run")?;
    } else {
        manifest.write()?;
    }

    Ok(())
}
