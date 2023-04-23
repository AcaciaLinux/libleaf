//! Some utility functions to aid the management of the leaf package manager

use crate::mirror::{self, Mirror};
use crate::{config::Config, error::*, package::remote::*};
use serde::{Deserialize, Deserializer};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::{fmt::Display, fs::create_dir_all, str::FromStr};
use tar::Archive;
use xz::read::XzDecoder;

fn ensure_dir(dir: &PathBuf) -> Result<(), LError> {
    if !dir.exists() {
        info!("Creating missing directory {}", dir.to_str().unwrap_or(""));
        create_dir_all(dir)?;
    }

    Ok(())
}

pub fn ensure_dirs(conf: &Config) -> Result<(), LError> {
    //Ensures /etc/leaf and /etc/leaf/mirrors
    ensure_dir(&conf.get_mirrors_dir())?;

    //Ensures /var/cache/leaf/ and /var/cache/leaf/download
    ensure_dir(&conf.get_download_dir())?;
    //Ensures /var/cache/leaf/package
    ensure_dir(&conf.get_packages_dir())?;

    Ok(())
}

/// Searches for a Package with the supplied name in the provided Vec of packages
/// # Arguments
/// * `name` - The package name to search for
/// * `list` - A reference to the vector of Package to search
/// # Returns
/// A reference to the package found wrapped in a Option, None if nothing has been found
pub fn find_package<'a, T: Package>(name: &str, list: &'a Vec<T>) -> Option<&'a T> {
    for package in list {
        if package.get_name() == name {
            return Some(package);
        }
    }

    None
}

/// Resolves the whole dependency tree of the package with the provided name
/// and sorts them in the order they should be installed in
/// # Arguments
/// * `package_name` - The name of the package to resolve
/// * `dependencies` - A Vec of RemotePackages where the tree gets put into
/// * `mirrors` - A Vec of Mirrors to search for the package and dependencies
pub fn resolve_dependencies<'a>(
    package_name: &str,
    dependencies: &'a mut Vec<RemotePackage>,
    mirrors: &'a Vec<Arc<Mutex<Mirror>>>,
) -> Result<(), LError> {
    //Try resolving the package
    let package = mirror::resolve_package(package_name, mirrors)?;

    // Check if this package hasn't already been resolved
    if find_package(package_name, &dependencies).is_some() {
        trace!(
            "[resolver] Skipping dependency resolving of package {}",
            package.get_full_name()
        );
        return Ok(());
    }

    //Push the package to prevent double resolving
    dependencies.push(package.clone());

    //Go through all dependencies and resolve them
    for dep in package.get_dependencies() {
        resolve_dependencies(dep.as_str(), dependencies, mirrors)?;
    }

    //Move the package back, it gets installed AFTER its dependencies
    match dependencies
        .iter()
        .position(|p| p.get_hash() == package.get_hash())
    {
        Some(pos) => {
            let dep = dependencies.remove(pos);
            dependencies.push(dep);
        }
        None => {}
    }

    Ok(())
}

/// Deserializes a integer from a string
pub fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
    }

    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
        StringOrInt::Number(i) => Ok(i),
    }
}

/// A convenient wrapper around the tar and xz libararies
/// # Arguments
/// * `source` - The path to the source tarball
/// * `destination` - The destination path to extract into
pub fn extract(source: &Path, destination: &Path) -> Result<(), LError> {
    let tar_file = File::open(source)?;
    let tar = XzDecoder::new(tar_file);

    let mut archive = Archive::new(tar);
    archive.set_overwrite(true);
    archive.unpack(destination)?;

    Ok(())
}
