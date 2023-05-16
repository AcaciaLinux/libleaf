use super::*;
use std::sync::{Arc, RwLock};

/// A reference to a package defined as `Arc<RwLock<PackageVariant>>`
pub type PackageRef = Arc<RwLock<PackageVariant>>;

/// This trait represents the common interface for all package variants, be it remote, local or installed
pub trait PackageRefTrait: Clone {
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
    fn get_description(&self) -> String;
    /// Set the description for the package
    fn set_description(&mut self, description: &str);

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

    /// Returns true of this package is a dependency of the provided one
    /// # Arguments
    /// * `package` - The package to check
    fn is_dependency_of<T: Package>(&self, package: &T) -> bool {
        match package.get_dependencies() {
            Dependencies::Resolved(deps) => deps.iter().any(|p| p.get_name() == self.get_name()),
            Dependencies::Unresolved(deps) => deps.iter().any(|p| p == &self.get_name()),
        }
    }
}

impl PackageRefTrait for Arc<RwLock<PackageVariant>> {
    fn get_name(&self) -> String {
        self.read().expect("Lock Package mutex").get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.write().expect("Lock Package mutex").set_name(name)
    }

    fn get_version(&self) -> String {
        self.read().expect("Lock Package mutex").get_version()
    }

    fn set_version(&mut self, version: &str) {
        self.write()
            .expect("Lock Package mutex")
            .set_version(version)
    }

    fn get_real_version(&self) -> u64 {
        self.read().expect("Lock Package mutex").get_real_version()
    }

    fn set_real_version(&mut self, real_version: u64) {
        self.write()
            .expect("Lock Package mutex")
            .set_real_version(real_version)
    }

    fn get_description(&self) -> String {
        self.read().expect("Lock Package mutex").get_description()
    }

    fn set_description(&mut self, description: &str) {
        self.write()
            .expect("Lock Package mutex")
            .set_description(description)
    }

    fn set_dependencies(&mut self, dependencies: Dependencies) {
        self.write()
            .expect("Lock Package mutex")
            .set_dependencies(dependencies)
    }

    fn get_hash(&self) -> String {
        self.read().expect("Lock Package mutex").get_hash()
    }

    fn set_hash(&mut self, hash: &str) {
        self.write().expect("Lock Package mutex").set_hash(hash)
    }

    fn is_dependency_of<T: Package>(&self, package: &T) -> bool {
        match package.get_dependencies() {
            Dependencies::Resolved(deps) => deps.iter().any(|p| p.get_name() == self.get_name()),
            Dependencies::Unresolved(deps) => deps.iter().any(|p| p == &self.get_name()),
        }
    }
}
