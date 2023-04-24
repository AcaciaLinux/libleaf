use serde::{Deserialize, Serialize};

use crate::{config::Config, error::LError, util};

use super::remote::RemotePackage;
pub use super::Package;

/// A remote package is a package available locally, ready to be deployed
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct LocalPackage {
    name: String,
    version: String,
    #[serde(deserialize_with = "crate::util::deserialize_number_from_string")]
    real_version: u64,
    description: String,
    dependencies: Vec<String>,
    hash: String,
}

impl Package for LocalPackage {
    fn get_name(&self) -> String {
        self.name.to_owned()
    }
    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned()
    }

    fn get_version(&self) -> String {
        self.version.to_owned()
    }
    fn set_version(&mut self, version: &str) {
        self.version = version.to_owned()
    }

    fn get_real_version(&self) -> u64 {
        self.real_version
    }
    fn set_real_version(&mut self, real_version: u64) {
        self.real_version = real_version
    }

    fn get_description(&self) -> &str {
        self.description.as_str()
    }
    fn set_description(&mut self, description: &str) {
        self.description = description.to_owned()
    }

    fn get_dependencies<'a>(&'a self) -> &'a Vec<String> {
        &self.dependencies
    }
    fn set_dependencies(&mut self, dependencies: Vec<String>) {
        self.dependencies = dependencies
    }

    fn get_hash(&self) -> String {
        self.hash.to_owned()
    }

    fn set_hash(&mut self, hash: &str) {
        self.hash = hash.to_owned()
    }

    fn get_full_name(&self) -> String {
        format!("{}-{}", self.name, self.version)
    }
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
