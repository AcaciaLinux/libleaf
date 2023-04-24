use super::Config;
use crate::error::LError;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

/// Updates the mirrors in the provided config
/// # Arguments
/// * `config` - The config to process
pub fn update(config: &Config) -> Result<(), Vec<LError>> {
    let pool = ThreadPool::new(config.download_workers);

    let errors: Arc<Mutex<Vec<LError>>> = Arc::new(Mutex::new(vec![]));

    for mirror in &config.mirrors {
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
