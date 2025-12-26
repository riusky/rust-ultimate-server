//! String commands: GET, SET, MGET, MSET, INCR, DECR

use crate::Result;
use redis::AsyncCommands;

/// Set a key-value pair with optional expiration (in seconds)
pub async fn set<C, K, V>(conn: &mut C, key: K, value: V, expire_secs: Option<u64>) -> Result<()>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	if let Some(secs) = expire_secs {
		conn.set_ex::<_, _, ()>(key, value, secs).await?;
	} else {
		conn.set::<_, _, ()>(key, value).await?;
	}
	Ok(())
}

/// Get a value by key
pub async fn get<C, K, V>(conn: &mut C, key: K) -> Result<Option<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let value: Option<V> = conn.get(key).await?;
	Ok(value)
}

/// Get a value, return error if not found
pub async fn get_required<C, K, V>(conn: &mut C, key: K) -> Result<V>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync + Clone + ToString,
	V: redis::FromRedisValue,
{
	let value: Option<V> = conn.get(key.clone()).await?;
	value.ok_or_else(|| crate::Error::KeyNotFound {
		key: key.to_string(),
	})
}

/// Set multiple key-value pairs
pub async fn mset<C, K, V>(conn: &mut C, pairs: &[(K, V)]) -> Result<()>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let mut cmd = redis::cmd("MSET");
	for (k, v) in pairs {
		cmd.arg(k).arg(v);
	}
	cmd.query_async::<()>(conn).await?;
	Ok(())
}

/// Get multiple values by keys
pub async fn mget<C, K, V>(conn: &mut C, keys: &[K]) -> Result<Vec<Option<V>>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let values: Vec<Option<V>> = conn.mget(keys).await?;
	Ok(values)
}

/// Increment a key by 1
pub async fn incr<C, K>(conn: &mut C, key: K) -> Result<i64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let value: i64 = conn.incr(key, 1).await?;
	Ok(value)
}

/// Increment a key by a specific amount
pub async fn incr_by<C, K>(conn: &mut C, key: K, delta: i64) -> Result<i64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let value: i64 = conn.incr(key, delta).await?;
	Ok(value)
}

/// Decrement a key by 1
pub async fn decr<C, K>(conn: &mut C, key: K) -> Result<i64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let value: i64 = conn.decr(key, 1).await?;
	Ok(value)
}

/// Set if not exists
pub async fn set_nx<C, K, V>(conn: &mut C, key: K, value: V) -> Result<bool>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let result: bool = conn.set_nx(key, value).await?;
	Ok(result)
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
	async fn test_set_get_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_set_get");
		let key = format!("{}:name", prefix);
		let fx_value = "test_value_01";

		// -- Exec
		set(&mut *conn, &key, fx_value, None).await?;
		let value: Option<String> = get(&mut *conn, &key).await?;

		// -- Check
		assert_eq!(value, Some(fx_value.to_string()));

		// -- Clean
		let count = clean_test_prefix(&mut *conn, &prefix).await?;
		assert_eq!(count, 1, "Should have cleaned 1 key");

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_set_with_expiration_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_set_exp");
		let key = format!("{}:temp", prefix);
		let fx_value = "temp_value";

		// -- Exec
		set(&mut *conn, &key, fx_value, Some(60)).await?;
		let value: Option<String> = get(&mut *conn, &key).await?;

		// -- Check
		assert_eq!(value, Some(fx_value.to_string()));

		// Check TTL is set
		let ttl: i64 = redis::cmd("TTL").arg(&key).query_async(&mut *conn).await?;
		assert!(ttl > 0 && ttl <= 60, "TTL should be between 1 and 60");

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_incr_decr_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_incr");
		let key = format!("{}:counter", prefix);

		// -- Exec & Check
		// Start with 0, increment to 1
		let val = incr(&mut *conn, &key).await?;
		assert_eq!(val, 1);

		// Increment by 5
		let val = incr_by(&mut *conn, &key, 5).await?;
		assert_eq!(val, 6);

		// Decrement by 1
		let val = decr(&mut *conn, &key).await?;
		assert_eq!(val, 5);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_set_nx_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_setnx");
		let key = format!("{}:unique", prefix);

		// -- Exec & Check
		// First set should succeed
		let result = set_nx(&mut *conn, &key, "first").await?;
		assert!(result, "First set_nx should succeed");

		// Second set should fail (key exists)
		let result = set_nx(&mut *conn, &key, "second").await?;
		assert!(!result, "Second set_nx should fail");

		// Value should be "first"
		let value: Option<String> = get(&mut *conn, &key).await?;
		assert_eq!(value, Some("first".to_string()));

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}
}

// endregion: --- Tests
