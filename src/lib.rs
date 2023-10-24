mod util;

pub mod download;
pub mod error;
pub mod pbar;
pub mod package;

use std::sync::atomic::AtomicBool;
pub static RUNNING: AtomicBool = AtomicBool::new(true);

