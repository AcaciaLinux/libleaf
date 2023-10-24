use log::{error, info};
use serde::Deserialize;
use std::io::Write;

use crate::config::Config;
use crate::download;
use crate::error::LError;
use crate::util;

use super::{CorePackage, LocalPackage};

/// A remote package, ready to be fetched from a remote server
#[derive(Debug, Deserialize)]
pub struct RemotePackage {
    pub name: String,
    pub version: String,
    #[serde(deserialize_with = "crate::util::deserialize_number_from_string")]
    pub real_version: u64,
    pub description: String,
    pub dependencies: Vec<String>,
    pub hash: String,
    pub url: String,
}

impl RemotePackage {
    /// Uses the provided configuration to fetch this remote package to the local system
    /// # Arguments
    /// * `config` - The configuration to use for acquiring information about the fetch process
    pub fn fetch(&self, config: &Config) -> Result<LocalPackage, LError> {
        util::ensure_dir(&config.get_download_dir())?;

        let file_path = config.get_download_dir().join(self.full_name() + ".lfpkg");

        //Check if a file exists and if so, check if the hash matches and skip the download
        if file_path.exists() && util::hash::hash_file(&file_path)? == self.hash {
            info!("Skipped fetching of package: {}", self.full_name());

            return Ok(LocalPackage::from(self));
        }

        let mut file = std::fs::File::create(&file_path)?;

        match download::download(
            &self.url,
            format!("Fetching package {}", self.full_name()).as_str(),
            config.render_bar,
            move |data| file.write_all(data).is_ok(),
        ) {
            Ok(_) => info!("Fetched package {}", self.full_name()),
            Err(e) => error!("Failed to fetch package {}: {}", self.full_name(), e),
        };

        let hash = util::hash::hash_file(&file_path).expect("Hash");

        let mut local_package = LocalPackage::from(self);
        local_package.hash = hash;

        Ok(local_package)
    }
}

impl CorePackage for RemotePackage {
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
    fn hash(&self) -> &str {
        &self.hash
    }
}
