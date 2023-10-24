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

    /// Returns the directory where leaf should put its caches (overrides the `root_dir` default)
    ///
    /// Default: `root_dir/var/cache/leaf`
    pub fn get_cache_dir(&self) -> PathBuf {
        match &self.cache_dir {
            Some(p) => PathBuf::from(p),
            None => self.get_root().join("var").join("cache").join("leaf"),
        }
    }

    /// Returns the directory where leaf should put its download caches (overrides the `cache_dir` default)
    ///
    /// /// Default: `cache_dir/download`
    pub fn get_download_dir(&self) -> PathBuf {
        match &self.download_dir {
            Some(p) => PathBuf::from(p),
            None => self.get_cache_dir().join("download"),
        }
    }

    /// Returns the directory where leaf should put its package caches (overrides the `cache_dir` default)
    ///
    /// Default: `cache_dir/package`
    pub fn get_packages_dir(&self) -> PathBuf {
        match &self.packages_dir {
            Some(p) => PathBuf::from(p),
            None => self.get_cache_dir().join("package"),
        }
    }
}
