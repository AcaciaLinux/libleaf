use super::Config;
use crate::{db::DBConnection, error::LError, mirror::Mirror, package::*, *};
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;
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

    //Create a database connection for looking up already installed packages
    let mut db_con = DBConnection::open(&config.get_config_dir().join("installed.db"))?;

    // Resolve dependencies into the pool
    let mut pool: Vec<PackageRef> = Vec::new();
    for package in packages {
        let package = crate::mirror::resolve_package(package, mirrors)?;
        crate::util::dependencies::resolve_dependencies(
            package,
            &mut pool,
            mirrors,
            &mut db_con.new_transaction()?,
        )?;
    }

    // Download the packages and update the pool
    let results = download_packages(config, &pool);
    for result in results {
        match result {
            Ok(res) => match pool.iter().find(|p| p.get_name() == res.get_name()) {
                Some(p) => {
                    let mut pkg = p.write().expect("Lock package mutex");
                    *pkg = res.read().expect("Lock results mutex").clone();
                }
                None => panic!(
                    "[BUG] Could not find downloaded package in pool anymore: {}",
                    res.get_fq_name()
                ),
            },
            Err(e) => {
                return Err(e);
            }
        }
    }

    // Now install the packages one after the other
    for package_ref in &pool {
        util::transaction::install_package(package_ref.clone(), config, &mut db_con)?;
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
    packages: &Vec<PackageRef>,
) -> Vec<Result<PackageRef, LError>> {
    let pool = ThreadPool::new(config.download_workers);
    type Return = Vec<Result<PackageRef, LError>>;
    let results: Arc<Mutex<Return>> = Arc::new(Mutex::new(vec![]));

    for package in packages {
        let config = config.clone();
        let results = results.clone();
        let package = package.clone();
        pool.execute(move || {
            let package = package.write().unwrap();
            match package.deref() {
                PackageVariant::Remote(package) => {
                    let res = package.fetch(&config);
                    results.lock().expect("Lock results mutex").push(match res {
                        Ok(res) => Ok(res),
                        Err(e) => Err(e),
                    });
                }
                PackageVariant::Installed(package) => {
                    info!(
                        "Skipping download of installed package {}",
                        package.get_fq_name()
                    );
                }
                _ => {}
            }
        })
    }

    pool.join();

    Arc::try_unwrap(results)
        .expect("Move out of results mutex")
        .into_inner()
        .expect("Move out of results mutex")
}
