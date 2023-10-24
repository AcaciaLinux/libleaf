mod remote;
pub use remote::*;

/// This trait provides core information about a package
pub trait CorePackage {
    /// Return the name of the package
    fn name(&self) -> &str;
    /// Return the version string for the package
    fn version(&self) -> &str;
    /// Return the real version number for the package
    fn real_version(&self) -> u64;
    /// Return the description of the package
    fn description(&self) -> &str;
    /// Return the vector of dependencies for the package
    fn dependencies(&self) -> &Vec<String>;
    /// Return the hash value for the package
    fn hash(&self) -> &str;
}

#[derive(Debug)]
/// All possible package variants that can exist
pub enum PackageVariant {
    /// A remote package that can be fetched
    Remote(RemotePackage),
}

impl CorePackage for PackageVariant {
    fn name(&self) -> &str {
        match self {
            PackageVariant::Remote(p) => &p.name,
        }
    }

    fn version(&self) -> &str {
        match self {
            PackageVariant::Remote(p) => &p.version,
        }
    }

    fn real_version(&self) -> u64 {
        match self {
            PackageVariant::Remote(p) => p.real_version,
        }
    }

    fn description(&self) -> &str {
        match self {
            PackageVariant::Remote(p) => &p.description,
        }
    }

    fn dependencies(&self) -> &Vec<String> {
        match self {
            PackageVariant::Remote(p) => &p.dependencies,
        }
    }

    fn hash(&self) -> &str {
        match self {
            PackageVariant::Remote(p) => &p.hash,
        }
    }
}
