use super::{CorePackage, InstalledPackage, RemotePackage};
use crate::{
    config::Config,
    error::{LError, LErrorExt},
    util,
};
use std::path::PathBuf;

use std::time::Instant;

use log::debug;

#[derive(Debug)]
/// A local package, ready to be installed
pub struct LocalPackage {
    pub name: String,
    pub version: String,
    pub real_version: u64,
    pub description: String,
    pub dependencies: Vec<String>,
    pub hash: String,
}

impl From<&RemotePackage> for LocalPackage {
    fn from(value: &RemotePackage) -> Self {
        Self {
            name: value.name.clone(),
            version: value.version.clone(),
            real_version: value.real_version,
            description: value.description.clone(),
            dependencies: value.dependencies.clone(),
            hash: value.hash.clone(),
        }
    }
}

impl LocalPackage {
    /// Deploys this package to the system using the provided config
    /// # Arguments
    /// * `config` - The config to reference for deployment
    /// * `db_con` - The database connection to use for inserting this package
    pub fn deploy(&self, config: &Config) -> Result<InstalledPackage, LError> {
        util::ensure_dir(&self.get_data_dir(config))?;
        debug!("Extracting package {}", self.full_name());
        self.extract(config)
            .err_prepend("When extracting package:")?;

        // Index the package contents
        let mut files: Vec<util::fs::FSEntry> = Vec::new();
        debug!("Indexing package {}", self.full_name());
        let start = Instant::now();
        files.append(
            &mut util::fs::index(&self.get_data_dir(config))
                .err_prepend("While indexing package contents:")?,
        );
        debug!("Took {} ms", start.elapsed().as_millis());

        debug!(
            "Copying package {} to root {:?}...",
            self.full_name(),
            config.get_root()
        );
        let start = Instant::now();
        let installed_pkg = self.copy_to_root(config, files).err_prepend(&format!(
            "When copying package contents to root {}:",
            config.get_root().to_string_lossy()
        ))?;
        debug!("Took {} ms", start.elapsed().as_millis());

        Ok(installed_pkg)
    }

    /// Copies the package contents to the new root using the supplied config
    /// # Arguments
    /// * `config` - The configuration to use for copying
    /// * `files` - The vector of files to copy
    fn copy_to_root(
        &self,
        config: &Config,
        files: Vec<util::fs::FSEntry>,
    ) -> Result<InstalledPackage, LError> {
        let mut cur_src: PathBuf = self.get_data_dir(config);
        let mut cur_dest: PathBuf = PathBuf::from(config.get_root());

        // Copy the fsentries
        let mut iter = files.iter();

        util::fs::copy_recursive(&mut cur_src, &mut cur_dest, &mut iter, &|path| {
            config.callbacks.file_exists(config, path)
        })
        .err_prepend("When copying files recursively:")?;

        let installed_pkg = InstalledPackage::from(self);

        Ok(installed_pkg)
    }

    /// Extracts the local package into the packages_dir
    /// # Arguments
    /// * `config` - The config to refer to for paths
    pub fn extract(&self, config: &Config) -> Result<(), LError> {
        let target_dir = config.get_packages_dir();
        let target_path = self.get_extracted_dir(config);

        // Remove the target path if it already exists
        if target_path.exists() {
            debug!(
                "Removing already existing extracted package tree at {}",
                target_path.to_str().unwrap_or("")
            );
            std::fs::remove_dir_all(target_path)
                .err_prepend("While removing existing extracted package tree:")?;
        }

        // Now extract the package and take the time
        debug!(
            "Extracting package {} into {}",
            self.full_name(),
            target_dir.to_string_lossy()
        );

        let start = Instant::now();
        util::extract::extract(
            &config.get_download_dir().join(self.full_name() + ".lfpkg"),
            &config.get_packages_dir(),
        )
        .err_prepend("While extracting lfpkg file:")?;
        debug!("Took {} ms", start.elapsed().as_millis());

        Ok(())
    }

    /// Returns the directory that results when the package is extracted
    ///
    /// Example: package `glibc-2.36` -> `<package_dir/glibc-2.36/`
    /// # Arguments
    /// * `config` - The configuration to use for getting the directories
    pub fn get_extracted_dir(&self, config: &Config) -> PathBuf {
        config.get_packages_dir().join(self.extract_name())
    }

    /// Returns the directory that the package root filesystem lives in
    ///
    /// Example: package `glibc-2.36` -> `<package_dir/glibc-2.36/data/`
    /// # Arguments
    /// * `config` - The configuration to use for getting the directories
    pub fn get_data_dir(&self, config: &Config) -> PathBuf {
        config
            .get_packages_dir()
            .join(self.extract_name())
            .join("data")
    }
}

impl CorePackage for LocalPackage {
    fn name(&self) -> &str {
        &self.name
    }
    fn version(&self) -> &str {
        &self.version
    }
    fn real_version(&self) -> u64 {
        self.real_version
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn dependencies(&self) -> &Vec<String> {
        &self.dependencies
    }
}
