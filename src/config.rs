pub mod dirs;
pub mod ensure_config;
pub mod files;

use std::path::PathBuf;

/// The configuration leaf should process
#[repr(C)]
#[derive(Clone)]
pub struct Config {
    /// The loglevel to use
    pub loglevel: LogLevel,

    /// How many parallel downloads should be performed if possible
    pub download_workers: usize,

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
#[derive(Clone, Copy, Default)]
pub enum LogLevel {
    #[default]
    Default,
    Verbose,
    Superverbose,
    Ultraverbose,
}
