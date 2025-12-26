//! List commands: LPUSH, RPUSH, LPOP, RPOP, LRANGE, LLEN, LINDEX, LSET, LREM, LTRIM

use crate::Result;
use redis::AsyncCommands;

/// Push values to the left of a list
pub async fn lpush<C, K, V>(conn: &mut C, key: K, values: &[V]) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let len: u64 = conn.lpush(key, values).await?;
	Ok(len)
}

/// Push a single value to the left of a list
pub async fn lpush_one<C, K, V>(conn: &mut C, key: K, value: V) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let len: u64 = conn.lpush(key, value).await?;
	Ok(len)
}

/// Push values to the right of a list
pub async fn rpush<C, K, V>(conn: &mut C, key: K, values: &[V]) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let len: u64 = conn.rpush(key, values).await?;
	Ok(len)
}

/// Push a single value to the right of a list
pub async fn rpush_one<C, K, V>(conn: &mut C, key: K, value: V) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let len: u64 = conn.rpush(key, value).await?;
	Ok(len)
}

/// Pop a value from the left of a list
pub async fn lpop<C, K, V>(conn: &mut C, key: K) -> Result<Option<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let value: Option<V> = conn.lpop(key, None).await?;
	Ok(value)
}

/// Pop multiple values from the left of a list
pub async fn lpop_count<C, K, V>(conn: &mut C, key: K, count: usize) -> Result<Vec<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let values: Vec<V> = conn.lpop(key, Some(std::num::NonZeroUsize::new(count).unwrap())).await?;
	Ok(values)
}

/// Pop a value from the right of a list
pub async fn rpop<C, K, V>(conn: &mut C, key: K) -> Result<Option<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let value: Option<V> = conn.rpop(key, None).await?;
	Ok(value)
}

/// Pop multiple values from the right of a list
pub async fn rpop_count<C, K, V>(conn: &mut C, key: K, count: usize) -> Result<Vec<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let values: Vec<V> = conn.rpop(key, Some(std::num::NonZeroUsize::new(count).unwrap())).await?;
	Ok(values)
}

/// Get a range of elements from a list
pub async fn lrange<C, K, V>(conn: &mut C, key: K, start: i64, stop: i64) -> Result<Vec<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let values: Vec<V> = conn.lrange(key, start as isize, stop as isize).await?;
	Ok(values)
}

/// Get the length of a list
pub async fn llen<C, K>(conn: &mut C, key: K) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let len: u64 = conn.llen(key).await?;
	Ok(len)
}

/// Get an element by index
pub async fn lindex<C, K, V>(conn: &mut C, key: K, index: i64) -> Result<Option<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let value: Option<V> = conn.lindex(key, index as isize).await?;
	Ok(value)
}

/// Set the value of an element by index
pub async fn lset<C, K, V>(conn: &mut C, key: K, index: i64, value: V) -> Result<()>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	redis::cmd("LSET")
		.arg(key)
		.arg(index)
		.arg(value)
		.query_async::<()>(conn)
		.await?;
	Ok(())
}

/// Remove elements equal to value
pub async fn lrem<C, K, V>(conn: &mut C, key: K, count: i64, value: V) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let removed: u64 = conn.lrem(key, count as isize, value).await?;
	Ok(removed)
}

/// Trim a list to the specified range
pub async fn ltrim<C, K>(conn: &mut C, key: K, start: i64, stop: i64) -> Result<()>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	conn.ltrim::<_, ()>(key, start as isize, stop as isize).await?;
	Ok(())
}

/// Blocking pop from the left of multiple lists
pub async fn blpop<C, K, V>(conn: &mut C, keys: &[K], timeout: f64) -> Result<Option<(String, V)>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let result: Option<(String, V)> = conn.blpop(keys, timeout).await?;
	Ok(result)
}

/// Blocking pop from the right of multiple lists
pub async fn brpop<C, K, V>(conn: &mut C, keys: &[K], timeout: f64) -> Result<Option<(String, V)>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let result: Option<(String, V)> = conn.brpop(keys, timeout).await?;
	Ok(result)
}

/// Move element from one list to another
pub async fn lmove<C, K, V>(
	conn: &mut C,
	source: K,
	destination: K,
	from_left: bool,
	to_left: bool,
) -> Result<Option<V>>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let wherefrom = if from_left { "LEFT" } else { "RIGHT" };
	let whereto = if to_left { "LEFT" } else { "RIGHT" };

	let value: Option<V> = redis::cmd("LMOVE")
		.arg(source)
		.arg(destination)
		.arg(wherefrom)
		.arg(whereto)
		.query_async(conn)
		.await?;
	Ok(value)
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
	async fn test_lpush_lpop_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_list");
		let key = format!("{}:queue", prefix);

		// -- Exec
		lpush_one(&mut *conn, &key, "first").await?;
		lpush_one(&mut *conn, &key, "second").await?;

		let val: Option<String> = lpop(&mut *conn, &key).await?;

		// -- Check (LPUSH adds to left, so "second" is first)
		assert_eq!(val, Some("second".to_string()));

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_rpush_rpop_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_rpush");
		let key = format!("{}:stack", prefix);

		// -- Exec
		rpush_one(&mut *conn, &key, "first").await?;
		rpush_one(&mut *conn, &key, "second").await?;

		let val: Option<String> = rpop(&mut *conn, &key).await?;

		// -- Check (RPUSH adds to right, RPOP gets from right)
		assert_eq!(val, Some("second".to_string()));

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_lrange_llen_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_lrange");
		let key = format!("{}:items", prefix);

		// -- Exec
		rpush(&mut *conn, &key, &["a", "b", "c", "d"]).await?;

		let len = llen(&mut *conn, &key).await?;
		assert_eq!(len, 4);

		let range: Vec<String> = lrange(&mut *conn, &key, 1, 2).await?;
		assert_eq!(range, vec!["b".to_string(), "c".to_string()]);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_lindex_lset_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_lindex");
		let key = format!("{}:data", prefix);

		// -- Exec
		rpush(&mut *conn, &key, &["x", "y", "z"]).await?;

		let val: Option<String> = lindex(&mut *conn, &key, 1).await?;
		assert_eq!(val, Some("y".to_string()));

		lset(&mut *conn, &key, 1, "Y").await?;
		let val: Option<String> = lindex(&mut *conn, &key, 1).await?;
		assert_eq!(val, Some("Y".to_string()));

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}
}

// endregion: --- Tests
