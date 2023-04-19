use serde::{Deserialize, Serialize};

pub use super::Package;

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