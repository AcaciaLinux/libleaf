mod remote;
pub use remote::*;

mod local;
pub use local::*;

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

    /// Return the full name for the package, including the version and real_version
    fn full_name(&self) -> String {
        format!("{} {}-{}", self.name(), self.version(), self.real_version())
    }
}

#[derive(Debug)]
/// All possible package variants that can exist
pub enum PackageVariant {
    /// A remote package that can be fetched
    Remote(RemotePackage),
    /// A local package that can be installed
    Local(LocalPackage),
}

impl CorePackage for PackageVariant {
    fn name(&self) -> &str {
        match self {
            PackageVariant::Remote(p) => &p.name,
            PackageVariant::Local(p) => &p.name,
        }
    }

    fn version(&self) -> &str {
        match self {
            PackageVariant::Remote(p) => &p.version,
            PackageVariant::Local(p) => &p.version,
        }
    }

    fn real_version(&self) -> u64 {
        match self {
            PackageVariant::Remote(p) => p.real_version,
            PackageVariant::Local(p) => p.real_version,
        }
    }

    fn description(&self) -> &str {
        match self {
            PackageVariant::Remote(p) => &p.description,
            PackageVariant::Local(p) => &p.description,
        }
    }

    fn dependencies(&self) -> &Vec<String> {
        match self {
            PackageVariant::Remote(p) => &p.dependencies,
            PackageVariant::Local(p) => &p.dependencies,
        }
    }

    fn hash(&self) -> &str {
        match self {
            PackageVariant::Remote(p) => &p.hash,
            PackageVariant::Local(p) => &p.hash,
        }
    }
}
