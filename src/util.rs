//! Some utility functions to aid the management of the leaf package manager

use crate::{error::*, package::remote::*, Config};
use serde::{Deserialize, Deserializer};
use std::{fmt::Display, fs::create_dir_all, str::FromStr};

pub fn ensure_dirs(conf: &Config) -> Result<(), LError> {
    let dir = conf.get_mirrors_dir();

    if !dir.exists() {
        info!(
            "Creating missing mirrors directory {}",
            dir.to_str().unwrap_or("")
        );
        create_dir_all(dir)?
    }

    Ok(())
}

/// Searches for a Package with the supplied name in the provided Vec of packages
/// # Arguments
/// * `name` - The package name to search for
/// * `list` - A reference to the vector of Package to search
/// # Returns
/// A clone of the package found wrapped in a Option, None if nothing has been found
pub fn find_package<T: Package>(name: &str, list: &Vec<T>) -> Option<T> {
    for package in list {
        if package.get_name() == name {
            return Some(package.clone());
        }
    }

    None
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
