use serde::Deserialize;
use std::path::PathBuf;

use self::callbacks::Callbacks;

mod callbacks;
mod dirs;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    /// The root leaf should operate on, default: `/`
    pub root: Option<PathBuf>,

    /// The directory leaf should store its configs in (default: `/etc/leaf/`)
    pub config_dir: Option<PathBuf>,
    /// The directory leaf should look for and store the mirror files (default: `/etc/leaf/mirrors/`)
    pub mirrors_dir: Option<PathBuf>,

    /// The directory leaf should store its caches in (default: `/var/cache/leaf/`)
    pub cache_dir: Option<PathBuf>,
    /// The directory leaf should use to cache its downloads (default: `/var/cache/leaf/download/`)
    pub download_dir: Option<PathBuf>,
    /// The directory leaf should use to cache its packages (default: `/var/cache/leaf/package/`)
    pub packages_dir: Option<PathBuf>,

    /// If leaf should work aggressively (overwrite files, etc...)
    pub force: Option<bool>,

    /// If a progress bar should be rendered or not
    #[serde(default = "default_render_bar")]
    pub render_bar: bool,

    /// Callbacks that leaf can use to inform the caller or to ask for input
    #[serde(skip)]
    pub callbacks: Callbacks,
}

/// Provides a default for the `render_bar` field
fn default_render_bar() -> bool {
    true
}
