//! Distributed lock implementation using Valkey/Redis

use crate::Result;
use redis::AsyncCommands;
use uuid::Uuid;

/// A distributed lock backed by Valkey/Redis
pub struct DistributedLock {
	key: String,
	token: String,
	ttl_secs: u64,
}

impl DistributedLock {
	/// Create a new lock instance (does not acquire the lock)
	pub fn new(key: impl Into<String>, ttl_secs: u64) -> Self {
		Self {
			key: format!("lock:{}", key.into()),
			token: Uuid::new_v4().to_string(),
			ttl_secs,
		}
	}

	/// Get the lock key
	pub fn key(&self) -> &str {
		&self.key
	}

	/// Get the lock token
	pub fn token(&self) -> &str {
		&self.token
	}

	/// Try to acquire the lock
	/// Returns true if lock was acquired, false if already held
	pub async fn try_acquire<C>(&self, conn: &mut C) -> Result<bool>
	where
		C: AsyncCommands,
	{
		let result: Option<String> = redis::cmd("SET")
			.arg(&self.key)
			.arg(&self.token)
			.arg("NX")
			.arg("EX")
			.arg(self.ttl_secs)
			.query_async(conn)
			.await?;

		Ok(result.is_some())
	}

	/// Acquire the lock, waiting up to max_wait_ms
	pub async fn acquire<C>(&self, conn: &mut C, max_wait_ms: u64) -> Result<bool>
	where
		C: AsyncCommands,
	{
		let start = std::time::Instant::now();
		let max_wait = std::time::Duration::from_millis(max_wait_ms);

		loop {
			if self.try_acquire(conn).await? {
				return Ok(true);
			}

			if start.elapsed() >= max_wait {
				return Ok(false);
			}

			tokio::time::sleep(std::time::Duration::from_millis(50)).await;
		}
	}

	/// Release the lock (only if we own it)
	pub async fn release<C>(&self, conn: &mut C) -> Result<bool>
	where
		C: AsyncCommands,
	{
		// Lua script to atomically check and delete
		let script = r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#;

		let result: i32 = redis::Script::new(script)
			.key(&self.key)
			.arg(&self.token)
			.invoke_async(conn)
			.await?;

		Ok(result == 1)
	}

	/// Extend the lock TTL (only if we own it)
	pub async fn extend<C>(&self, conn: &mut C, additional_secs: u64) -> Result<bool>
	where
		C: AsyncCommands,
	{
		let script = r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("expire", KEYS[1], ARGV[2])
            else
                return 0
            end
        "#;

		let new_ttl = self.ttl_secs + additional_secs;
		let result: i32 = redis::Script::new(script)
			.key(&self.key)
			.arg(&self.token)
			.arg(new_ttl)
			.invoke_async(conn)
			.await?;

		Ok(result == 1)
	}
}

/// Execute a closure while holding a distributed lock
pub async fn with_lock<C, F, Fut, T>(
	conn: &mut C,
	lock_key: &str,
	ttl_secs: u64,
	max_wait_ms: u64,
	f: F,
) -> Result<Option<T>>
where
	C: AsyncCommands,
	F: FnOnce() -> Fut,
	Fut: std::future::Future<Output = T>,
{
	let lock = DistributedLock::new(lock_key, ttl_secs);

	if lock.acquire(conn, max_wait_ms).await? {
		let result = f().await;
		lock.release(conn).await?;
		Ok(Some(result))
	} else {
		Ok(None)
	}
}

/// A guard that releases the lock when dropped
pub struct LockGuard<'a, C>
where
	C: AsyncCommands,
{
	lock: DistributedLock,
	conn: &'a mut C,
	released: bool,
}

impl<'a, C> LockGuard<'a, C>
where
	C: AsyncCommands,
{
	/// Create a new lock guard (internal use)
	fn new(lock: DistributedLock, conn: &'a mut C) -> Self {
		Self {
			lock,
			conn,
			released: false,
		}
	}

	/// Manually release the lock
	pub async fn release(mut self) -> Result<bool> {
		self.released = true;
		self.lock.release(self.conn).await
	}

	/// Extend the lock TTL
	pub async fn extend(&mut self, additional_secs: u64) -> Result<bool> {
		self.lock.extend(self.conn, additional_secs).await
	}
}

/// Try to acquire a lock and return a guard
pub async fn try_lock<'a, C>(
	conn: &'a mut C,
	lock_key: &str,
	ttl_secs: u64,
) -> Result<Option<LockGuard<'a, C>>>
where
	C: AsyncCommands,
{
	let lock = DistributedLock::new(lock_key, ttl_secs);

	if lock.try_acquire(conn).await? {
		Ok(Some(LockGuard::new(lock, conn)))
	} else {
		Ok(None)
	}
}
