use crate::{error::LError, package::installed::InstalledPackage, package::Package};

use super::*;

impl DBConnection {
    /// Queries the database for the package hash matching the supplied name
    ///
    /// If the package is not in the database, this returns Ok(None)
    /// # Arguments
    /// * `name` - The name of the package to search for
    pub fn get_package_hash(&mut self, name: &str) -> Result<Option<String>, LError> {
        self.new_transaction()?.get_package_hash(name)
    }

    /// Queries the database for the package id using the supplied name
    ///
    /// If the package is not in the database, this returns Ok(None)
    /// # Arguments
    /// * `name` - The name of the package to search for
    pub fn get_package_id(&mut self, name: &str) -> Result<Option<i64>, LError> {
        self.new_transaction()?.get_package_id(name)
    }

    /// Add the supplied InstalledPackage to the database
    ///
    /// The fstree of the package and its dependencies are inserted automatically.
    /// If the package in the database is already up-to-date (hash is the same),
    /// the insertion gets skipped (`Ok()`).
    /// # Arguments
    /// * `package` - The package to insert
    pub fn add_package(&mut self, package: &InstalledPackage) -> Result<(), LError> {
        let transaction = self.new_transaction()?;
        transaction.add_package(package)?;
        transaction.commit()
    }
}

impl<'a> DBTransaction<'a> {
    /// Queries the database for the package hash matching the supplied name
    ///
    /// If the package is not in the database, this returns Ok(None)
    /// # Arguments
    /// * `name` - The name of the package to search for
    pub fn get_package_hash(&self, name: &str) -> Result<Option<String>, LError> {
        let mut stmt = self
            .transaction
            .prepare("SELECT hash FROM packages WHERE name = ?")?;
        let mut packages_iter = stmt.query_map([name], |row| {
            let res: String = row.get(0)?;
            Ok(res)
        })?;
        match packages_iter.next() {
            None => Ok(None),
            Some(hash) => Ok(Some(hash?)),
        }
    }

    /// Queries the database for the package id using the supplied name
    ///
    /// If the package is not in the database, this returns Ok(None)
    /// # Arguments
    /// * `name` - The name of the package to search for
    pub fn get_package_id(&self, name: &str) -> Result<Option<i64>, LError> {
        let mut stmt = self
            .transaction
            .prepare("SELECT id FROM packages WHERE name = ?")?;
        let mut packages_iter = stmt.query_map([name], |row| {
            let res: i64 = row.get(0)?;
            Ok(res)
        })?;
        match packages_iter.next() {
            None => Ok(None),
            Some(hash) => Ok(Some(hash?)),
        }
    }

    /// Add the supplied InstalledPackage to the database
    ///
    /// The fstree of the package and its dependencies are inserted automatically.
    /// If the package in the database is already up-to-date (hash is the same),
    /// the insertion gets skipped (`Ok()`).
    /// # Arguments
    /// * `package` - The package to insert
    pub fn add_package(&self, package: &InstalledPackage) -> Result<(), LError> {
        //Check if this package isn't already in the database
        if let Some(hash) = self.get_package_hash(&package.get_name())? {
            if hash == package.get_hash() {
                debug!(
                    "Skipping insertion of up-to-date package {} into database",
                    package.get_fq_name()
                );
                return Ok(());
            } else {
                warn!("TODO: Update package if the package differs in the database");
                return Ok(());
            }
        }

        // Before inserting this package, insert all of its dependencies if needed
        for dependency in package.get_dependencies().get_resolved()? {
            debug!(
                "Inserting dependency {} before package {}",
                dependency.get_fq_name(),
                package.get_fq_name()
            );
            self.add_package(dependency.get_installed()?)?;
        }

        // Prepare the statement and insert the package
        let mut stmt = self
            .transaction
            .prepare("INSERT INTO packages (name, version, real_version, description, hash) VALUES (?, ?, ?, ?, ?)")?;

        // Store the package id for inserting the files
        let pkgid = stmt.insert([
            package.get_name(),
            package.get_version(),
            package.get_real_version().to_string(),
            package.get_description().to_string(),
            package.get_hash(),
        ])?;

        // Add the files of the package
        self.add_files(pkgid, None, package.get_files())?;

        //Setup the dependency tree
        for dependency in package.get_dependencies().get_resolved()? {
            let dep_id = self.get_package_id(&dependency.get_name())?;
            let mut stmt = self
                .transaction
                .prepare("INSERT INTO dependencies (depender, dependency) VALUES (?, ?)")?;
            stmt.execute([pkgid, dep_id.expect("Hin")])?;
        }

        Ok(())
    }
}
