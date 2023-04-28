use super::Config;
use crate::{
    error::LError,
    mirror::Mirror,
    package::{local::LocalPackage, *},
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

pub fn download_packages(
    config: &Config,
    packages: &Vec<Arc<PackageVariant>>,
) -> Vec<Result<LocalPackage, LError>> {
    let pool = ThreadPool::new(config.download_workers);
    let results: Arc<Mutex<Vec<Result<LocalPackage, LError>>>> = Arc::new(Mutex::new(vec![]));

    for package in packages {
        let config = config.clone();
        let package = package.clone();
        let results = results.clone();
        pool.execute(move || match package.get_remote() {
            Ok(package) => {
                let res = package.fetch(&config);
                results.lock().expect("Lock results mutex").push(res);
            }
            Err(e) => {
                results.lock().expect("Lock results mutex").push(Err(e));
            }
        })
    }

    pool.join();

    Arc::try_unwrap(results)
        .expect("Move out of results mutex")
        .into_inner()
        .expect("Move out of results mutex")
}
