use super::{CorePackage, RemotePackage};

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
    fn hash(&self) -> &str {
        &self.hash
    }
}
