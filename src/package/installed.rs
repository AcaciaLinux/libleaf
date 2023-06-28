use std::sync::{Arc, RwLock};

pub use super::Dependencies;
use super::{local::LocalPackage, Package, PackageRef, PackageRefTrait, PackageVariant};
use crate::{
    db::DBTransaction,
    error::{LError, LErrorClass},
    util::fs::FSEntry,
};
use serde::Deserialize;

/// A installed package has a vector of the FSEntries that it contains
#[derive(Clone, Package, Debug, Deserialize)]
#[allow(dead_code)]
pub struct InstalledPackage {
    name: String,
    version: String,
    #[serde(deserialize_with = "crate::util::deserialize_number_from_string")]
    real_version: u64,
    description: String,
    #[serde(deserialize_with = "crate::package::Dependencies::deserialize_unresolved")]
    dependencies: Dependencies,
    hash: String,
    #[serde(skip)]
    files: Vec<FSEntry>,
}

impl InstalledPackage {
    /// Sets the files for the packages
    /// # Arguments
    /// * `files` - The files to set
    pub fn set_files(&mut self, files: Vec<FSEntry>) {
        self.files = files;
    }

    /// Returns a reference to the vector of FSEntries that are provided by this package
    pub fn get_files(&self) -> &Vec<FSEntry> {
        &self.files
    }

    /// Creates a installed package from the supplied local package using the additional information provided
    /// # Arguments
    /// * `remote` - The remote package to derive
    /// * `files` - The files provied by this package
    pub fn from_local(local: LocalPackage, files: Vec<FSEntry>) -> InstalledPackage {
        Self {
            name: local.get_name(),
            version: local.get_version(),
            real_version: local.get_real_version(),
            description: local.get_description(),
            dependencies: local.get_dependencies().clone(),
            hash: local.get_hash(),
            files,
        }
    }

    /// Creates a raw InstalledPackage missing its dependencies and files from the supplied transaction.
    ///
    /// Dependencies are unresolved and need to be set manually.
    /// The files vector is empty and needs to be set manually.
    /// # Arguments
    /// * `transaction` - The transaction to use
    /// * `name` - The name of the package to search for
    /// # Returns
    /// None if the package hasn't been found
    pub fn raw_from_sql(
        transaction: &mut DBTransaction,
        name: &str,
    ) -> Result<Option<Self>, LError> {
        let mut stmt = transaction.prepare(
            "SELECT name, version, real_version, description, hash FROM packages WHERE name = ?",
        )?;

        let mut packages_iter = stmt.query_map([name], |row| {
            let res = Self {
                name: row.get(0)?,
                version: row.get(1)?,
                real_version: row.get(2)?,
                description: row.get(3)?,
                hash: row.get(4)?,
                dependencies: Dependencies::Unresolved(vec![]),
                files: Vec::new(),
            };
            Ok(res)
        })?;

        match packages_iter.next() {
            Some(p) => Ok(Some(p?)),
            None => Ok(None),
        }
    }

    /// Creates a stub InstalledPackage from the supplied transaction.
    /// A stub package does not contain its files.
    ///
    /// The files vector is empty and needs to be set manually.
    /// # Arguments
    /// * `transaction` - The transaction to use
    /// * `name` - The name of the package to search for
    /// * `pool` - The pool to resolve package dependencies from and insert the new package into
    /// # Returns
    /// None if the package hasn't been found
    pub fn stub_from_sql(
        transaction: &mut DBTransaction,
        name: &str,
        pool: &mut Vec<PackageRef>,
    ) -> Result<Option<PackageRef>, LError> {
        // First check if the package isn't already in the pool
        if let Some(package) = pool.iter().find(|p| p.get_name() == name) {
            return Ok(Some(package.clone()));
        }

        // Retrieve the raw package
        let mut new_package = match Self::raw_from_sql(transaction, name)? {
            Some(p) => p,
            None => return Ok(None),
        };

        // Resolve the dependencies
        let mut new_deps: Vec<Arc<RwLock<PackageVariant>>> = Vec::new();

        for dep in transaction.get_package_dependencies(&new_package.hash)? {
            match InstalledPackage::stub_from_sql(transaction, &dep, pool)? {
                None => {
                    return Err(LError::new(
                        LErrorClass::UnresolvedDependencies,
                        &format!(
                            "The dependency {} of {} is missing from the installed database",
                            dep,
                            new_package.get_fq_name()
                        ),
                    ))
                }
                Some(package) => {
                    new_deps.push(package);
                }
            }
        }

        new_package.dependencies = Dependencies::Resolved(new_deps);

        // Finally push the package to the pool and return a copy
        let new_package = Arc::new(RwLock::new(PackageVariant::Installed(new_package)));
        pool.push(new_package.clone());

        Ok(Some(new_package))
    }
}
