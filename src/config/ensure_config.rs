use crate::{config::Config, error::LError, util};
use std::{fs::*, path::PathBuf};

pub const DEFAULT_CONFIG: &str = include_str!("../../defconfig.conf");

/// Checks if the config file is present.
/// If not it will ensure that a sensible default config is available at the default path.
pub fn ensure_config_file(config: &Config) -> Result<(), LError> {
    util::ensure_dirs(config)?;

    let etc_conf_buf = config.get_config_file();
    let etc_conf = etc_conf_buf.as_path();
    let lib_conf_buf = config.get_config_file_lib();
    let lib_conf = lib_conf_buf.as_path();

    if etc_conf.exists() {
        return Ok(());
    }

    if lib_conf.exists() {
        match copy(lib_conf, etc_conf) {
            Ok(_) => Ok(()),
            Err(e) => Err(LError::from(e)),
        }
    } else {
        match write(etc_conf, DEFAULT_CONFIG) {
            Ok(_) => Ok(()),
            Err(e) => Err(LError::from(e)),
        }
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
