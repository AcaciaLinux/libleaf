pub mod dirs;
pub mod ensure_config;
pub mod files;

use serde::Deserialize;
use std::path::PathBuf;

/// The configuration leaf should process
#[repr(C)]
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    /// The loglevel to use
    #[serde(default)]
    pub loglevel: LogLevel,

    /// How many parallel downloads should be performed if possible
    #[serde(default = "default_download_workers")]
    pub download_workers: usize,

    /// If a progress bar should be rendered or not
    #[serde(default = "default_render_bar")]
    pub render_bar: bool,

    /// The root directory leaf should work on (default: `/`)
    pub root: Option<PathBuf>,

    /// The directory leaf should store its configs in (default: `/etc/leaf/`)
    pub config_dir: Option<PathBuf>,
    /// The location to search the config file at (default: `/etc/leaf/leaf.conf`)
    pub config_file: Option<PathBuf>,
    /// The directory leaf should look for and store the mirror files (default: `/etc/leaf/mirrors/`)
    pub mirrors_dir: Option<PathBuf>,

    /// The directory to search for lib files (deafult files) (default: `/lib/leaf`)
    pub lib_dir: Option<PathBuf>,

    /// The directory leaf should store its caches in (default: `/var/cache/leaf/`)
    pub cache_dir: Option<PathBuf>,
    /// The directory leaf should use to cache its downloads (default: `/var/cache/leaf/download/`)
    pub download_dir: Option<PathBuf>,
    /// The directory leaf should use to cache its packages (default: `/var/cache/leaf/package/`)
    pub packages_dir: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            loglevel: LogLevel::Default,
            download_workers: 5,
            render_bar: true,
            root: None,
            config_dir: None,
            config_file: None,
            mirrors_dir: None,
            lib_dir: None,
            cache_dir: None,
            download_dir: None,
            packages_dir: None,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, Deserialize)]
pub enum LogLevel {
    #[default]
    #[serde(rename(deserialize = "0"))]
    Default,
    #[serde(rename(deserialize = "1"))]
    Verbose,
    #[serde(rename(deserialize = "2"))]
    Superverbose,
    #[serde(rename(deserialize = "3"))]
    Ultraverbose,
}

/// Provides a default for the `download_workers` field
fn default_download_workers() -> usize {
    5
}

/// Provides a default for the `render_bar` field
fn default_render_bar() -> bool {
    true
}
