//! lib-valkey-core - Valkey/Redis client library for the application.
//!
//! Provides connection pooling, common commands, and utilities like distributed locks.

// region:    --- Modules

mod config;
mod error;
mod pool;
pub mod transaction;

pub mod commands;
pub mod utils;

#[cfg(feature = "with-axum")]
pub mod extractor;

// For tests and development
#[cfg(test)]
pub mod _dev_utils;

// endregion: --- Modules

pub use self::config::{valkey_config, ValkeyConfig};
pub use self::error::{Error, Result};
pub use self::pool::{new_valkey_pool, new_valkey_pool_with_config, ValkeyConnection, ValkeyManager, ValkeyPool};

#[cfg(feature = "with-axum")]
pub use self::extractor::ValkeyConn;

// Re-export redis types for convenience
pub use redis::{AsyncCommands, RedisError, Value as RedisValue};
