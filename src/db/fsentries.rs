use crate::util::fs::FSEntry;

use super::*;

impl<'a> DBTransaction<'a> {
    /// Adds the supplied files to the parent owned by the supplied package
    ///
    /// # Arguments
    /// * `pkgid` - The package the files are owned by
    /// * `parent` - The id of the parent file id (None for root entry)
    /// * `files` - The files to add
    pub fn insert_files(
        &self,
        pkgid: i64,
        parent: Option<i64>,
        files: &[FSEntry],
    ) -> Result<(), LError> {
        let mut stmt = self
            .transaction
            .prepare("INSERT INTO fsentries (name, package, parent, hash) VALUES (?, ?, ?, ?)")?;

        for file in files {
            trace!("Inserting fsentry {}", &file.name);
            let parent = Some(stmt.insert([
                Some(file.name.clone()),
                Some(pkgid.to_string()),
                parent.map(|p| p.to_string()),
                file.hash.clone(),
            ])?);
            self.insert_files(pkgid, parent, &file.children)?;
        }

        Ok(())
    }
}
