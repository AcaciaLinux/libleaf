use crate::config::*;
use crate::db::*;
use crate::error::*;
use crate::package::*;
use crate::usermsg;

/// Installs the provided package using the supplied configuration.
/// It recurses through all the dependencies and makes sure they are installed.
/// # Arguments
/// * `package` - The package to install
/// * `config` - The configuration to use for installing
/// * `db_con` - The database connection to use for installing
pub fn install_package(
    package: PackageRef,
    config: &Config,
    db_con: &mut DBConnection,
) -> Result<(), LError> {
    // Install the package
    install_package_rec(package.clone(), config, db_con)?;

    // Then add the dependencies to the database
    let package_read = package.read().expect("Lock package mutex");
    db_con.insert_package_dependencies(&package_read)?;

    Ok(())
}

/// The recursive version of install_package(). This gets called recursively
/// for all the dependencies.
fn install_package_rec(
    package: PackageRef,
    config: &Config,
    db_con: &mut DBConnection,
) -> Result<(), LError> {
    // Check if not already locked, if so, there is nothing to be done here
    match package.try_write() {
        Ok(package) => match package.clone() {
            PackageVariant::Installed(_) => return Ok(()), // An already installed package gets skipped
            PackageVariant::Local(_) => {}                 // We can install a local package
            PackageVariant::Remote(r) => {
                return Err(LError::new(
                    LErrorClass::UnexpectedPackageVariant,
                    &format!("Can't install remote package {}", r.get_fq_name()),
                ));
            }
        },
        Err(_) => return Ok(()),
    };

    // Before locking, insert the package
    let package_read = package.read().expect("Lock package mutex");
    db_con.insert_package(&package_read)?;
    drop(package_read);

    // Lock and ensure ensure dependencies
    let mut package_write = package.write().expect("Lock package mutex for writing");

    for dependency in package_write.get_dependencies().get_resolved()? {
        install_package_rec(dependency.clone(), config, db_con)?;
    }

    // Deploy the package
    let old_package = package_write.get_local()?.clone();
    usermsg!("Installing package {}", old_package.get_fq_name());
    *package_write = PackageVariant::Installed(old_package.deploy(config)?);

    drop(package_write);

    Ok(())
}
