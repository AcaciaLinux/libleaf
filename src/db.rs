//! A wrapper around the leaf database
use rusqlite::{Connection, OpenFlags, Transaction};
use std::path::Path;

use crate::error::LError;
mod reg;
mod tables;

mod fsentries;
mod packages;
pub use fsentries::*;
pub use packages::*;

/// Represents a connection to a database
pub struct DBConnection {
    connection: Connection,
}

/// Represents a database transaction
pub struct DBTransaction<'a> {
    transaction: Transaction<'a>,
}

/// A struct representing a connection to the leaf database
impl DBConnection {
    /// Creates a new connection opening the supplied file
    /// # Arguments
    /// * `path` - The path to read from
    pub fn open(path: &Path) -> Result<DBConnection, LError> {
        let connection = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_FULL_MUTEX,
        )?;

        let con = DBConnection { connection };
        con.ensure_tables()?;

        Ok(con)
    }

    /// Creates a new transaction for this database
    pub fn new_transaction(&mut self) -> Result<DBTransaction, LError> {
        Ok(DBTransaction {
            transaction: self.connection.transaction()?,
        })
    }
}

impl<'a> DBTransaction<'a> {
    /// Consumes this transaction and commits the changes to the database
    pub fn commit(self) -> Result<(), LError> {
        Ok(self.transaction.commit()?)
    }

    /// Consumes this transaction and rolls the changes back
    pub fn rollback(self) -> Result<(), LError> {
        Ok(self.transaction.rollback()?)
    }
}
