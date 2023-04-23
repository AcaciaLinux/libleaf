use std::io::Write;

use serde::{Deserialize, Serialize};

use super::local::LocalPackage;
pub use super::Package;
use crate::config::Config;
use crate::download::*;
use crate::error::*;
use crate::usermsg;
use crate::util::compute_hash;

/// A remote package is a package available at a mirror for downloading
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct RemotePackage {
    name: String,
    version: String,
    #[serde(deserialize_with = "crate::util::deserialize_number_from_string")]
    real_version: u64,
    description: String,
    dependencies: Vec<String>,
    hash: String,
    url: String,
}

impl Package for RemotePackage {
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

impl RemotePackage {
    /// Uses the provided configuration to fetch this remote package to the local system
    /// # Arguments
    /// * `config` - The configuration to use for acquiring information about the fetch process
    pub fn fetch(&self, config: &Config) -> Result<LocalPackage, LError> {
        crate::util::ensure_dirs(config)?;

        let file_path = config
            .get_download_dir()
            .join(self.get_full_name() + ".lfpkg");

        let mut file = std::fs::File::create(&file_path)?;

        download(
            &self.url,
            format!("Fetching package {}", self.get_full_name()).as_str(),
            config.render_bar,
            move |data| file.write_all(data).is_ok(),
        )?;

        usermsg!("Fetched package {}", self.get_full_name());

        let hash = compute_hash(&file_path).expect("Hash");
        let mut local_package = LocalPackage::from(self);
        local_package.set_hash(hash.as_str());

        Ok(local_package)
    }
}
