//! A package is the main work horse of the leaf package manager.
//! Every variant of a package implements the trait Package.

pub mod local;
pub mod remote;

pub use derive::Package;
use serde::Deserializer;
pub use serde::{de, Deserialize, Serialize};
use std::sync::Arc;

use crate::error::{LError, LErrorClass};

#[derive(Clone, Debug)]
pub enum PackageVariant {
    Local(local::LocalPackage),
    Remote(remote::RemotePackage),
}

impl Package for PackageVariant {
    fn get_name(&self) -> String {
        match self {
            PackageVariant::Local(p) => p.get_name(),
            PackageVariant::Remote(p) => p.get_name(),
        }
    }

    fn set_name(&mut self, name: &str) {
        match self {
            PackageVariant::Local(p) => p.set_name(name),
            PackageVariant::Remote(p) => p.set_name(name),
        }
    }

    fn get_version(&self) -> String {
        match self {
            PackageVariant::Local(p) => p.get_version(),
            PackageVariant::Remote(p) => p.get_version(),
        }
    }

    fn set_version(&mut self, version: &str) {
        match self {
            PackageVariant::Local(p) => p.set_version(version),
            PackageVariant::Remote(p) => p.set_version(version),
        }
    }

    fn get_real_version(&self) -> u64 {
        match self {
            PackageVariant::Local(p) => p.get_real_version(),
            PackageVariant::Remote(p) => p.get_real_version(),
        }
    }

    fn set_real_version(&mut self, real_version: u64) {
        match self {
            PackageVariant::Local(p) => p.set_real_version(real_version),
            PackageVariant::Remote(p) => p.set_real_version(real_version),
        }
    }

    fn get_description(&self) -> &str {
        match self {
            PackageVariant::Local(p) => p.get_description(),
            PackageVariant::Remote(p) => p.get_description(),
        }
    }

    fn set_description(&mut self, description: &str) {
        match self {
            PackageVariant::Local(p) => p.set_description(description),
            PackageVariant::Remote(p) => p.set_description(description),
        }
    }

    fn get_dependencies(&self) -> &Dependencies {
        match self {
            PackageVariant::Local(p) => p.get_dependencies(),
            PackageVariant::Remote(p) => p.get_dependencies(),
        }
    }

    fn set_dependencies(&mut self, dependencies: Dependencies) {
        match self {
            PackageVariant::Local(p) => p.set_dependencies(dependencies),
            PackageVariant::Remote(p) => p.set_dependencies(dependencies),
        }
    }

    fn get_hash(&self) -> String {
        match self {
            PackageVariant::Local(p) => p.get_hash(),
            PackageVariant::Remote(p) => p.get_hash(),
        }
    }

    fn set_hash(&mut self, hash: &str) {
        match self {
            PackageVariant::Local(p) => p.set_hash(hash),
            PackageVariant::Remote(p) => p.set_hash(hash),
        }
    }
}

impl PackageVariant {
    /// Returns a LocalPackage if this is a local package, else UnexpectedPackageVariant
    pub fn get_local(&self) -> Result<&local::LocalPackage, LError> {
        match self {
            Self::Local(p) => Ok(p),
            _ => Err(LError::new(
                LErrorClass::UnexpectedPackageVariant,
                "Expected local",
            )),
        }
    }

    /// Returns a RemotePackage if this is a remote package, else UnexpectedPackageVariant
    pub fn get_remote(&self) -> Result<&remote::RemotePackage, LError> {
        match self {
            Self::Remote(p) => Ok(p),
            _ => Err(LError::new(
                LErrorClass::UnexpectedPackageVariant,
                "Expected remote",
            )),
        }
    }
}

#[derive(Debug)]
pub enum Dependencies {
    Unresolved(Vec<String>),
    Resolved(Vec<Arc<PackageVariant>>),
}

impl Dependencies {
    /// Returns unresolved dependencies if available, else UnexpectedDependenciesVariant
    pub fn get_unresolved(&self) -> Result<&Vec<String>, LError> {
        match self {
            Self::Unresolved(d) => Ok(d),
            _ => Err(LError::new(
                LErrorClass::UnexpectedDependenciesVariant,
                "Expected unresolved",
            )),
        }
    }

    /// Returns resolved dependencies if available, else UnexpectedDependenciesVariant
    pub fn get_resolved(&self) -> Result<&Vec<Arc<PackageVariant>>, LError> {
        match self {
            Self::Resolved(d) => Ok(d),
            _ => Err(LError::new(
                LErrorClass::UnexpectedDependenciesVariant,
                "Expected resolved",
            )),
        }
    }
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
        format!(
            "{}-{}-{}",
            self.get_name(),
            self.get_version(),
            self.get_real_version()
        )
    }

    /// Convert the Package to one containing an empty Vec of resolved dependencies
    fn clone_to_resolved(&self) -> Self {
        let mut s = self.clone();
        s.set_dependencies(Dependencies::Resolved(Vec::new()));
        s
    }

    /// Convert the Package to one containing an empty Vec of unresolved dependencies
    fn clone_to_unresolved(&self) -> Self {
        let mut s = self.clone();
        s.set_dependencies(Dependencies::Unresolved(Vec::new()));
        s
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
