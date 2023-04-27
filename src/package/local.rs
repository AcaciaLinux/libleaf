use crate::{config::Config, error::LError, util};
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
}

impl LocalPackage {
    /// Extracts the local package into the packages_dir
    /// # Arguments
    /// * `config` - The config to refer to for paths
    pub fn extract(&self, config: &Config) -> Result<(), LError> {
        let target_dir = config.get_packages_dir();
        let target_path = target_dir.join(self.get_full_name());

        if target_path.exists() {
            info!(
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
}

impl From<&RemotePackage> for LocalPackage {
    fn from(value: &RemotePackage) -> Self {
        Self {
            name: value.get_name(),
            version: value.get_version(),
            real_version: value.get_real_version(),
            description: value.get_description().to_owned(),
            dependencies: value.get_dependencies().clone(),
            hash: "".to_owned(),
        }
    }
}
