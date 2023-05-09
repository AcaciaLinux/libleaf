//! A wrapper around the leaf database
use rusqlite::{Connection, OpenFlags};
use std::path::Path;

use crate::error::LError;
mod reg;
mod tables;

pub struct DBConnection {
    connection: Connection,
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
}
