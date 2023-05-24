use std::path::{Path, PathBuf};

use crate::{
    config::Config,
    db::DBConnection,
    error::*,
    package::{installed::*, *},
    util::{self, fs::FSEntry},
    *,
};
use serde::Deserialize;

use super::remote::RemotePackage;
pub use super::Dependencies;
use super::Package;
use std::time::*;

/// A remote package is a package available locally, ready to be deployed
#[derive(Clone, Package, Debug, Deserialize)]
#[allow(dead_code)]
pub struct LocalPackage {
    name: String,
    version: String,
    #[serde(deserialize_with = "crate::util::deserialize_number_from_string")]
    real_version: u64,
    description: String,
    #[serde(deserialize_with = "crate::package::Dependencies::deserialize_unresolved")]
    dependencies: Dependencies,
    hash: String,
    file_path: PathBuf,
}

impl LocalPackage {
    /// Installs this package, returns a InstalledPackage variant.
    /// This function also ensures that the dependencies are installed first.
    /// # Arguments
    /// * `config` - The configuration to use
    /// * `db_con` - The database connection to insert this package into
    pub fn install(
        self,
        config: &Config,
        db_con: &mut DBConnection,
    ) -> Result<PackageVariant, LError> {
        let emsg = format!("When installing package {}", self.get_fq_name());
        warn!("TODO: Check if this package isn't already installed");

        // Ensure the dependencies first
        for dependency in self.get_dependencies().get_resolved().err_prepend(&emsg)? {
            let mut dependency_guard = dependency.write().unwrap();
            let dependency: PackageVariant = dependency_guard.clone();

            match dependency {
                PackageVariant::Installed(_) => {}
                PackageVariant::Remote(_) => {
                    return Err(LError::new(
                        LErrorClass::UnexpectedPackageVariant,
                        "Can't install remote package",
                    ))
                    .err_prepend(&emsg);
                }
                PackageVariant::Local(p) => {
                    debug!(
                        "Installing dependency {} of package {}",
                        p.get_fq_name(),
                        self.get_fq_name()
                    );
                    *dependency_guard = p.install(config, db_con)?;
                }
            }
        }

        usermsg!("Installing package {}", self.get_fq_name());
        let installed_pkg = self.deploy(config, db_con)?;

        debug!("Done installing");
        Ok(PackageVariant::Installed(installed_pkg))
    }

    /// Deploys this package to the system using the provided config
    /// # Arguments
    /// * `config` - The config to reference for deployment
    /// * `db_con` - The database connection to use for inserting this package
    pub fn deploy(
        self,
        config: &Config,
        db_con: &mut DBConnection,
    ) -> Result<InstalledPackage, LError> {
        debug!("Extracting package {}", self.get_fq_name());
        self.extract(config)?;

        // Index the package contents
        let mut files: Vec<FSEntry> = Vec::new();
        debug!("Indexing package {}", self.get_fq_name());
        let start = Instant::now();
        files.append(&mut util::fs::index(&self.get_data_dir(config))?);
        debug!("Took {} ms", start.elapsed().as_millis());

        let transaction = db_con.new_transaction()?;
        let installed_pkg = self.copy_to_root(config, files)?;

        transaction
            .add_package(&installed_pkg)
            .err_prepend(&format!(
                "When inserting package {} into database",
                installed_pkg.get_fq_name()
            ))?;

        transaction.commit()?;

        Ok(installed_pkg)
    }

    /// Copies the package contents to the new root using the supplied config
    /// # Arguments
    /// * `config` - The configuration to use for copying
    /// * `files` - The vector of files to copy
    fn copy_to_root(
        self,
        config: &Config,
        files: Vec<FSEntry>,
    ) -> Result<InstalledPackage, LError> {
        let mut cur_src: PathBuf = self.get_data_dir(config);
        let mut cur_dest: PathBuf = PathBuf::from(config.get_root());

        // Copy the fsentries
        let mut iter = files.iter();
        util::fs::copy_recursive(&mut cur_src, &mut cur_dest, &mut iter).err_prepend(&format!(
            "When copying files of package {}",
            self.get_fq_name()
        ))?;

        let installed_pkg = InstalledPackage::from_local(self, files);

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
            std::fs::remove_dir_all(target_path)?;
        }

        // Now extract the package and take the time
        debug!(
            "Extracting package {} into {}",
            self.get_fq_name(),
            target_dir.to_string_lossy()
        );

        let start = Instant::now();
        util::extract(
            &config
                .get_download_dir()
                .join(self.get_full_name() + ".lfpkg"),
            &config.get_packages_dir(),
        )?;
        debug!("Took {} ms", start.elapsed().as_millis());

        Ok(())
    }

    /// Creates a local package from the supplied remote package using the additional information provided.
    /// # Arguments
    /// * `remote` - The remote package to derive
    /// * `file_path` - The path to the downloaded .lfpkg file
    /// * `hash` - The local hash of the file
    pub fn from_remote(remote: &RemotePackage, file_path: &Path, hash: &str) -> LocalPackage {
        Self {
            name: remote.get_name(),
            version: remote.get_version(),
            real_version: remote.get_real_version(),
            description: remote.get_description(),
            dependencies: remote.get_dependencies().clone(),
            hash: hash.to_string(),
            file_path: file_path.to_path_buf(),
        }
    }

    /// Returns the directory that results when the package is extracted
    ///
    /// Example: package `glibc-2.36` -> `<package_dir/glibc-2.36/`
    /// # Arguments
    /// * `config` - The configuration to use for getting the directories
    pub fn get_extracted_dir(&self, config: &Config) -> PathBuf {
        config.get_packages_dir().join(self.get_full_name())
    }

    /// Returns the directory that the package root filesystem lives in
    ///
    /// Example: package `glibc-2.36` -> `<package_dir/glibc-2.36/data/`
    /// # Arguments
    /// * `config` - The configuration to use for getting the directories
    pub fn get_data_dir(&self, config: &Config) -> PathBuf {
        config
            .get_packages_dir()
            .join(self.get_full_name())
            .join("data")
    }
}
