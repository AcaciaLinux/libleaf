//! A package is the main work horse of the leaf package manager.
//! Every variant of a package implements the trait Package.

pub mod local;
pub mod remote;

use serde::Deserializer;
pub use serde::{de, Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug)]
pub enum PackageVariant {
    Local(local::LocalPackage),
    Remote(remote::RemotePackage),
}

#[derive(Debug)]
pub enum Dependencies {
    Unresolved(Vec<String>),
    Resolved(Vec<Arc<PackageVariant>>),
}

/// This trait represents the common interface for all package variants, be it remote, local or installed
pub trait Package: Clone {
    /// Get the package name
    fn get_name(&self) -> String;
    /// Set the package name
    fn set_name(&mut self, name: &str);

    /// Get the package version string
    fn get_version(&self) -> String;
    /// Set the package version string
    fn set_version(&mut self, version: &str);

    /// Get the package real version
    fn get_real_version(&self) -> u64;
    /// Set the package real version
    fn set_real_version(&mut self, real_version: u64);

    /// Get the description of the package
    fn get_description(&self) -> &str;
    /// Set the description for the package
    fn set_description(&mut self, description: &str);

    /// Get a reference to the dependencies required by the package
    fn get_dependencies(&self) -> &Dependencies;
    /// Set the dependencies needed by this package
    fn set_dependencies(&mut self, dependencies: Dependencies);

    /// Get the package MD5 hash
    fn get_hash(&self) -> String;
    /// Set the package MD5 hash
    fn set_hash(&mut self, hash: &str);

    /// Get the full name for the package: `<name>-<version>` (E.g: glibc-2.3)
    fn get_full_name(&self) -> String {
        format!("{}-{}", self.get_name(), self.get_version())
    }

    /// Get the fully qualified name for the package: `<name>-<version>-<real_version>` (E.g: glibc-2.3-9)
    fn get_fq_name(&self) -> String {
        format!("{}-{}-{}", self.get_name(), self.get_version(), self.get_real_version())
    }
}

impl Clone for Dependencies {
    /// If resolved dependencies get cloned, they become unresolved
    fn clone(&self) -> Self {
        match self {
            Self::Unresolved(arg0) => Self::Unresolved(arg0.clone()),
            Self::Resolved(arg0) => Self::Unresolved(
                arg0.iter()
                    .map(|pkg| match pkg.as_ref() {
                        PackageVariant::Local(pkg) => pkg.get_name(),
                        PackageVariant::Remote(pkg) => pkg.get_name(),
                    })
                    .collect(),
            ),
        }
    }
}

impl Dependencies {
    pub fn deserialize_unresolved<'de, D>(deserializer: D) -> Result<Dependencies, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct UnresolvedData {
            data: Vec<String>,
        }

        let data = UnresolvedData::deserialize(deserializer)?;
        Ok(Dependencies::Unresolved(data.data))
    }
}
