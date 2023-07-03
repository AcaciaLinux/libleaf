use crate::{
    actions::{install, update},
    config::Config,
    error::LError,
    mirror::Mirror,
    Leaf,
};

impl Leaf {
    /// Constructs a new Leaf handle using the supplied mirrors and a default config
    /// # Arguments
    /// * `mirrors` - The mirrors to use for operations
    pub fn new(mirrors: Vec<Mirror>) -> Self {
        Self {
            config: Config::default(),
            mirrors,
            pool: Vec::new(),
        }
    }

    /// Updates the local package index
    pub fn update(&mut self) -> Result<(), Vec<LError>> {
        update(&self.config, &mut self.mirrors)
    }

    /// Installs the supplied vector of packages
    /// # Arguments
    /// * `packages` - The packages to install
    pub fn install(&mut self, packages: &Vec<String>) -> Result<(), LError> {
        install(&self.config, packages, &mut self.mirrors, &mut self.pool)
    }

    /// Clears the internal pool of packages, forcing new resolving of installed,
    /// local and remote packages
    pub fn drop_pool(&mut self) {
        self.pool.clear()
    }
}
