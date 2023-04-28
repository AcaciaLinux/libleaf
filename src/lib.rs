pub mod actions;
pub mod config;
pub mod db;
mod download;
pub mod error;
pub mod mirror;
pub mod package;
mod pbar;
pub mod util;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

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
