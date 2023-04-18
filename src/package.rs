//! A package is the main work horse of the leaf package manager.
//! Every variant of a package implements the trait Package.

pub mod remote;

pub use serde::{de, Deserialize, Serialize};

/// This trait represents the common interface for all package variants, be it remote, local or installed
pub trait Package: Clone {
    /// Get the package name
    fn get_name(&self) -> String;
    /// Set the package name
    fn set_name(&mut self, name: &str);

    /// Get the package version string
    fn get_version(&self) -> String;
    /// Set the package version string
    fn set_version(&mut self, version: &str);

    /// Get the package real version
    fn get_real_version(&self) -> u64;
    /// Set the package real version
    fn set_real_version(&mut self, real_version: u64);

    /// Get a reference to the dependencies required by the package
    fn get_dependencies<'a>(&'a self) -> &'a Vec<String>;
    /// Set the dependencies needed by this package
    fn set_dependencies(&mut self, dependencies: Vec<String>);

    /// Get the package MD5 hash
    fn get_hash(&self) -> String;
    /// Set the package MD5 hash
    fn set_hash(&mut self, hash: &str);

    /// Get the full name for the package: <name>-<version> (E.g: glibc-2.3)
    fn get_full_name(&self) -> String;
}
