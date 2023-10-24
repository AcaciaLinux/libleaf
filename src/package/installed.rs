use super::{CorePackage, LocalPackage};

#[derive(Debug)]
/// A local package, ready to be installed
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub real_version: u64,
    pub description: String,
    pub dependencies: Vec<String>,
}

impl From<&LocalPackage> for InstalledPackage {
    fn from(value: &LocalPackage) -> Self {
        Self {
            name: value.name.clone(),
            version: value.version.clone(),
            real_version: value.real_version,
            description: value.description.clone(),
            dependencies: value.dependencies.clone(),
        }
    }
}

impl CorePackage for InstalledPackage {
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
