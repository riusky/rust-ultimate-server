//! Transaction support: MULTI, EXEC, DISCARD, WATCH

use crate::{Error, Result};

/// A transaction builder for atomic operations
pub struct Transaction<'a, C> {
	conn: &'a mut C,
	commands: Vec<redis::Cmd>,
}

impl<'a, C> Transaction<'a, C>
where
	C: redis::aio::ConnectionLike + Send,
{
	/// Create a new transaction
	pub fn new(conn: &'a mut C) -> Self {
		Self {
			conn,
			commands: Vec::new(),
		}
	}

	/// Add a command to the transaction
	pub fn add(&mut self, cmd: redis::Cmd) -> &mut Self {
		self.commands.push(cmd);
		self
	}

	/// Add a SET command
	pub fn set<K, V>(&mut self, key: K, value: V) -> &mut Self
	where
		K: redis::ToRedisArgs,
		V: redis::ToRedisArgs,
	{
		let mut cmd = redis::cmd("SET");
		cmd.arg(key).arg(value);
		self.add(cmd)
	}

	/// Add a GET command
	pub fn get<K>(&mut self, key: K) -> &mut Self
	where
		K: redis::ToRedisArgs,
	{
		let mut cmd = redis::cmd("GET");
		cmd.arg(key);
		self.add(cmd)
	}

	/// Add an INCR command
	pub fn incr<K>(&mut self, key: K) -> &mut Self
	where
		K: redis::ToRedisArgs,
	{
		let mut cmd = redis::cmd("INCR");
		cmd.arg(key);
		self.add(cmd)
	}

	/// Add a DECR command
	pub fn decr<K>(&mut self, key: K) -> &mut Self
	where
		K: redis::ToRedisArgs,
	{
		let mut cmd = redis::cmd("DECR");
		cmd.arg(key);
		self.add(cmd)
	}

	/// Add a DEL command
	pub fn del<K>(&mut self, keys: &[K]) -> &mut Self
	where
		K: redis::ToRedisArgs,
	{
		let mut cmd = redis::cmd("DEL");
		for k in keys {
			cmd.arg(k);
		}
		self.add(cmd)
	}

	/// Add an HSET command
	pub fn hset<K, F, V>(&mut self, key: K, field: F, value: V) -> &mut Self
	where
		K: redis::ToRedisArgs,
		F: redis::ToRedisArgs,
		V: redis::ToRedisArgs,
	{
		let mut cmd = redis::cmd("HSET");
		cmd.arg(key).arg(field).arg(value);
		self.add(cmd)
	}

	/// Add an HGET command
	pub fn hget<K, F>(&mut self, key: K, field: F) -> &mut Self
	where
		K: redis::ToRedisArgs,
		F: redis::ToRedisArgs,
	{
		let mut cmd = redis::cmd("HGET");
		cmd.arg(key).arg(field);
		self.add(cmd)
	}

	/// Add an LPUSH command
	pub fn lpush<K, V>(&mut self, key: K, values: &[V]) -> &mut Self
	where
		K: redis::ToRedisArgs,
		V: redis::ToRedisArgs,
	{
		let mut cmd = redis::cmd("LPUSH");
		cmd.arg(key);
		for v in values {
			cmd.arg(v);
		}
		self.add(cmd)
	}

	/// Add an RPUSH command
	pub fn rpush<K, V>(&mut self, key: K, values: &[V]) -> &mut Self
	where
		K: redis::ToRedisArgs,
		V: redis::ToRedisArgs,
	{
		let mut cmd = redis::cmd("RPUSH");
		cmd.arg(key);
		for v in values {
			cmd.arg(v);
		}
		self.add(cmd)
	}

	/// Add a SADD command
	pub fn sadd<K, V>(&mut self, key: K, members: &[V]) -> &mut Self
	where
		K: redis::ToRedisArgs,
		V: redis::ToRedisArgs,
	{
		let mut cmd = redis::cmd("SADD");
		cmd.arg(key);
		for m in members {
			cmd.arg(m);
		}
		self.add(cmd)
	}

	/// Add a ZADD command
	pub fn zadd<K, V>(&mut self, key: K, score: f64, member: V) -> &mut Self
	where
		K: redis::ToRedisArgs,
		V: redis::ToRedisArgs,
	{
		let mut cmd = redis::cmd("ZADD");
		cmd.arg(key).arg(score).arg(member);
		self.add(cmd)
	}

	/// Add an EXPIRE command
	pub fn expire<K>(&mut self, key: K, seconds: u64) -> &mut Self
	where
		K: redis::ToRedisArgs,
	{
		let mut cmd = redis::cmd("EXPIRE");
		cmd.arg(key).arg(seconds);
		self.add(cmd)
	}

	/// Execute the transaction and return results
	pub async fn execute(self) -> Result<Vec<redis::Value>> {
		if self.commands.is_empty() {
			return Ok(Vec::new());
		}

		// Start transaction
		redis::cmd("MULTI")
			.query_async::<()>(self.conn)
			.await?;

		// Queue all commands
		for cmd in &self.commands {
			let _: redis::Value = cmd.query_async(self.conn).await?;
		}

		// Execute and get results
		let results: Vec<redis::Value> = redis::cmd("EXEC")
			.query_async(self.conn)
			.await?;

		Ok(results)
	}

	/// Discard the transaction
	pub async fn discard(self) -> Result<()> {
		redis::cmd("DISCARD")
			.query_async::<()>(self.conn)
			.await?;
		Ok(())
	}
}

/// Watch keys for changes (optimistic locking)
pub async fn watch<C, K>(conn: &mut C, keys: &[K]) -> Result<()>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs,
{
	let mut cmd = redis::cmd("WATCH");
	for k in keys {
		cmd.arg(k);
	}
	cmd.query_async::<()>(conn).await?;
	Ok(())
}

/// Unwatch all keys
pub async fn unwatch<C>(conn: &mut C) -> Result<()>
where
	C: redis::aio::ConnectionLike + Send,
{
	redis::cmd("UNWATCH")
		.query_async::<()>(conn)
		.await?;
	Ok(())
}

/// Execute a simple transaction with a closure
pub async fn atomic<C, F, R>(conn: &mut C, f: F) -> Result<R>
where
	C: redis::aio::ConnectionLike + Send,
	for<'a> F: FnOnce(&mut Transaction<'a, C>),
	R: redis::FromRedisValue,
{
	let mut tx = Transaction::new(conn);
	f(&mut tx);
	let results = tx.execute().await?;
	if results.is_empty() {
		return Err(Error::Custom {
			message: "Transaction returned no results".to_string(),
		});
	}
	R::from_redis_value(&results[0]).map_err(Error::Redis)
}

/// Execute with optimistic locking (watch + transaction)
pub async fn with_watch<C, K, F, Fut, T>(
	conn: &mut C,
	keys: &[K],
	f: F,
	max_retries: usize,
) -> Result<T>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Clone,
	F: Fn(&mut C) -> Fut,
	Fut: std::future::Future<Output = Result<T>>,
{
	for _ in 0..max_retries {
		watch(conn, keys).await?;

		match f(conn).await {
			Ok(result) => return Ok(result),
			Err(Error::Redis(e)) if e.to_string().contains("EXECABORT") => {
				// Transaction aborted due to watched key change, retry
				continue;
			}
			Err(e) => {
				unwatch(conn).await?;
				return Err(e);
			}
		}
	}

	Err(Error::Custom {
		message: "Transaction failed after max retries".to_string(),
	})
}

/// Pipeline builder for non-atomic batch operations
pub struct Pipeline<'a, C> {
	conn: &'a mut C,
	pipe: redis::Pipeline,
}

impl<'a, C> Pipeline<'a, C>
where
	C: redis::aio::ConnectionLike + Send,
{
	/// Create a new pipeline
	pub fn new(conn: &'a mut C) -> Self {
		Self {
			conn,
			pipe: redis::pipe(),
		}
	}

	/// Add a command to the pipeline
	pub fn cmd(&mut self, cmd: redis::Cmd) -> &mut Self {
		self.pipe.add_command(cmd);
		self
	}

	/// Add a SET command
	pub fn set<K, V>(&mut self, key: K, value: V) -> &mut Self
	where
		K: redis::ToRedisArgs,
		V: redis::ToRedisArgs,
	{
		self.pipe.set(key, value);
		self
	}

	/// Add a GET command
	pub fn get<K>(&mut self, key: K) -> &mut Self
	where
		K: redis::ToRedisArgs,
	{
		self.pipe.get(key);
		self
	}

	/// Add an INCR command
	pub fn incr<K>(&mut self, key: K, delta: i64) -> &mut Self
	where
		K: redis::ToRedisArgs,
	{
		self.pipe.incr(key, delta);
		self
	}

	/// Add a DEL command
	pub fn del<K>(&mut self, key: K) -> &mut Self
	where
		K: redis::ToRedisArgs,
	{
		self.pipe.del(key);
		self
	}

	/// Execute the pipeline
	pub async fn execute(self) -> Result<Vec<redis::Value>> {
		let results: Vec<redis::Value> = self.pipe.query_async(self.conn).await?;
		Ok(results)
	}

	/// Execute and ignore results
	pub async fn execute_ignore(self) -> Result<()> {
		self.pipe.query_async::<()>(self.conn).await?;
		Ok(())
	}
}
