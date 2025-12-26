//! Lua script execution: EVAL, EVALSHA, SCRIPT LOAD/EXISTS/FLUSH

use crate::Result;

/// Execute a Lua script
pub async fn eval<C, R>(
	conn: &mut C,
	script: &str,
	keys: &[&str],
	args: &[&str],
) -> Result<R>
where
	C: redis::aio::ConnectionLike + Send,
	R: redis::FromRedisValue,
{
	let result: R = redis::Script::new(script)
		.key(keys)
		.arg(args)
		.invoke_async(conn)
		.await?;
	Ok(result)
}

/// Execute a script by SHA1 hash (must be loaded first)
pub async fn evalsha<C, R>(
	conn: &mut C,
	sha1: &str,
	keys: &[&str],
	args: &[&str],
) -> Result<R>
where
	C: redis::aio::ConnectionLike + Send,
	R: redis::FromRedisValue,
{
	let mut cmd = redis::cmd("EVALSHA");
	cmd.arg(sha1).arg(keys.len());
	for k in keys {
		cmd.arg(*k);
	}
	for a in args {
		cmd.arg(*a);
	}
	let result: R = cmd.query_async(conn).await?;
	Ok(result)
}

/// Load a script into the script cache
pub async fn script_load<C>(conn: &mut C, script: &str) -> Result<String>
where
	C: redis::aio::ConnectionLike + Send,
{
	let sha1: String = redis::cmd("SCRIPT")
		.arg("LOAD")
		.arg(script)
		.query_async(conn)
		.await?;
	Ok(sha1)
}

/// Check if scripts exist in the cache
pub async fn script_exists<C>(conn: &mut C, sha1s: &[&str]) -> Result<Vec<bool>>
where
	C: redis::aio::ConnectionLike + Send,
{
	let mut cmd = redis::cmd("SCRIPT");
	cmd.arg("EXISTS");
	for sha1 in sha1s {
		cmd.arg(*sha1);
	}
	let results: Vec<i32> = cmd.query_async(conn).await?;
	Ok(results.into_iter().map(|v| v == 1).collect())
}

/// Flush the script cache
pub async fn script_flush<C>(conn: &mut C, sync: bool) -> Result<()>
where
	C: redis::aio::ConnectionLike + Send,
{
	let mut cmd = redis::cmd("SCRIPT");
	cmd.arg("FLUSH");
	if sync {
		cmd.arg("SYNC");
	} else {
		cmd.arg("ASYNC");
	}
	cmd.query_async::<()>(conn).await?;
	Ok(())
}

/// Kill the currently executing script
pub async fn script_kill<C>(conn: &mut C) -> Result<()>
where
	C: redis::aio::ConnectionLike + Send,
{
	redis::cmd("SCRIPT")
		.arg("KILL")
		.query_async::<()>(conn)
		.await?;
	Ok(())
}

/// A cached script that can be executed efficiently
pub struct CachedScript {
	script: redis::Script,
}

impl CachedScript {
	/// Create a new cached script
	pub fn new(script: &str) -> Self {
		Self {
			script: redis::Script::new(script),
		}
	}

	/// Get the SHA1 hash of the script
	pub fn sha1(&self) -> &str {
		self.script.get_hash()
	}

	/// Execute the script with keys and args
	pub async fn invoke<C, R>(&self, conn: &mut C, keys: &[&str], args: &[&str]) -> Result<R>
	where
		C: redis::aio::ConnectionLike + Send,
		R: redis::FromRedisValue,
	{
		let result: R = self.script.key(keys).arg(args).invoke_async(conn).await?;
		Ok(result)
	}

	/// Execute with typed keys and args
	pub async fn invoke_typed<C, K, A, R>(
		&self,
		conn: &mut C,
		keys: &[K],
		args: &[A],
	) -> Result<R>
	where
		C: redis::aio::ConnectionLike + Send,
		K: redis::ToRedisArgs,
		A: redis::ToRedisArgs,
		R: redis::FromRedisValue,
	{
		let mut invocation = self.script.prepare_invoke();
		for k in keys {
			invocation.key(k);
		}
		for a in args {
			invocation.arg(a);
		}
		let result: R = invocation.invoke_async(conn).await?;
		Ok(result)
	}
}

// Common Lua script patterns

/// Atomic increment with maximum value
pub fn script_incr_max() -> CachedScript {
	CachedScript::new(
		r#"
		local current = redis.call('GET', KEYS[1])
		if current == false then current = 0 else current = tonumber(current) end
		local max = tonumber(ARGV[1])
		if current >= max then return -1 end
		return redis.call('INCR', KEYS[1])
		"#,
	)
}

/// Get and delete atomically
pub fn script_get_del() -> CachedScript {
	CachedScript::new(
		r#"
		local value = redis.call('GET', KEYS[1])
		if value then redis.call('DEL', KEYS[1]) end
		return value
		"#,
	)
}

/// Set with expiration only if key exists
pub fn script_set_xx_ex() -> CachedScript {
	CachedScript::new(
		r#"
		if redis.call('EXISTS', KEYS[1]) == 1 then
			return redis.call('SET', KEYS[1], ARGV[1], 'EX', ARGV[2])
		end
		return nil
		"#,
	)
}

/// Rate limiter script (sliding window)
pub fn script_rate_limit() -> CachedScript {
	CachedScript::new(
		r#"
		local key = KEYS[1]
		local window = tonumber(ARGV[1])
		local limit = tonumber(ARGV[2])
		local now = tonumber(ARGV[3])
		
		redis.call('ZREMRANGEBYSCORE', key, 0, now - window)
		local count = redis.call('ZCARD', key)
		
		if count < limit then
			redis.call('ZADD', key, now, now .. '-' .. math.random())
			redis.call('EXPIRE', key, window)
			return 1
		end
		return 0
		"#,
	)
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
	async fn test_eval_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_script");
		let key = format!("{}:val", prefix);

		// Simple script: SET and return value
		let script = r#"
			redis.call('SET', KEYS[1], ARGV[1])
			return redis.call('GET', KEYS[1])
		"#;

		// -- Exec
		let result: String = eval(&mut *conn, script, &[&key], &["hello"]).await?;

		// -- Check
		assert_eq!(result, "hello");

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_script_load_exists_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;

		let script = "return 42";

		// -- Exec
		let sha1 = script_load(&mut *conn, script).await?;
		let exists = script_exists(&mut *conn, &[&sha1]).await?;

		// -- Check
		assert!(!sha1.is_empty());
		assert_eq!(exists, vec![true]);

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_cached_script_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_cached");
		let key = format!("{}:counter", prefix);

		let script = CachedScript::new("return redis.call('INCR', KEYS[1])");

		// -- Exec
		let v1: i64 = script.invoke(&mut *conn, &[&key], &[]).await?;
		let v2: i64 = script.invoke(&mut *conn, &[&key], &[]).await?;

		// -- Check
		assert_eq!(v1, 1);
		assert_eq!(v2, 2);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}
}

// endregion: --- Tests
