pub mod actions;
pub mod config;
pub mod db;
mod download;
pub mod error;
pub mod mirror;
pub mod package;
pub mod pbar;
pub mod util;

mod leaf;

#[macro_use]
extern crate log;

use package::PackageRef;

use crate::config::*;
use std::sync::atomic::AtomicBool;

pub static RUNNING: AtomicBool = AtomicBool::new(true);

#[macro_export]
macro_rules! usermsg {
    ($($arg:tt)*) => {
        $crate::pbar::println(format!($($arg)*).as_str()).expect("Failed to print above progress bar!")
    };
}

#[macro_export]
macro_rules! usererr {
    ($($arg:tt)*) => {
        $crate::pbar::println(format!("\x1B[1;91m{}\x1B[0m", format!($($arg)*))).expect("Failed to print above progress bar!")
    };
}

#[macro_export]
macro_rules! userwarn {
    ($($arg:tt)*) => {
        $crate::pbar::println(format!("\x1B[1;93m{}\x1B[0m", format!($($arg)*))).expect("Failed to print above progress bar!")
    };
}

/// This struct represents a leaf instance that can retain certain state
/// between invocations and function calls, serving as the main handle for
/// operations using the libleaf library
pub struct Leaf {
    /// The configuration to consult for all actions
    pub config: config::Config,

    /// The list of mirrors to consult for package resolving
    pub mirrors: Vec<mirror::Mirror>,

    /// A pool of resolved packages
    pool: Vec<PackageRef>,
}
