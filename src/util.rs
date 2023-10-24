use serde::Deserialize;

use crate::error::LError;
use log::info;
use std::path::PathBuf;

/// Ensures a directory exists
/// # Arguments
/// * `dir` - The directory to ensure
pub fn ensure_dir(dir: &PathBuf) -> Result<(), LError> {
    if !dir.exists() {
        info!("Creating missing directory {}", dir.to_str().unwrap_or(""));
        std::fs::create_dir_all(dir)?;
    }

    Ok(())
}

/// Deserializes a integer from a string using serde
pub fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr + serde::Deserialize<'de>,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    #[derive(serde::Deserialize)]
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
