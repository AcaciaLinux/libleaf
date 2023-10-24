use log::{debug, trace, warn};

use crate::{
    config::Config,
    error::{LError, LErrorExt},
    mirror::Mirror,
    package::{CorePackage, PackageVariant},
};

/// The PackageIndex struct bundles mirrors, local packages and installed packages
/// in one place to provide common functions and dependency resolving.
pub struct PackageIndex {
    pub mirrors: Vec<Mirror>,
}

impl PackageIndex {
    /// Create a new PackageIndex from the provided mirrors
    pub fn new(mirrors: Vec<Mirror>) -> Self {
        Self { mirrors }
    }

    /// Update the mirrors
    /// # Arguments
    /// * `config` - The configuration to use for this action
    pub fn update_mirrors(&self, config: &Config) -> Result<(), LError> {
        for mirror in &self.mirrors {
            mirror
                .update(config)
                .err_prepend(&format!("When updating mirror {}:", &mirror.name))?;
        }
        Ok(())
    }

    /// Load the mirrors
    /// # Arguments
    /// * `config` - The configuration to use for this action
    pub fn load_mirrors(&mut self, config: &Config) -> Result<(), LError> {
        for mirror in &mut self.mirrors {
            mirror
                .load(config)
                .err_prepend(&format!("When loading mirror {}:", &mirror.name))?;
        }
        Ok(())
    }

    /// Tries to find the most relevant package within this index, considering all sources
    /// # Arguments
    /// * `name` - THe name for the package
    pub fn find_package(&self, name: &str) -> Option<&PackageVariant> {
        for mirror in &self.mirrors {
            match &mirror.packages {
                None => {
                    warn!(
                        "Skipping searching mirror '{}' for package '{}': NOT LOADED",
                        mirror.name, name
                    );
                }
                Some(packages) => {
                    if let Some(package) = packages.iter().find(|p| p.name() == name) {
                        debug!(
                            "Mirror '{}' has package '{}'",
                            mirror.name,
                            package.full_name(),
                        );
                        return Some(package);
                    }
                }
            }
        }

        None
    }

    /// Resolve a list of packages to install for the provided packages to work.
    /// This will consult this index for packages and resolve all dependencies if possible
    /// # Arguments
    /// * `package_names` - The names for the packages to search
    pub fn resolve_packages<'a>(
        &'a self,
        package_names: &[&str],
    ) -> Result<Vec<&'a PackageVariant>, LError> {
        let mut pool: Vec<&PackageVariant> = Vec::new();

        for package_name in package_names {
            self.resolve_package_dependencies(package_name, &mut pool)
                .err_prepend(&format!("When resolving package {package_name}"))?;
        }

        Ok(pool)
    }

    /// Resolve a package's dependencies into the pool
    /// # Arguments
    /// * `package_name` - The name for the package to resolve the dependencies of
    /// * `pool` - The pool to resolve into
    fn resolve_package_dependencies<'a>(
        &'a self,
        package_name: &str,
        pool: &mut Vec<&'a PackageVariant>,
    ) -> Result<(), LError> {
        // Check if the package isn't already in the pool
        if let Some(p) = pool.iter().find(|p| p.name() == package_name) {
            trace!(
                "Skipping dependency resolving of package '{}' - already done",
                p.full_name()
            );
            return Ok(());
        }

        // Find the package
        let package = match self.find_package(package_name) {
            None => {
                return Err(LError::new(
                    crate::error::LErrorClass::PackageNotFound,
                    &format!("Package '{}' has not been found", package_name),
                ))
            }
            Some(p) => p,
        };

        // Push the package for now
        pool.push(package);

        let msg = format!(
            "When resolving dependencies for package '{}':",
            package.full_name()
        );
        for dependency in package.dependencies() {
            self.resolve_package_dependencies(&dependency, pool)
                .err_prepend(&msg)?;
        }

        // Pull back the package
        pool.retain(|p| p.name() != package_name);
        pool.push(package);

        Ok(())
    }
}
