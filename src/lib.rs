mod util;

pub mod config;

pub mod download;
pub mod error;
pub mod package;
pub mod pbar;

pub mod index;
pub mod mirror;

use std::sync::atomic::AtomicBool;
pub static RUNNING: AtomicBool = AtomicBool::new(true);
