//! Functions to manage and use the registry part of the database
use super::DBConnection;
use std::str::FromStr;

impl DBConnection {
    /// Gets an entry from the registry matching the supplied key
    /// # Arguments
    /// * `key` - The key to search from
    pub fn reg_get<T: FromStr>(&self, key: &str) -> Result<Option<T>, rusqlite::Error> {
        let mut stmt = self
            .connection
            .prepare("SELECT reg_value FROM registry WHERE reg_key = ?")?;
        let mut iter = stmt.query_map([key], |row| {
            let str: String = row.get(0)?;
            Ok(str)
        })?;

        match iter.next() {
            Some(element) => match element?.parse::<T>() {
                Ok(v) => Ok(Some(v)),
                Err(_) => Err(rusqlite::Error::from(
                    rusqlite::types::FromSqlError::InvalidType,
                )),
            },
            None => Ok(None),
        }
    }

    /// Updates or sets the entry matching the key to the supplied value
    /// # Arguments
    /// * `key` - The key to update or set
    /// * `value` - The value to set
    pub fn reg_set<T: ToString>(&self, key: &str, value: &T) -> Result<usize, rusqlite::Error> {
        self.connection.execute(
            "REPLACE INTO registry VALUES (?, ?)",
            (key, value.to_string()),
        )
    }
}
