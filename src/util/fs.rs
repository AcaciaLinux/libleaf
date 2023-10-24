//! This module groups utility functions for interacting with the filesystem
use crate::error::{LError, LErrorExt};
use log::{trace, warn};
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

    /// Appends the supplied string to create a recursive tree
    /// # Arguments
    /// * `depth` - The starting depth, should be 1 for a nice tree
    /// * `string` - A mutable reference to the string to append to
    pub fn print(&self, depth: usize, string: &mut String) {
        let msg = " |".repeat(depth);
        match &self.hash {
            Some(hash) => {
                let msg = format!("{}_ {}", &msg, self.name);
                string.push_str(&format!("\n{:.<50}{}", &msg, hash));
            }
            None => string.push_str(&format!("\n{}_ {}/", &msg, self.name)),
        }

        for child in &self.children {
            child.print(depth + 1, string);
        }
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
/// * `file_exists_handler` - A handler that gets called if the entry does already exist, true indicates overwrite
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
/// copy_recursive(&mut src, &mut dest, &mut entries.iter(), &|path| false).unwrap();
/// ```
pub fn copy_recursive<F>(
    src: &mut PathBuf,
    dest: &mut PathBuf,
    iter: &mut Iter<FSEntry>,
    file_exists_handler: &F,
) -> Result<(), LError>
where
    F: Fn(&Path) -> bool,
{
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
            copy_recursive(src, dest, &mut entry.children.iter(), file_exists_handler)?;
        } else {
            // If the destination exists, call the callback
            if dest.is_symlink() || dest.exists() {
                let can_overwrite = file_exists_handler(dest);
                if !can_overwrite {
                    return Err(LError::new(
                        crate::error::LErrorClass::IO(std::io::ErrorKind::AlreadyExists),
                        &dest.to_string_lossy(),
                    ));
                }
                warn!("Overwriting destination at {:?}", dest);
                std::fs::remove_file(&dest).err_prepend("When removing file")?;
            }

            // If the source is a symlink, create it in the destination
            if src.is_symlink() {
                let symlink_dest = src.read_link()?;
                let msg = format!(
                    "Creating symlink {} pointing to {}",
                    dest.to_string_lossy(),
                    symlink_dest.to_string_lossy()
                );
                trace!("{}", &msg);
                std::os::unix::fs::symlink(symlink_dest, &dest).err_append(&msg)?;
            } else {
                // Else just copy the file
                let msg = format!(
                    "Copying {} ==> {}",
                    src.to_string_lossy(),
                    dest.to_string_lossy()
                );
                trace!("{}", &msg);
                std::fs::copy(&src, &dest).err_append(&msg)?;
            }
        }

        src.pop();
        dest.pop();
    }
    Ok(())
}
