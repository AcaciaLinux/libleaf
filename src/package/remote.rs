use serde::Deserialize;
use std::io::Write;

use super::local::LocalPackage;
pub use super::Dependencies;
use super::Package;
use crate::config::Config;
use crate::download::*;
use crate::error::*;
use crate::util;
use crate::{usererr, usermsg};

/// A remote package is a package available at a mirror for downloading
#[derive(Clone, Package, Debug, Deserialize)]
#[allow(dead_code)]
pub struct RemotePackage {
    name: String,
    version: String,
    #[serde(deserialize_with = "crate::util::deserialize_number_from_string")]
    real_version: u64,
    description: String,
    #[serde(deserialize_with = "crate::package::Dependencies::deserialize_unresolved")]
    dependencies: Dependencies,
    hash: String,
    url: String,
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

        //Check if a file exists and if so, check if the hash matches and skip the download
        if file_path.exists() && util::hash::hash_file(&file_path)? == self.hash {
            usermsg!("Skipped fetching of package: {}", self.get_fq_name());

            let hash = self.hash.clone();
            let local_package = LocalPackage::from_remote_unresolved(self, &file_path, &hash);

            return Ok(local_package);
        }

        let mut file = std::fs::File::create(&file_path)?;

        match download(
            &self.url,
            format!("Fetching package {}", self.get_fq_name()).as_str(),
            config.render_bar,
            move |data| file.write_all(data).is_ok(),
        ) {
            Ok(_) => usermsg!("Fetched package {}", self.get_fq_name()),
            Err(e) => usererr!(
                "Failed to fetch package {}: {}",
                self.get_fq_name(),
                e.message.unwrap_or("".to_string())
            ),
        };

        let hash = util::hash::hash_file(&file_path).expect("Hash");
        let local_package = LocalPackage::from_remote_unresolved(self, &file_path, &hash);

        Ok(local_package)
    }
}
