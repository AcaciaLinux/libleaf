//! This module groups utility functions for using the hashing algorithms
use md5::*;

use crate::error::LError;
use std::path::Path;

/// Computes the MD5 hash of the source string
/// # Arguments
/// * `source` - The string to hash
pub fn hash_str(source: &str) -> String {
    //Create the hasher and hash
    let mut hasher = Md5::new();
    hasher.update(source);

    //Finalize and compute base16 string
    let mut buf = [0u8; 32];
    let hash = hasher.finalize();
    let res = base16ct::lower::encode_str(&hash, &mut buf).expect("Convert hash to base16");

    trace!("Computed hash of string '{}': {}", source, res);

    res.to_string()
}

/// Computes the MD5 hash of the file supplied as source
/// # Arguments
/// * `source` - The source file to hash
pub fn hash_file(source: &Path) -> Result<String, LError> {
    //Open the file
    let mut file = std::fs::File::open(source)?;

    //Create the hasher and hash
    let mut hasher = Md5::new();
    std::io::copy(&mut file, &mut hasher)?;

    //Finalize and compute base16 string
    let mut buf = [0u8; 32];
    let hash = hasher.finalize();
    let res = base16ct::lower::encode_str(&hash, &mut buf).expect("Convert hash to base16");

    trace!(
        "Computed hash of file {}: {}",
        source.to_str().unwrap_or(""),
        res
    );

    Ok(res.to_owned())
}
