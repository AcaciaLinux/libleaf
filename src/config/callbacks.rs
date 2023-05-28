use std::path::Path;

use super::Config;

/// A struct containing all the callbacks that leaf can make
#[derive(Clone, Default)]
pub struct Callbacks {
    /// The callback for the case that a file does already exist but is not
    /// installed by leaf
    pub cb_file_exists: Option<fn(&Config, &Path) -> bool>,
}

impl std::fmt::Debug for Callbacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Callbacks (Not much to debug...)").finish()
    }
}

impl Callbacks {
    /// Calls the matching callback for this function if set, else the default
    pub fn file_exists(&self, config: &Config, path: &Path) -> bool {
        match self.cb_file_exists {
            Some(cb) => cb(config, path),
            None => config.force.unwrap_or(false),
        }
    }
}
