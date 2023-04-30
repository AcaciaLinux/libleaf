use super::Config;
use crate::{
    error::LError,
    mirror::Mirror,
    package::{local::LocalPackage, *},
    util::get_root_packages,
};
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

/// Updates the mirrors in the provided config
/// # Arguments
/// * `config` - The config to process
pub fn update(config: &Config, mirrors: &Vec<Mirror>) -> Result<(), Vec<LError>> {
    let pool = ThreadPool::new(config.download_workers);

    let errors: Arc<Mutex<Vec<LError>>> = Arc::new(Mutex::new(vec![]));

    for mirror in mirrors {
        let config = config.clone();
        let mirror = mirror.clone();
        let errors = errors.clone();
        pool.execute(move || match mirror.update(&config) {
            Ok(_) => (),
            Err(e) => {
                errors.lock().expect("Lock errors mutex").push(e);
            }
        })
    }

    pool.join();

    let errors = Arc::try_unwrap(errors)
        .expect("Move out of errors mutex")
        .into_inner()
        .expect("Move out of errors mutex");

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Installs the provided packages using the provided mirrors and config
/// # Arguments
/// * `config` - The configuration to use
/// * `packages` - The package names to install
/// * `mirrors` - The mirrors to search for the packages
pub fn install(config: &Config, packages: &[String], mirrors: &mut [Mirror]) -> Result<(), LError> {
    load_mirrors(config, mirrors)?;

    let mut pool: Vec<Arc<PackageVariant>> = Vec::new();

    for package in packages {
        let package = crate::mirror::resolve_package(package, mirrors)?;
        crate::util::resolve_dependencies(package, &mut pool, mirrors)?;
    }

    let results = download_packages(config, &pool);
    let mut local_packages: Vec<Arc<PackageVariant>> = Vec::new();
    for result in results {
        match result {
            Ok(res) => {
                local_packages.push(Arc::new(PackageVariant::Local(res.as_ref().clone())));
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    let root_packages = get_root_packages(&local_packages);

    for pkg in root_packages {
        let pkg = crate::util::resolve_dependencies_pool(pkg, &local_packages)?;
        pkg.get_local()?.deploy(config)?;
    }

    Ok(())
}

/// Loads the cached mirror file of every mirror
/// # Arguments
/// * `config` - The configuration to use
/// * `mirrors` - The mirrors to update
pub fn load_mirrors(config: &Config, mirrors: &mut [Mirror]) -> Result<(), LError> {
    for mirror in mirrors.iter_mut() {
        mirror.load(config)?;
    }

    Ok(())
}

pub fn download_packages(
    config: &Config,
    packages: &Vec<Arc<PackageVariant>>,
) -> Vec<Result<Arc<LocalPackage>, LError>> {
    let pool = ThreadPool::new(config.download_workers);
    type Return = Vec<Result<Arc<LocalPackage>, LError>>;
    let results: Arc<Mutex<Return>> = Arc::new(Mutex::new(vec![]));

    for package in packages {
        let config = config.clone();
        let package = package.clone();
        let results = results.clone();
        pool.execute(move || match &package.get_remote() {
            Ok(package) => {
                let res = package.fetch(&config);
                results.lock().expect("Lock results mutex").push(match res {
                    Ok(res) => Ok(Arc::new(res)),
                    Err(e) => Err(e),
                });
            }
            Err(e) => {
                results
                    .lock()
                    .expect("Lock results mutex")
                    .push(Err(e.clone()));
            }
        })
    }

    pool.join();

    Arc::try_unwrap(results)
        .expect("Move out of results mutex")
        .into_inner()
        .expect("Move out of results mutex")
}
