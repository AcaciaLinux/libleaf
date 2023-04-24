use super::Config;
use crate::mirror::Mirror;

impl Config {
    /// Adds the supplied mirror to the config if not already present
    /// # Arguments
    /// * `mirror` - The mirror to add
    pub fn add_mirror(&mut self, mirror: Mirror) {
        if !self.mirrors.contains(&mirror) {
            self.mirrors.push(mirror);
        }
    }

    /// Adds the supplied Vec of mirrors to the config if not already present
    /// # Arguments
    /// * `mirrors` - The mirrors to add
    pub fn add_mirrors(&mut self, mirrors: Vec<Mirror>) {
        for mirror in mirrors {
            self.add_mirror(mirror);
        }
    }
}
