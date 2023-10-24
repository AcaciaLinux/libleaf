pub mod download;
pub mod error;
pub mod pbar;

use std::sync::atomic::AtomicBool;
pub static RUNNING: AtomicBool = AtomicBool::new(true);

