#[cfg(feature = "with-ts")]
extern crate self as lib_core;

pub mod config;
pub mod ctx;
pub mod model;

// #[cfg(test)] // Commented during early development.
pub mod _dev_utils;

use config::core_config;
