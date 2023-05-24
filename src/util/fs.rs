//! This module groups utility functions for interacting with the filesystem
use crate::error::{LError, LErrorExt};
use std::{
    path::{Path, PathBuf},
    slice::Iter,
};

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

/// Copies the supplied iterator of FSEntries from the `src` directory to the `dest`
/// directory recursively iterating over all the children.
///
/// The provided arguments must be mutable due to them getting modified during the
/// copy process.
/// Once the function exits they are restored to their original value.
/// # Arguments
/// * `src` - The source root directory
/// * `dest` - The destination root directory
/// * `iter` - The iterator of FSEntries to copy
/// # Example
/// ```
/// use leaf::util::fs::*;
/// use std::path::PathBuf;
///
/// //Create an entry for testing
/// let entry = FSEntry {
///     name: "test".to_string(),
///     hash: None,
///     children: Vec::new()
/// };
///
/// // The entry must be wrapped in a iterator
/// let mut entries: Vec<FSEntry> = vec![entry];
///
/// // The source and destination
/// let mut src = PathBuf::from("./src");
/// let mut dest = PathBuf::from("./dest");
///
/// copy_recursive(&mut src, &mut dest, &mut entries.iter()).unwrap();
/// ```
pub fn copy_recursive(
    src: &mut PathBuf,
    dest: &mut PathBuf,
    iter: &mut Iter<FSEntry>,
) -> Result<(), LError> {
    for entry in iter {
        src.push(&entry.name);
        dest.push(&entry.name);

        if entry.hash.is_none() {
            // If the destination directory does not exist, create it
            if !dest.exists() {
                trace!("Creating directory {}", dest.to_string_lossy());
                std::fs::create_dir_all(&dest).err_append(&format!(
                    "When creating directory {}",
                    dest.to_string_lossy()
                ))?;
            }

            // And copy the directory contents, too
            copy_recursive(src, dest, &mut entry.children.iter())?;
        } else if !dest.exists() {
            // If the source is a symlink, create it in the destination
            if src.is_symlink() {
                let symlink_dest = src.read_link()?;
                trace!(
                    "Creating symlink {} pointing to {}",
                    dest.to_string_lossy(),
                    symlink_dest.to_string_lossy()
                );
                std::os::unix::fs::symlink(symlink_dest, &dest)?;
            } else {
                // Else just copy the file
                trace!(
                    "Copying {} ==> {}",
                    src.to_string_lossy(),
                    dest.to_string_lossy()
                );
                std::fs::copy(&src, &dest)?;
            }
        } else {
            // If the file does already exist, continue
            warn!("TODO: Create handler for existing files");
            error!("FSEntry {} does already exist", &dest.to_string_lossy());
            return Err(LError::new(
                crate::error::LErrorClass::IO(std::io::ErrorKind::AlreadyExists),
                &dest.to_string_lossy(),
            ));
        }

        src.pop();
        dest.pop();
    }
    Ok(())
}
