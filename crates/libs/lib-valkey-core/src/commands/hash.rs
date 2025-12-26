//! Hash commands: HGET, HSET, HDEL, HGETALL, HMSET

use crate::Result;
use redis::AsyncCommands;
use std::collections::HashMap;

/// Set a hash field
pub async fn hset<C, K, F, V>(conn: &mut C, key: K, field: F, value: V) -> Result<()>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	F: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	conn.hset::<_, _, _, ()>(key, field, value).await?;
	Ok(())
}

/// Get a hash field
pub async fn hget<C, K, F, V>(conn: &mut C, key: K, field: F) -> Result<Option<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	F: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let value: Option<V> = conn.hget(key, field).await?;
	Ok(value)
}

/// Delete hash fields
pub async fn hdel<C, K, F>(conn: &mut C, key: K, fields: &[F]) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	F: redis::ToRedisArgs + Send + Sync,
{
	let count: u64 = conn.hdel(key, fields).await?;
	Ok(count)
}

/// Get all hash fields and values
pub async fn hgetall<C, K, F, V>(conn: &mut C, key: K) -> Result<HashMap<F, V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	F: redis::FromRedisValue + Eq + std::hash::Hash,
	V: redis::FromRedisValue,
{
	let map: HashMap<F, V> = conn.hgetall(key).await?;
	Ok(map)
}

/// Set multiple hash fields
pub async fn hmset<C, K, F, V>(conn: &mut C, key: K, fields: &[(F, V)]) -> Result<()>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	F: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	conn.hset_multiple::<_, _, _, ()>(key, fields).await?;
	Ok(())
}

/// Check if hash field exists
pub async fn hexists<C, K, F>(conn: &mut C, key: K, field: F) -> Result<bool>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	F: redis::ToRedisArgs + Send + Sync,
{
	let exists: bool = conn.hexists(key, field).await?;
	Ok(exists)
}

/// Increment hash field by integer
pub async fn hincrby<C, K, F>(conn: &mut C, key: K, field: F, delta: i64) -> Result<i64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	F: redis::ToRedisArgs + Send + Sync,
{
	let value: i64 = conn.hincr(key, field, delta).await?;
	Ok(value)
}

/// Get hash length (number of fields)
pub async fn hlen<C, K>(conn: &mut C, key: K) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let len: u64 = conn.hlen(key).await?;
	Ok(len)
}

/// Get all hash field names
pub async fn hkeys<C, K, F>(conn: &mut C, key: K) -> Result<Vec<F>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	F: redis::FromRedisValue,
{
	let keys: Vec<F> = conn.hkeys(key).await?;
	Ok(keys)
}

/// Get all hash values
pub async fn hvals<C, K, V>(conn: &mut C, key: K) -> Result<Vec<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let vals: Vec<V> = conn.hvals(key).await?;
	Ok(vals)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>;

	use super::*;
	use crate::_dev_utils::{clean_test_prefix, init_test, test_key_prefix};
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_hset_hget_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_hash");
		let key = format!("{}:user", prefix);

		// -- Exec
		hset(&mut *conn, &key, "name", "Alice").await?;
		hset(&mut *conn, &key, "age", "30").await?;

		let name: Option<String> = hget(&mut *conn, &key, "name").await?;
		let age: Option<String> = hget(&mut *conn, &key, "age").await?;

		// -- Check
		assert_eq!(name, Some("Alice".to_string()));
		assert_eq!(age, Some("30".to_string()));

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_hgetall_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_hgetall");
		let key = format!("{}:product", prefix);

		// -- Exec
		hset(&mut *conn, &key, "name", "Widget").await?;
		hset(&mut *conn, &key, "price", "99").await?;

		let map: HashMap<String, String> = hgetall(&mut *conn, &key).await?;

		// -- Check
		assert_eq!(map.len(), 2);
		assert_eq!(map.get("name"), Some(&"Widget".to_string()));
		assert_eq!(map.get("price"), Some(&"99".to_string()));

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_hincrby_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_hincrby");
		let key = format!("{}:stats", prefix);

		// -- Exec & Check
		let val = hincrby(&mut *conn, &key, "views", 1).await?;
		assert_eq!(val, 1);

		let val = hincrby(&mut *conn, &key, "views", 10).await?;
		assert_eq!(val, 11);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_hdel_hexists_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_hdel");
		let key = format!("{}:data", prefix);

		// -- Exec
		hset(&mut *conn, &key, "field1", "value1").await?;
		hset(&mut *conn, &key, "field2", "value2").await?;

		assert!(hexists(&mut *conn, &key, "field1").await?);

		let deleted = hdel(&mut *conn, &key, &["field1"]).await?;
		assert_eq!(deleted, 1);

		assert!(!hexists(&mut *conn, &key, "field1").await?);
		assert!(hexists(&mut *conn, &key, "field2").await?);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}
}

// endregion: --- Tests
