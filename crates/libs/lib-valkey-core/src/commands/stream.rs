//! Redis Streams commands: XADD, XREAD, XREADGROUP, XACK, XRANGE, XLEN

use crate::Result;
use std::collections::HashMap;

/// Add entry to a stream
pub async fn xadd<C, K>(
	conn: &mut C,
	key: K,
	id: Option<&str>,
	fields: &[(&str, &str)],
) -> Result<String>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let mut cmd = redis::cmd("XADD");
	cmd.arg(key).arg(id.unwrap_or("*"));
	for (field, value) in fields {
		cmd.arg(*field).arg(*value);
	}
	let entry_id: String = cmd.query_async(conn).await?;
	Ok(entry_id)
}

/// Add entry with max length constraint
pub async fn xadd_maxlen<C, K>(
	conn: &mut C,
	key: K,
	maxlen: u64,
	approximate: bool,
	fields: &[(&str, &str)],
) -> Result<String>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let mut cmd = redis::cmd("XADD");
	cmd.arg(key);
	cmd.arg("MAXLEN");
	if approximate {
		cmd.arg("~");
	}
	cmd.arg(maxlen);
	cmd.arg("*");
	for (field, value) in fields {
		cmd.arg(*field).arg(*value);
	}
	let entry_id: String = cmd.query_async(conn).await?;
	Ok(entry_id)
}

/// Stream entry
#[derive(Debug, Clone)]
pub struct StreamEntry {
	pub id: String,
	pub fields: HashMap<String, String>,
}

/// Read from streams
pub async fn xread<C, K>(
	conn: &mut C,
	count: Option<u64>,
	block: Option<u64>,
	streams: &[(K, &str)],
) -> Result<Vec<(String, Vec<StreamEntry>)>>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let mut cmd = redis::cmd("XREAD");
	if let Some(c) = count {
		cmd.arg("COUNT").arg(c);
	}
	if let Some(b) = block {
		cmd.arg("BLOCK").arg(b);
	}
	cmd.arg("STREAMS");
	for (key, _) in streams {
		cmd.arg(key);
	}
	for (_, id) in streams {
		cmd.arg(*id);
	}

	let result: Option<Vec<(String, Vec<(String, HashMap<String, String>)>)>> =
		cmd.query_async(conn).await?;

	Ok(result
		.unwrap_or_default()
		.into_iter()
		.map(|(stream, entries)| {
			let entries = entries
				.into_iter()
				.map(|(id, fields)| StreamEntry { id, fields })
				.collect();
			(stream, entries)
		})
		.collect())
}

/// Create a consumer group
pub async fn xgroup_create<C, K>(
	conn: &mut C,
	key: K,
	group: &str,
	id: &str,
	mkstream: bool,
) -> Result<()>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let mut cmd = redis::cmd("XGROUP");
	cmd.arg("CREATE").arg(key).arg(group).arg(id);
	if mkstream {
		cmd.arg("MKSTREAM");
	}
	cmd.query_async::<()>(conn).await?;
	Ok(())
}

/// Destroy a consumer group
pub async fn xgroup_destroy<C, K>(conn: &mut C, key: K, group: &str) -> Result<bool>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let result: u64 = redis::cmd("XGROUP")
		.arg("DESTROY")
		.arg(key)
		.arg(group)
		.query_async(conn)
		.await?;
	Ok(result > 0)
}

/// Read from stream as a consumer group member
pub async fn xreadgroup<C, K>(
	conn: &mut C,
	group: &str,
	consumer: &str,
	count: Option<u64>,
	block: Option<u64>,
	streams: &[(K, &str)],
) -> Result<Vec<(String, Vec<StreamEntry>)>>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let mut cmd = redis::cmd("XREADGROUP");
	cmd.arg("GROUP").arg(group).arg(consumer);
	if let Some(c) = count {
		cmd.arg("COUNT").arg(c);
	}
	if let Some(b) = block {
		cmd.arg("BLOCK").arg(b);
	}
	cmd.arg("STREAMS");
	for (key, _) in streams {
		cmd.arg(key);
	}
	for (_, id) in streams {
		cmd.arg(*id);
	}

	let result: Option<Vec<(String, Vec<(String, HashMap<String, String>)>)>> =
		cmd.query_async(conn).await?;

	Ok(result
		.unwrap_or_default()
		.into_iter()
		.map(|(stream, entries)| {
			let entries = entries
				.into_iter()
				.map(|(id, fields)| StreamEntry { id, fields })
				.collect();
			(stream, entries)
		})
		.collect())
}

/// Acknowledge processed messages
pub async fn xack<C, K>(conn: &mut C, key: K, group: &str, ids: &[&str]) -> Result<u64>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let mut cmd = redis::cmd("XACK");
	cmd.arg(key).arg(group);
	for id in ids {
		cmd.arg(*id);
	}
	let count: u64 = cmd.query_async(conn).await?;
	Ok(count)
}

/// Get range of entries
pub async fn xrange<C, K>(
	conn: &mut C,
	key: K,
	start: &str,
	end: &str,
	count: Option<u64>,
) -> Result<Vec<StreamEntry>>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let mut cmd = redis::cmd("XRANGE");
	cmd.arg(key).arg(start).arg(end);
	if let Some(c) = count {
		cmd.arg("COUNT").arg(c);
	}

	let entries: Vec<(String, HashMap<String, String>)> = cmd.query_async(conn).await?;
	Ok(entries
		.into_iter()
		.map(|(id, fields)| StreamEntry { id, fields })
		.collect())
}

/// Get range in reverse order
pub async fn xrevrange<C, K>(
	conn: &mut C,
	key: K,
	end: &str,
	start: &str,
	count: Option<u64>,
) -> Result<Vec<StreamEntry>>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let mut cmd = redis::cmd("XREVRANGE");
	cmd.arg(key).arg(end).arg(start);
	if let Some(c) = count {
		cmd.arg("COUNT").arg(c);
	}

	let entries: Vec<(String, HashMap<String, String>)> = cmd.query_async(conn).await?;
	Ok(entries
		.into_iter()
		.map(|(id, fields)| StreamEntry { id, fields })
		.collect())
}

/// Get stream length
pub async fn xlen<C, K>(conn: &mut C, key: K) -> Result<u64>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let len: u64 = redis::cmd("XLEN").arg(key).query_async(conn).await?;
	Ok(len)
}

/// Delete entries from a stream
pub async fn xdel<C, K>(conn: &mut C, key: K, ids: &[&str]) -> Result<u64>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let mut cmd = redis::cmd("XDEL");
	cmd.arg(key);
	for id in ids {
		cmd.arg(*id);
	}
	let count: u64 = cmd.query_async(conn).await?;
	Ok(count)
}

/// Trim stream to a maximum length
pub async fn xtrim<C, K>(conn: &mut C, key: K, maxlen: u64, approximate: bool) -> Result<u64>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let mut cmd = redis::cmd("XTRIM");
	cmd.arg(key).arg("MAXLEN");
	if approximate {
		cmd.arg("~");
	}
	cmd.arg(maxlen);
	let trimmed: u64 = cmd.query_async(conn).await?;
	Ok(trimmed)
}

/// Get pending entries info
pub async fn xpending_summary<C, K>(
	conn: &mut C,
	key: K,
	group: &str,
) -> Result<Option<(u64, String, String, Vec<(String, u64)>)>>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let result: Option<(u64, String, String, Vec<(String, u64)>)> = redis::cmd("XPENDING")
		.arg(key)
		.arg(group)
		.query_async(conn)
		.await?;
	Ok(result)
}

/// Claim pending messages
pub async fn xclaim<C, K>(
	conn: &mut C,
	key: K,
	group: &str,
	consumer: &str,
	min_idle_time: u64,
	ids: &[&str],
) -> Result<Vec<StreamEntry>>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let mut cmd = redis::cmd("XCLAIM");
	cmd.arg(key).arg(group).arg(consumer).arg(min_idle_time);
	for id in ids {
		cmd.arg(*id);
	}

	let entries: Vec<(String, HashMap<String, String>)> = cmd.query_async(conn).await?;
	Ok(entries
		.into_iter()
		.map(|(id, fields)| StreamEntry { id, fields })
		.collect())
}

/// Stream info
#[derive(Debug)]
pub struct StreamInfo {
	pub length: u64,
	pub first_entry_id: Option<String>,
	pub last_entry_id: Option<String>,
	pub groups: u64,
}

/// Get stream info
pub async fn xinfo_stream<C, K>(conn: &mut C, key: K) -> Result<StreamInfo>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync,
{
	let info: HashMap<String, redis::Value> = redis::cmd("XINFO")
		.arg("STREAM")
		.arg(key)
		.query_async(conn)
		.await?;

	let length = match info.get("length") {
		Some(redis::Value::Int(n)) => *n as u64,
		_ => 0,
	};
	let groups = match info.get("groups") {
		Some(redis::Value::Int(n)) => *n as u64,
		_ => 0,
	};

	Ok(StreamInfo {
		length,
		first_entry_id: None, // Simplified
		last_entry_id: None,
		groups,
	})
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
	async fn test_xadd_xlen_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_stream");
		let key = format!("{}:events", prefix);

		// -- Exec
		let id1 = xadd(&mut *conn, &key, None, &[("type", "click"), ("user", "alice")]).await?;
		let id2 = xadd(&mut *conn, &key, None, &[("type", "view"), ("user", "bob")]).await?;

		let len = xlen(&mut *conn, &key).await?;

		// -- Check
		assert!(!id1.is_empty());
		assert!(!id2.is_empty());
		assert_eq!(len, 2);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_xrange_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_xrange");
		let key = format!("{}:log", prefix);

		// -- Exec
		xadd(&mut *conn, &key, None, &[("msg", "first")]).await?;
		xadd(&mut *conn, &key, None, &[("msg", "second")]).await?;

		let entries = xrange(&mut *conn, &key, "-", "+", None).await?;

		// -- Check
		assert_eq!(entries.len(), 2);
		assert_eq!(entries[0].fields.get("msg"), Some(&"first".to_string()));
		assert_eq!(entries[1].fields.get("msg"), Some(&"second".to_string()));

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_xdel_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_xdel");
		let key = format!("{}:data", prefix);

		// -- Exec
		let id = xadd(&mut *conn, &key, None, &[("k", "v")]).await?;
		assert_eq!(xlen(&mut *conn, &key).await?, 1);

		let deleted = xdel(&mut *conn, &key, &[&id]).await?;
		assert_eq!(deleted, 1);
		assert_eq!(xlen(&mut *conn, &key).await?, 0);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_xgroup_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_xgroup");
		let key = format!("{}:queue", prefix);

		// -- Exec (create group with MKSTREAM)
		xgroup_create(&mut *conn, &key, "mygroup", "$", true).await?;

		// Add message and verify stream was created
		xadd(&mut *conn, &key, None, &[("job", "process")]).await?;
		let len = xlen(&mut *conn, &key).await?;
		assert_eq!(len, 1);

		// -- Clean
		let _ = xgroup_destroy(&mut *conn, &key, "mygroup").await;
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}
}

// endregion: --- Tests
