//! Set commands: SADD, SREM, SMEMBERS, SISMEMBER, SCARD, SPOP, SRANDMEMBER, SDIFF, SINTER, SUNION

use crate::Result;
use redis::AsyncCommands;

/// Add members to a set
pub async fn sadd<C, K, V>(conn: &mut C, key: K, members: &[V]) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let added: u64 = conn.sadd(key, members).await?;
	Ok(added)
}

/// Add a single member to a set
pub async fn sadd_one<C, K, V>(conn: &mut C, key: K, member: V) -> Result<bool>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let added: u64 = conn.sadd(key, member).await?;
	Ok(added > 0)
}

/// Remove members from a set
pub async fn srem<C, K, V>(conn: &mut C, key: K, members: &[V]) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let removed: u64 = conn.srem(key, members).await?;
	Ok(removed)
}

/// Remove a single member from a set
pub async fn srem_one<C, K, V>(conn: &mut C, key: K, member: V) -> Result<bool>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let removed: u64 = conn.srem(key, member).await?;
	Ok(removed > 0)
}

/// Get all members of a set
pub async fn smembers<C, K, V>(conn: &mut C, key: K) -> Result<Vec<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let members: Vec<V> = conn.smembers(key).await?;
	Ok(members)
}

/// Check if a member exists in a set
pub async fn sismember<C, K, V>(conn: &mut C, key: K, member: V) -> Result<bool>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let exists: bool = conn.sismember(key, member).await?;
	Ok(exists)
}

/// Check if multiple members exist in a set
pub async fn smismember<C, K, V>(conn: &mut C, key: K, members: &[V]) -> Result<Vec<bool>>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let mut cmd = redis::cmd("SMISMEMBER");
	cmd.arg(key);
	for m in members {
		cmd.arg(m);
	}
	let results: Vec<i32> = cmd.query_async(conn).await?;
	Ok(results.into_iter().map(|v| v == 1).collect())
}

/// Get the number of members in a set
pub async fn scard<C, K>(conn: &mut C, key: K) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let count: u64 = conn.scard(key).await?;
	Ok(count)
}

/// Remove and return random members from a set
pub async fn spop<C, K, V>(conn: &mut C, key: K) -> Result<Option<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let member: Option<V> = conn.spop(key).await?;
	Ok(member)
}

/// Remove and return multiple random members from a set
pub async fn spop_count<C, K, V>(conn: &mut C, key: K, count: u64) -> Result<Vec<V>>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let members: Vec<V> = redis::cmd("SPOP")
		.arg(key)
		.arg(count)
		.query_async(conn)
		.await?;
	Ok(members)
}

/// Get random members from a set (without removing)
pub async fn srandmember<C, K, V>(conn: &mut C, key: K) -> Result<Option<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let member: Option<V> = conn.srandmember(key).await?;
	Ok(member)
}

/// Get multiple random members from a set (without removing)
pub async fn srandmember_count<C, K, V>(conn: &mut C, key: K, count: i64) -> Result<Vec<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let members: Vec<V> = conn.srandmember_multiple(key, count as usize).await?;
	Ok(members)
}

/// Get difference between sets
pub async fn sdiff<C, K, V>(conn: &mut C, keys: &[K]) -> Result<Vec<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let diff: Vec<V> = conn.sdiff(keys).await?;
	Ok(diff)
}

/// Get intersection of sets
pub async fn sinter<C, K, V>(conn: &mut C, keys: &[K]) -> Result<Vec<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let inter: Vec<V> = conn.sinter(keys).await?;
	Ok(inter)
}

/// Get union of sets
pub async fn sunion<C, K, V>(conn: &mut C, keys: &[K]) -> Result<Vec<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let union: Vec<V> = conn.sunion(keys).await?;
	Ok(union)
}

/// Store difference of sets in a new set
pub async fn sdiffstore<C, K>(conn: &mut C, dest: K, keys: &[K]) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let count: u64 = conn.sdiffstore(dest, keys).await?;
	Ok(count)
}

/// Store intersection of sets in a new set
pub async fn sinterstore<C, K>(conn: &mut C, dest: K, keys: &[K]) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let count: u64 = conn.sinterstore(dest, keys).await?;
	Ok(count)
}

/// Store union of sets in a new set
pub async fn sunionstore<C, K>(conn: &mut C, dest: K, keys: &[K]) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let count: u64 = conn.sunionstore(dest, keys).await?;
	Ok(count)
}

/// Move a member from one set to another
pub async fn smove<C, K, V>(conn: &mut C, source: K, dest: K, member: V) -> Result<bool>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let moved: bool = conn.smove(source, dest, member).await?;
	Ok(moved)
}

/// Scan set members
pub async fn sscan<C, K>(
	conn: &mut C,
	key: K,
	cursor: u64,
	pattern: Option<&str>,
	count: Option<usize>,
) -> Result<(u64, Vec<String>)>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let mut cmd = redis::cmd("SSCAN");
	cmd.arg(key).arg(cursor);

	if let Some(p) = pattern {
		cmd.arg("MATCH").arg(p);
	}
	if let Some(c) = count {
		cmd.arg("COUNT").arg(c);
	}

	let (next_cursor, members): (u64, Vec<String>) = cmd.query_async(conn).await?;
	Ok((next_cursor, members))
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
	async fn test_sadd_smembers_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_set");
		let key = format!("{}:tags", prefix);

		// -- Exec
		sadd(&mut *conn, &key, &["rust", "redis", "valkey"]).await?;

		let mut members: Vec<String> = smembers(&mut *conn, &key).await?;
		members.sort();

		// -- Check
		assert_eq!(members, vec!["redis", "rust", "valkey"]);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_sismember_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_sismember");
		let key = format!("{}:colors", prefix);

		// -- Exec
		sadd(&mut *conn, &key, &["red", "green", "blue"]).await?;

		// -- Check
		assert!(sismember(&mut *conn, &key, "red").await?);
		assert!(!sismember(&mut *conn, &key, "yellow").await?);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_sinter_sunion_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_setops");
		let key1 = format!("{}:set1", prefix);
		let key2 = format!("{}:set2", prefix);

		// -- Exec
		sadd(&mut *conn, &key1, &["a", "b", "c"]).await?;
		sadd(&mut *conn, &key2, &["b", "c", "d"]).await?;

		let mut inter: Vec<String> = sinter(&mut *conn, &[&key1, &key2]).await?;
		inter.sort();

		let mut union: Vec<String> = sunion(&mut *conn, &[&key1, &key2]).await?;
		union.sort();

		// -- Check
		assert_eq!(inter, vec!["b", "c"]);
		assert_eq!(union, vec!["a", "b", "c", "d"]);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_srem_scard_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_srem");
		let key = format!("{}:items", prefix);

		// -- Exec
		sadd(&mut *conn, &key, &["x", "y", "z"]).await?;
		assert_eq!(scard(&mut *conn, &key).await?, 3);

		srem_one(&mut *conn, &key, "y").await?;
		assert_eq!(scard(&mut *conn, &key).await?, 2);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}
}

// endregion: --- Tests
