//! Development utilities for testing.
//!
//! Provides test initialization and helper functions for Valkey/Redis operations.

use crate::{new_valkey_pool_with_config, ValkeyPool};
use tokio::sync::OnceCell;

/// Initialize test environment and return a ValkeyPool.
///
/// Usage in tests:
/// ```ignore
/// let pool = _dev_utils::init_test().await;
/// let mut conn = pool.get().await.unwrap();
/// ```
pub async fn init_test() -> ValkeyPool {
	static INIT: OnceCell<ValkeyPool> = OnceCell::const_new();

	let pool = INIT
		.get_or_init(|| async {
			// Use default local Redis/Valkey for testing
			let url =
				std::env::var("TEST_VALKEY_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
			new_valkey_pool_with_config(&url, 5, 1)
				.await
				.expect("Failed to create test pool")
		})
		.await;

	pool.clone()
}

/// Generate a unique test key prefix to isolate tests.
///
/// Usage:
/// ```ignore
/// let prefix = test_key_prefix("my_test");
/// let key = format!("{}:user:1", prefix);
/// ```
pub fn test_key_prefix(test_name: &str) -> String {
	use std::time::{SystemTime, UNIX_EPOCH};
	let ts = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_nanos();
	format!("test:{}:{}", test_name, ts)
}

/// Clean up all keys matching a pattern.
///
/// **Warning**: Uses KEYS command, only for testing.
pub async fn clean_test_keys<C>(conn: &mut C, pattern: &str) -> crate::Result<usize>
where
	C: redis::aio::ConnectionLike + Send,
{
	let keys: Vec<String> = redis::cmd("KEYS").arg(pattern).query_async(conn).await?;

	if keys.is_empty() {
		return Ok(0);
	}

	let count = keys.len();
	let mut cmd = redis::cmd("DEL");
	for key in &keys {
		cmd.arg(key);
	}
	cmd.query_async::<()>(conn).await?;

	Ok(count)
}

/// Clean up keys with a specific prefix.
pub async fn clean_test_prefix<C>(conn: &mut C, prefix: &str) -> crate::Result<usize>
where
	C: redis::aio::ConnectionLike + Send,
{
	clean_test_keys(conn, &format!("{}*", prefix)).await
}
