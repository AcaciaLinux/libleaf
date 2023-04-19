use std::path::PathBuf;

use super::Config;

impl Config {
    /// Returns the path to expect the config file at
    ///
    /// Default: `config_dir/leaf.conf`
    pub fn get_config_file(&self) -> PathBuf {
        match &self.config_file {
            Some(p) => PathBuf::from(p),
            None => self.get_config_dir().join("leaf.conf"),
        }
    }
}
