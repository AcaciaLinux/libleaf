use crate::{config::Config, error::LError, util};
use std::path::{Path, PathBuf};

use super::ConfigFile;

/// This is a buffer containing the absolute default for the config file
pub const DEFAULT_CONFIG: &str = include_str!("../../defconfig.conf");

/// Uses the supplied root to determine the best location for retrieving the config file.
/// If there is nothing available, this returns None.
///
/// First, this searches `/etc/leaf/leaf.conf`, if that does not exist, it searches for
/// `/lib/leaf/leaf.conf` and if even that does not exist, this returns None.
/// # Arguments
/// * `root` - The root to search in for the config file
pub fn get_config_file_path(root: &Path) -> Result<Option<PathBuf>, LError> {
    let config = Config {
        root: Some(PathBuf::from(root)),
        ..Default::default()
    };

    util::ensure_dirs(&config)?;

    let etc_conf_buf = config.get_config_file();
    let etc_conf = etc_conf_buf.as_path();
    let lib_conf_buf = config.get_config_file_lib();
    let lib_conf = lib_conf_buf.as_path();

    match etc_conf.exists() {
        true => Ok(Some(etc_conf_buf)),
        false => match lib_conf.exists() {
            true => Ok(Some(lib_conf_buf)),
            false => Ok(None),
        },
    }
}

/// Tries to load the config file from the supplied root directory
/// # Arguments
/// * `root` - The root to search for the the config file
pub fn load_config_file_from_root(root: &Path) -> Result<ConfigFile, LError> {
    let contents = match get_config_file_path(root)? {
        Some(path) => std::fs::read_to_string(path)?,
        None => DEFAULT_CONFIG.to_string(),
    };

    Ok(toml::from_str(&contents)?)
}

/// Parses the leaf config file from the supplied path
/// # Arguments
/// * `source` - The path to the config file
pub fn parse_config_file(source: &Path) -> Result<ConfigFile, LError> {
    let file_contents = match std::fs::read_to_string(source) {
        Ok(v) => v,
        Err(e) => {
            let mut error: LError = e.into();

            error.prepend(&format!(
                "When loading config file from {}",
                source.to_string_lossy()
            ));

            return Err(error);
        }
    };
    match toml::from_str(&file_contents) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.into()),
    }
}

impl Config {
    /// Returns the config file's location in the lib directory
    /// It gets consulted if the main config file is missing and provides a default
    ///
    /// Default: /lib/leaf/leaf.conf
    pub fn get_config_file_lib(&self) -> PathBuf {
        self.get_lib_dir().join("leaf.conf")
    }
}
