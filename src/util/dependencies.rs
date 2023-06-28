use crate::db::DBTransaction;
use crate::error::*;
use crate::mirror::*;
use crate::package::*;

/// Resolves the dependencies of the supplied package using the provided mirrors
/// into the provided pool
/// # Arguments
/// * `package` - The reference to the package to process
/// * `pool` - The pool to use for storing the packages
/// * `mirrors` - The mirrors to search for the package
pub fn resolve_dependencies(
    package: PackageRef,
    pool: &mut Vec<PackageRef>,
    mirrors: &[Mirror],
    db: &mut DBTransaction,
) -> Result<(), LError> {
    // First of all, check if the package isn't already resolved
    let hash = package.get_hash();
    if pool.iter().any(|p| p.get_hash() == hash) {
        trace!("Skipping resolved package {}", package.get_fq_name());
        return Ok(());
    }

    // Check if we have an already installed package at the system
    if let Some(package) = db.get_stub_package(&package.get_name(), pool)? {
        trace!("Found already installed package {}", package.get_name());
        return Ok(());
    }

    // Push the package for now to prevent double resolving
    pool.push(package.clone());

    // Go through the dependencies and resolve them too
    let pkg_fq_name = package.get_fq_name();
    let mut new_deps: Vec<PackageRef> = Vec::new();
    for dependency in package
        .read()
        .expect("Lock package mutex")
        .get_dependencies()
        .get_unresolved()?
    {
        debug!(
            "Resolving dependency {} of package {}",
            dependency, pkg_fq_name
        );

        // Check if the dependency does already exist
        match pool.iter().find(|p| &p.get_name() == dependency) {
            Some(p) => {
                // Reuse the dependency
                trace!("Reusing resolved dependency {}", p.get_name());
                new_deps.push(p.clone());
            }
            None => {
                // If the dependency is to be resolved, query the mirrors, resolve its dependencies and push it
                let dependency = resolve_package(dependency, mirrors)?;
                resolve_dependencies(dependency.clone(), pool, mirrors, db)?;
                new_deps.push(dependency);
            }
        }
    }

    // Add the new dependencies as resolved
    package
        .write()
        .expect("Lock package mutex")
        .set_dependencies(Dependencies::Resolved(new_deps));

    // Pull the package back
    let hash = &package.get_hash();
    match pool.iter().position(|p| &p.get_hash() == hash) {
        Some(pos) => {
            trace!("Pulling back package {:?}", package.get_name());
            let pkg = pool.remove(pos);
            pool.push(pkg);
            Ok(())
        }
        None => Err(LError::new(
            LErrorClass::PackageNotFound,
            format!("Package disappeared: {}", package.get_name()).as_str(),
        )),
    }
}

/// Extracts the packages from the pool that have no dependers
/// # Arguments
/// * `pool` - The pool to search
pub fn get_root_packages(pool: &[PackageRef]) -> Vec<PackageRef> {
    let mut root_packages: Vec<PackageRef> = Vec::new();

    for package in pool {
        if !pool.iter().any(|p| {
            package
                .read()
                .expect("Lock package mutex")
                .is_dependency_of(&p.read().unwrap().clone())
        }) {
            root_packages.push(package.clone());
        }
    }

    root_packages
}
