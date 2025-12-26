//! Key commands: DEL, EXISTS, EXPIRE, TTL, KEYS

use crate::Result;
use redis::AsyncCommands;

/// Delete one or more keys
pub async fn del<C, K>(conn: &mut C, keys: &[K]) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let count: u64 = conn.del(keys).await?;
	Ok(count)
}

/// Delete a single key
pub async fn del_one<C, K>(conn: &mut C, key: K) -> Result<bool>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let count: u64 = conn.del(key).await?;
	Ok(count > 0)
}

/// Check if key exists
pub async fn exists<C, K>(conn: &mut C, key: K) -> Result<bool>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let exists: bool = conn.exists(key).await?;
	Ok(exists)
}

/// Set key expiration in seconds
pub async fn expire<C, K>(conn: &mut C, key: K, seconds: u64) -> Result<bool>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let result: bool = conn.expire(key, seconds as i64).await?;
	Ok(result)
}

/// Get key TTL in seconds (-1 if no expiry, -2 if key doesn't exist)
pub async fn ttl<C, K>(conn: &mut C, key: K) -> Result<i64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let ttl: i64 = conn.ttl(key).await?;
	Ok(ttl)
}

/// Find keys matching pattern (use with caution in production)
pub async fn keys<C>(conn: &mut C, pattern: &str) -> Result<Vec<String>>
where
	C: AsyncCommands,
{
	let keys: Vec<String> = conn.keys(pattern).await?;
	Ok(keys)
}

/// Rename a key
pub async fn rename<C, K1, K2>(conn: &mut C, old_key: K1, new_key: K2) -> Result<()>
where
	C: AsyncCommands,
	K1: redis::ToRedisArgs + Send + Sync,
	K2: redis::ToRedisArgs + Send + Sync,
{
	conn.rename::<_, _, ()>(old_key, new_key).await?;
	Ok(())
}

/// Set key expiration at Unix timestamp
pub async fn expire_at<C, K>(conn: &mut C, key: K, timestamp: i64) -> Result<bool>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let result: bool = conn.expire_at(key, timestamp).await?;
	Ok(result)
}

/// Remove expiration from key
pub async fn persist<C, K>(conn: &mut C, key: K) -> Result<bool>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let result: bool = conn.persist(key).await?;
	Ok(result)
}

/// Get key type
pub async fn key_type<C, K>(conn: &mut C, key: K) -> Result<String>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let key_type: String = redis::cmd("TYPE")
		.arg(key)
		.query_async(conn)
		.await?;
	Ok(key_type)
}

/// Scan keys with cursor (for large datasets)
pub async fn scan<C>(
	conn: &mut C,
	cursor: u64,
	pattern: Option<&str>,
	count: Option<usize>,
) -> Result<(u64, Vec<String>)>
where
	C: AsyncCommands,
{
	let mut cmd = redis::cmd("SCAN");
	cmd.arg(cursor);

	if let Some(p) = pattern {
		cmd.arg("MATCH").arg(p);
	}
	if let Some(c) = count {
		cmd.arg("COUNT").arg(c);
	}

	let (next_cursor, keys): (u64, Vec<String>) = cmd.query_async(conn).await?;
	Ok((next_cursor, keys))
}

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>;

	use super::*;
	use crate::_dev_utils::{clean_test_prefix, init_test, test_key_prefix};
	use crate::commands::string;
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_del_exists_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_key");
		let key = format!("{}:item", prefix);

		// -- Exec
		string::set(&mut *conn, &key, "value", None).await?;
		assert!(exists(&mut *conn, &key).await?);

		let deleted = del_one(&mut *conn, &key).await?;
		assert!(deleted);
		assert!(!exists(&mut *conn, &key).await?);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_expire_ttl_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_expire");
		let key = format!("{}:data", prefix);

		// -- Exec
		string::set(&mut *conn, &key, "value", None).await?;
		
		// No expiry set
		let t = ttl(&mut *conn, &key).await?;
		assert_eq!(t, -1);

		// Set expiry
		expire(&mut *conn, &key, 300).await?;
		let t = ttl(&mut *conn, &key).await?;
		assert!(t > 0 && t <= 300);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_rename_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_rename");
		let old_key = format!("{}:old", prefix);
		let new_key = format!("{}:new", prefix);

		// -- Exec
		string::set(&mut *conn, &old_key, "value", None).await?;
		rename(&mut *conn, &old_key, &new_key).await?;

		// -- Check
		assert!(!exists(&mut *conn, &old_key).await?);
		assert!(exists(&mut *conn, &new_key).await?);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_key_type_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_type");
		let key = format!("{}:str", prefix);

		// -- Exec
		string::set(&mut *conn, &key, "value", None).await?;
		let t = key_type(&mut *conn, &key).await?;

		// -- Check
		assert_eq!(t, "string");

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}
}

// endregion: --- Tests
