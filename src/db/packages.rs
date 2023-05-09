use crate::error::LError;

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
}
