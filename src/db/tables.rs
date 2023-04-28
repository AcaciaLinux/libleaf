use super::DBConnection;

impl DBConnection {
    /// Ensures that the tables needed for operation are available
    ///
    /// Gets run on open automatically
    pub fn ensure_tables(&self) -> Result<(), rusqlite::Error> {
        let connection = &self.connection;

        connection.execute("PRAGMA foreign_keys = ON", ())?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS packages (
                id              INTEGER PRIMARY KEY NOT NULL,
                name            TEXT UNIQUE NOT NULL,
                version         TEXT NOT NULL,
                real_version    INTEGER NOT NULL,
                description     TEXT,
                hash            TEXT
            )",
            (),
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS dependencies (
                depender        INTEGER NOT NULL,
                dependency      INTEGER NOT NULL,

                FOREIGN KEY(depender) REFERENCES packages(id) ON UPDATE CASCADE ON DELETE CASCADE,
                FOREIGN KEY(dependency) REFERENCES packages(id) ON UPDATE CASCADE ON DELETE CASCADE,

                PRIMARY KEY(depender, dependency)
            )",
            (),
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS files (
                package         INTEGER NOT NULL,
                path            TEXT UNIQUE NOT NULL,
                isfile          BOOLEAN NOT NULL,
                hash            TEXT,

                FOREIGN KEY (package) REFERENCES packages(id) ON UPDATE CASCADE ON DELETE CASCADE
            )",
            (),
        )?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS registry (
                reg_key         TEXT PRIMARY KEY NOT NULL,
                reg_value       TEXT
            )",
            (),
        )?;

        match self.reg_get::<usize>("db_version")? {
            None => {
                info!("Database version not set, assuming create, setting to newest version");
                self.reg_set("db_version", &1)?;
            }
            Some(version) => {
                info!("Database version: {} - no upgrade required", version);
            }
        }

        Ok(())
    }
}
