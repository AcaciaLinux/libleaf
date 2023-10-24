use std::path::{Path, PathBuf};

use super::Config;

impl Config {
    /// Return the root directory leaf should work in
    ///
    /// Default: `/`
    pub fn get_root(&self) -> &Path {
        match &self.root {
            Some(p) => &p,
            None => &Path::new("/"),
        }
    }

    /// Returns the directory where leaf should put its configs (overrides the `root_dir` default)
    ///
    /// Default: `root_dir/etc/leaf`
    pub fn get_config_dir(&self) -> PathBuf {
        match &self.config_dir {
            Some(p) => PathBuf::from(p),
            None => self.get_root().join("etc").join("leaf"),
        }
    }

    /// Returns the directory leaf should look for and store the mirror files (overrides the `config_dir` default)
    ///
    /// Default: `config_dir/mirrors`
    pub fn get_mirrors_dir(&self) -> PathBuf {
        match &self.mirrors_dir {
            Some(p) => PathBuf::from(p),
            None => self.get_config_dir().join("mirrors"),
        }
    }
}
