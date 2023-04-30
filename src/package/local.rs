use std::path::{Path, PathBuf};

use crate::{config::Config, error::LError, usermsg, util};
use serde::Deserialize;

use super::remote::RemotePackage;
pub use super::Dependencies;
use super::Package;

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
    /// Deploys this package to the system using the provided config
    /// # Arguments
    /// * `config` - The config to reference for deployment
    pub fn deploy(&self, config: &Config) -> Result<(), LError> {
        for dep in self.dependencies.get_resolved()? {
            let dep = dep.get_local()?;
            dep.deploy(config)?;
        }

        usermsg!("Deploying package {}", self.get_fq_name());

        self.extract(config)?;

        Ok(())
    }

    /// Extracts the local package into the packages_dir
    /// # Arguments
    /// * `config` - The config to refer to for paths
    pub fn extract(&self, config: &Config) -> Result<(), LError> {
        let target_dir = config.get_packages_dir();
        let target_path = target_dir.join(self.get_full_name());

        if target_path.exists() {
            debug!(
                "Removing already existing extracted package tree at {}",
                target_path.to_str().unwrap_or("")
            );
            std::fs::remove_dir_all(target_path)?;
        }

        info!(
            "Extracting package {} into {}",
            self.get_full_name(),
            target_dir.to_str().unwrap_or("")
        );

        util::extract(
            &config
                .get_download_dir()
                .join(self.get_full_name() + ".lfpkg"),
            &config.get_packages_dir(),
        )?;

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
            description: remote.get_description().to_owned(),
            dependencies: remote.get_dependencies().clone(),
            hash: hash.to_string(),
            file_path: file_path.to_path_buf(),
        }
    }
}
