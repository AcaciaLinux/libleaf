//! This module groups utility functions for interacting with the filesystem
use crate::error::LError;
use std::path::Path;

use crate::util;

/// Represents a filesystem entry stored in the database
#[derive(Clone, Debug)]
pub struct FSEntry {
    /// The name of the entry without its path
    pub name: String,
    /// The hash for the file, None if directory
    pub hash: Option<String>,
    /// If this is a directory, the children are stored here
    pub children: Vec<FSEntry>,
}

impl FSEntry {
    /// Indexes the supplied path recursively to this filesystem entry if both are a directory
    /// creating new FSEntry objects on the way and building a filesystem tree
    /// # Arguments
    /// * `directory` - The directory to index into the filesystem entry
    fn index(&mut self, directory: &Path) -> Result<(), LError> {
        if self.hash.is_none() {
            self.children = index(directory)?;
        }
        Ok(())
    }
}

/// Indexes the supplied directory into a vector of FSEntries
///
/// The entries in the directory are not wrapped into a FSEntry, but rather
/// are returned in the `Vec<FSEntry>` result of this function
/// # Arguments
/// * `directory` - The directory to index recursively
pub fn index(directory: &Path) -> Result<Vec<FSEntry>, LError> {
    let mut res: Vec<FSEntry> = Vec::new();

    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        let isdir = path.is_dir();
        let mut new_entry = FSEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            children: Vec::new(),
            // Compute the hash:
            // directory:   None
            // link:        Target
            // file:        File
            hash: {
                if path.is_symlink() {
                    let target_path = path.read_link()?;
                    trace!(
                        "Symlink {} points to {}, hashing target",
                        path.to_string_lossy(),
                        target_path.to_string_lossy()
                    );
                    Some(util::hash::hash_str(&target_path.to_string_lossy()))
                } else if isdir {
                    None
                } else {
                    Some(util::hash::hash_file(&path)?)
                }
            },
        };

        new_entry.index(&path)?;
        res.push(new_entry);
    }

    Ok(res)
}
