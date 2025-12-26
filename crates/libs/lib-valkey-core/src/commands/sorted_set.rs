//! Sorted Set commands: ZADD, ZREM, ZRANGE, ZSCORE, ZRANK, ZCARD, ZINCRBY, ZCOUNT

use crate::Result;
use redis::AsyncCommands;

/// Add members with scores to a sorted set
pub async fn zadd<C, K, V>(conn: &mut C, key: K, items: &[(f64, V)]) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let added: u64 = conn.zadd_multiple(key, items).await?;
	Ok(added)
}

/// Add a single member with score
pub async fn zadd_one<C, K, V>(conn: &mut C, key: K, score: f64, member: V) -> Result<bool>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let added: u64 = conn.zadd(key, member, score).await?;
	Ok(added > 0)
}

/// Add member only if it doesn't exist (NX option)
pub async fn zadd_nx<C, K, V>(conn: &mut C, key: K, score: f64, member: V) -> Result<bool>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let added: u64 = redis::cmd("ZADD")
		.arg(key)
		.arg("NX")
		.arg(score)
		.arg(member)
		.query_async(conn)
		.await?;
	Ok(added > 0)
}

/// Update member only if it exists (XX option)
pub async fn zadd_xx<C, K, V>(conn: &mut C, key: K, score: f64, member: V) -> Result<bool>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let updated: u64 = redis::cmd("ZADD")
		.arg(key)
		.arg("XX")
		.arg(score)
		.arg(member)
		.query_async(conn)
		.await?;
	Ok(updated > 0)
}

/// Remove members from a sorted set
pub async fn zrem<C, K, V>(conn: &mut C, key: K, members: &[V]) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let removed: u64 = conn.zrem(key, members).await?;
	Ok(removed)
}

/// Remove a single member
pub async fn zrem_one<C, K, V>(conn: &mut C, key: K, member: V) -> Result<bool>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let removed: u64 = conn.zrem(key, member).await?;
	Ok(removed > 0)
}

/// Get range of members by rank (low to high score)
pub async fn zrange<C, K, V>(conn: &mut C, key: K, start: i64, stop: i64) -> Result<Vec<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let members: Vec<V> = conn.zrange(key, start as isize, stop as isize).await?;
	Ok(members)
}

/// Get range of members with scores
pub async fn zrange_withscores<C, K, V>(
	conn: &mut C,
	key: K,
	start: i64,
	stop: i64,
) -> Result<Vec<(V, f64)>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let members: Vec<(V, f64)> = conn
		.zrange_withscores(key, start as isize, stop as isize)
		.await?;
	Ok(members)
}

/// Get range of members by rank (high to low score)
pub async fn zrevrange<C, K, V>(conn: &mut C, key: K, start: i64, stop: i64) -> Result<Vec<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let members: Vec<V> = conn.zrevrange(key, start as isize, stop as isize).await?;
	Ok(members)
}

/// Get range of members by rank (high to low) with scores
pub async fn zrevrange_withscores<C, K, V>(
	conn: &mut C,
	key: K,
	start: i64,
	stop: i64,
) -> Result<Vec<(V, f64)>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let members: Vec<(V, f64)> = conn
		.zrevrange_withscores(key, start as isize, stop as isize)
		.await?;
	Ok(members)
}

/// Get range of members by score
pub async fn zrangebyscore<C, K, V>(
	conn: &mut C,
	key: K,
	min: f64,
	max: f64,
) -> Result<Vec<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let members: Vec<V> = conn.zrangebyscore(key, min, max).await?;
	Ok(members)
}

/// Get range of members by score with limit
pub async fn zrangebyscore_limit<C, K, V>(
	conn: &mut C,
	key: K,
	min: f64,
	max: f64,
	offset: i64,
	count: i64,
) -> Result<Vec<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let members: Vec<V> = conn
		.zrangebyscore_limit(key, min, max, offset as isize, count as isize)
		.await?;
	Ok(members)
}

/// Get the score of a member
pub async fn zscore<C, K, V>(conn: &mut C, key: K, member: V) -> Result<Option<f64>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let score: Option<f64> = conn.zscore(key, member).await?;
	Ok(score)
}

/// Get the rank of a member (0-based, low to high score)
pub async fn zrank<C, K, V>(conn: &mut C, key: K, member: V) -> Result<Option<u64>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let rank: Option<u64> = conn.zrank(key, member).await?;
	Ok(rank)
}

/// Get the rank of a member (0-based, high to low score)
pub async fn zrevrank<C, K, V>(conn: &mut C, key: K, member: V) -> Result<Option<u64>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let rank: Option<u64> = conn.zrevrank(key, member).await?;
	Ok(rank)
}

/// Get the number of members in a sorted set
pub async fn zcard<C, K>(conn: &mut C, key: K) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let count: u64 = conn.zcard(key).await?;
	Ok(count)
}

/// Count members with scores in a range
pub async fn zcount<C, K>(conn: &mut C, key: K, min: f64, max: f64) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let count: u64 = conn.zcount(key, min, max).await?;
	Ok(count)
}

/// Increment the score of a member
pub async fn zincrby<C, K, V>(conn: &mut C, key: K, delta: f64, member: V) -> Result<f64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::ToRedisArgs + Send + Sync,
{
	let new_score: f64 = conn.zincr(key, member, delta).await?;
	Ok(new_score)
}

/// Remove members by rank range
pub async fn zremrangebyrank<C, K>(conn: &mut C, key: K, start: i64, stop: i64) -> Result<u64>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
{
	let removed: u64 = conn.zremrangebyrank(key, start as isize, stop as isize).await?;
	Ok(removed)
}

/// Remove members by score range
pub async fn zremrangebyscore<C, K>(conn: &mut C, key: K, min: f64, max: f64) -> Result<u64>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let removed: u64 = redis::cmd("ZREMRANGEBYSCORE")
		.arg(key)
		.arg(min)
		.arg(max)
		.query_async(conn)
		.await?;
	Ok(removed)
}

/// Pop members with lowest scores
pub async fn zpopmin<C, K, V>(conn: &mut C, key: K, count: u64) -> Result<Vec<(V, f64)>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let members: Vec<(V, f64)> = conn.zpopmin(key, count as isize).await?;
	Ok(members)
}

/// Pop members with highest scores
pub async fn zpopmax<C, K, V>(conn: &mut C, key: K, count: u64) -> Result<Vec<(V, f64)>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let members: Vec<(V, f64)> = conn.zpopmax(key, count as isize).await?;
	Ok(members)
}

/// Blocking pop with lowest score
pub async fn bzpopmin<C, K, V>(
	conn: &mut C,
	keys: &[K],
	timeout: f64,
) -> Result<Option<(String, V, f64)>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let result: Option<(String, V, f64)> = conn.bzpopmin(keys, timeout).await?;
	Ok(result)
}

/// Blocking pop with highest score
pub async fn bzpopmax<C, K, V>(
	conn: &mut C,
	keys: &[K],
	timeout: f64,
) -> Result<Option<(String, V, f64)>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: redis::FromRedisValue,
{
	let result: Option<(String, V, f64)> = conn.bzpopmax(keys, timeout).await?;
	Ok(result)
}

/// Scan sorted set members
pub async fn zscan<C, K>(
	conn: &mut C,
	key: K,
	cursor: u64,
	pattern: Option<&str>,
	count: Option<usize>,
) -> Result<(u64, Vec<(String, f64)>)>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let mut cmd = redis::cmd("ZSCAN");
	cmd.arg(key).arg(cursor);

	if let Some(p) = pattern {
		cmd.arg("MATCH").arg(p);
	}
	if let Some(c) = count {
		cmd.arg("COUNT").arg(c);
	}

	let (next_cursor, members): (u64, Vec<(String, f64)>) = cmd.query_async(conn).await?;
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
	async fn test_zadd_zrange_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_zset");
		let key = format!("{}:scores", prefix);

		// -- Exec
		zadd(&mut *conn, &key, &[(100.0, "alice"), (200.0, "bob"), (150.0, "charlie")]).await?;

		let members: Vec<String> = zrange(&mut *conn, &key, 0, -1).await?;

		// -- Check (sorted by score: alice < charlie < bob)
		assert_eq!(members, vec!["alice", "charlie", "bob"]);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_zscore_zrank_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_zscore");
		let key = format!("{}:leaderboard", prefix);

		// -- Exec
		zadd(&mut *conn, &key, &[(10.0, "a"), (20.0, "b"), (30.0, "c")]).await?;

		let score = zscore(&mut *conn, &key, "b").await?;
		let rank = zrank(&mut *conn, &key, "b").await?;

		// -- Check
		assert_eq!(score, Some(20.0));
		assert_eq!(rank, Some(1)); // 0-based: a=0, b=1, c=2

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_zincrby_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_zincrby");
		let key = format!("{}:points", prefix);

		// -- Exec
		zadd_one(&mut *conn, &key, 50.0, "player1").await?;
		let new_score = zincrby(&mut *conn, &key, 25.5, "player1").await?;

		// -- Check
		assert_eq!(new_score, 75.5);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_zcount_zcard_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_zcount");
		let key = format!("{}:data", prefix);

		// -- Exec
		zadd(&mut *conn, &key, &[(1.0, "a"), (2.0, "b"), (3.0, "c"), (4.0, "d")]).await?;

		let card = zcard(&mut *conn, &key).await?;
		let count = zcount(&mut *conn, &key, 2.0, 3.0).await?;

		// -- Check
		assert_eq!(card, 4);
		assert_eq!(count, 2); // b and c

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}
}

// endregion: --- Tests
