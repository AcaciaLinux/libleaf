use super::*;
use crate::{error::*, package::installed::*, package::*};

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

    /// Add the supplied PackageVariant to the database
    ///
    /// If the package in the database is already up-to-date (hash is the same),
    /// the insertion gets skipped (`Ok()`).
    /// # Arguments
    /// * `package` - The package to insert
    pub fn insert_package(&mut self, package: &PackageVariant) -> Result<(), LError> {
        let transaction = self.new_transaction()?;
        transaction.insert_package(package)?;
        transaction.commit()
    }

    /// Add the supplied PackageVariants dependencies to the database
    /// # Arguments
    /// * `package` - The package to insert the dependencies of
    pub fn insert_package_dependencies(&mut self, package: &PackageVariant) -> Result<(), LError> {
        let transaction = self.new_transaction()?;
        transaction.insert_package_dependencies(package)?;
        transaction.commit()
    }

    /// Retrieves the dependencies of the package matching the supplied hash
    ///
    /// # Arguments
    /// * `hash` - The hash to use for searching
    /// # Returns
    /// A vector of strings containing the dependency names
    pub fn get_package_dependencies(&mut self, hash: &str) -> Result<Vec<String>, LError> {
        self.new_transaction()?.get_package_dependencies(hash)
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
    pub fn insert_package(&self, package: &PackageVariant) -> Result<(), LError> {
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

        // Prepare the statement and insert the package
        let mut stmt = self
            .transaction
            .prepare("INSERT INTO packages (name, version, real_version, description, hash) VALUES (?, ?, ?, ?, ?)")?;

        stmt.insert([
            package.get_name(),
            package.get_version(),
            package.get_real_version().to_string(),
            package.get_description(),
            package.get_hash(),
        ])
        .err_prepend(&format!("When inserting package {}", package.get_fq_name()))?;

        Ok(())
    }

    pub fn insert_package_dependencies(&self, package: &PackageVariant) -> Result<(), LError> {
        let pkgid = match self.get_package_id(&package.get_name())? {
            Some(id) => id,
            None => {
                return Err(LError::new(
                    LErrorClass::PackageNotFound,
                    "Insert the package first",
                ));
            }
        };

        //Set up the dependency tree
        let name = &package.get_name();
        for dependency in package.get_dependencies().get_resolved()? {
            trace!("Inserting dependency [{}] {}", name, dependency.get_name());
            let dep_id = match self.get_package_id(&dependency.get_name())? {
                Some(id) => id,
                None => {
                    return Err(LError::new(
                        LErrorClass::PackageNotFound,
                        &format!(
                            "Dependency id of [{}] {} is missing",
                            name,
                            dependency.get_name()
                        ),
                    ))
                }
            };
            let mut stmt = self.transaction.prepare(
                "INSERT OR REPLACE INTO dependencies (depender, dependency) VALUES (?, ?)",
            )?;
            stmt.execute([pkgid, dep_id])?;
        }

        Ok(())
    }

    /// Retrieves the dependencies of the package matching the supplied hash
    ///
    /// # Arguments
    /// * `hash` - The hash to use for searching
    /// # Returns
    /// A vector of strings containing the dependency names
    pub fn get_package_dependencies(&self, hash: &str) -> Result<Vec<String>, LError> {
        let mut stmt = self.transaction.prepare(
            "SELECT p2.name
                        FROM dependencies, packages p1, packages p2
                        WHERE p1.hash = ?
                            AND p1.id = dependencies.depender
                            AND p2.id = dependencies.dependency;",
        )?;

        let dependencies = stmt.query_map([hash], |row| {
            let res: String = row.get(0)?;
            Ok(res)
        })?;

        let mut deps: Vec<String> = Vec::new();

        for dep in dependencies {
            deps.push(dep?);
        }

        Ok(deps)
    }

    /// Adds the supplied files to the parent owned by the supplied package
    /// # Arguments
    /// * `pkgid` - The package the files are owned by
    /// * `parent` - The id of the parent file id (None for root entry)
    /// * `files` - The files to add
    pub fn insert_package_files(&self, package: &InstalledPackage) -> Result<(), LError> {
        let pkgid = match self.get_package_id(&package.get_name())? {
            Some(id) => id,
            None => {
                return Err(LError::new(
                    LErrorClass::PackageNotFound,
                    "Insert the package first",
                ));
            }
        };

        self.insert_files(pkgid, None, package.get_files())?;

        Ok(())
    }
}
