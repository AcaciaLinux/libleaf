pub use super::Dependencies;
use super::{local::LocalPackage, Package};
use crate::util::fs::FSEntry;
use serde::Deserialize;

use rusqlite::Row;

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
            description: local.get_description().to_owned(),
            dependencies: local.get_dependencies().clone(),
            hash: local.get_hash(),
            files,
        }
    }

    /// Creates a installed package from the supplied local package using the additional information provided.
    ///
    /// The dependencies of this package become unresolved
    /// # Arguments
    /// * `remote` - The remote package to derive
    /// * `files` - The files provied by this package
    pub fn from_local_unresolved(local: LocalPackage, files: Vec<FSEntry>) -> InstalledPackage {
        Self {
            name: local.get_name(),
            version: local.get_version(),
            real_version: local.get_real_version(),
            description: local.get_description().to_owned(),
            dependencies: local.get_dependencies().clone_unresolved(),
            hash: local.get_hash(),
            files,
        }
    }

    /// Creates a InstalledPackage from the supplied rusqlite Row
    ///
    /// Dependencies are unresolved and need to be set manually.
    /// The files vector is empty and needs to be set manually.
    /// # Arguments
    /// * `row` - The row to create the package from
    pub fn from_sql(row: &Row) -> Result<InstalledPackage, rusqlite::Error> {
        Ok(InstalledPackage {
            name: row.get(0)?,
            version: row.get(1)?,
            real_version: row.get(2)?,
            description: row.get(3)?,
            hash: row.get(4)?,
            dependencies: Dependencies::Unresolved(vec![]),
            files: vec![],
        })
    }
}
