//! JSON serialization helpers for storing complex types

use crate::Result;
use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};

/// Set a JSON-serializable value
pub async fn set_json<C, K, V>(
	conn: &mut C,
	key: K,
	value: &V,
	expire_secs: Option<u64>,
) -> Result<()>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: Serialize,
{
	let json = serde_json::to_string(value)?;
	if let Some(secs) = expire_secs {
		conn.set_ex::<_, _, ()>(key, json, secs).await?;
	} else {
		conn.set::<_, _, ()>(key, json).await?;
	}
	Ok(())
}

/// Get and deserialize a JSON value
pub async fn get_json<C, K, V>(conn: &mut C, key: K) -> Result<Option<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	V: DeserializeOwned,
{
	let json: Option<String> = conn.get(key).await?;
	match json {
		Some(s) => {
			let value: V = serde_json::from_str(&s)?;
			Ok(Some(value))
		}
		None => Ok(None),
	}
}

/// Get JSON value, return error if not found
pub async fn get_json_required<C, K, V>(conn: &mut C, key: K) -> Result<V>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync + Clone + ToString,
	V: DeserializeOwned,
{
	let json: Option<String> = conn.get(key.clone()).await?;
	match json {
		Some(s) => {
			let value: V = serde_json::from_str(&s)?;
			Ok(value)
		}
		None => Err(crate::Error::KeyNotFound {
			key: key.to_string(),
		}),
	}
}

/// Set a JSON value in a hash field
pub async fn hset_json<C, K, F, V>(conn: &mut C, key: K, field: F, value: &V) -> Result<()>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	F: redis::ToRedisArgs + Send + Sync,
	V: Serialize,
{
	let json = serde_json::to_string(value)?;
	conn.hset::<_, _, _, ()>(key, field, json).await?;
	Ok(())
}

/// Get and deserialize a JSON value from hash field
pub async fn hget_json<C, K, F, V>(conn: &mut C, key: K, field: F) -> Result<Option<V>>
where
	C: AsyncCommands,
	K: redis::ToRedisArgs + Send + Sync,
	F: redis::ToRedisArgs + Send + Sync,
	V: DeserializeOwned,
{
	let json: Option<String> = conn.hget(key, field).await?;
	match json {
		Some(s) => {
			let value: V = serde_json::from_str(&s)?;
			Ok(Some(value))
		}
		None => Ok(None),
	}
}

/// Set multiple JSON values
pub async fn mset_json<C, K, V>(conn: &mut C, pairs: &[(K, V)]) -> Result<()>
where
	C: redis::aio::ConnectionLike + Send,
	K: redis::ToRedisArgs + Send + Sync + Clone,
	V: Serialize,
{
	let mut cmd = redis::cmd("MSET");
	for (k, v) in pairs {
		let json = serde_json::to_string(v)?;
		cmd.arg(k).arg(json);
	}
	cmd.query_async::<()>(conn).await?;
	Ok(())
}

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>;

	use super::*;
	use crate::_dev_utils::{clean_test_prefix, init_test, test_key_prefix};
	use serde::{Deserialize, Serialize};
	use serial_test::serial;

	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
	struct User {
		name: String,
		age: u32,
	}

	#[serial]
	#[tokio::test]
	async fn test_set_get_json_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_json");
		let key = format!("{}:user", prefix);

		let user = User {
			name: "Alice".to_string(),
			age: 30,
		};

		// -- Exec
		set_json(&mut *conn, &key, &user, None).await?;
		let result: Option<User> = get_json(&mut *conn, &key).await?;

		// -- Check
		assert_eq!(result, Some(user));

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_get_json_required_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_json_req");
		let key = format!("{}:data", prefix);

		#[derive(Debug, Serialize, Deserialize, PartialEq)]
		struct Config {
			debug: bool,
		}

		let config = Config { debug: true };

		// -- Exec
		set_json(&mut *conn, &key, &config, None).await?;
		let result: Config = get_json_required(&mut *conn, &key).await?;

		// -- Check
		assert_eq!(result, config);

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_hset_hget_json_ok() -> Result<()> {
		// -- Setup & Fixtures
		let pool = init_test().await;
		let mut conn = pool.get().await?;
		let prefix = test_key_prefix("test_hjson");
		let key = format!("{}:users", prefix);

		let user1 = User { name: "Bob".to_string(), age: 25 };
		let user2 = User { name: "Carol".to_string(), age: 35 };

		// -- Exec
		hset_json(&mut *conn, &key, "user:1", &user1).await?;
		hset_json(&mut *conn, &key, "user:2", &user2).await?;

		let r1: Option<User> = hget_json(&mut *conn, &key, "user:1").await?;
		let r2: Option<User> = hget_json(&mut *conn, &key, "user:2").await?;

		// -- Check
		assert_eq!(r1, Some(user1));
		assert_eq!(r2, Some(user2));

		// -- Clean
		clean_test_prefix(&mut *conn, &prefix).await?;

		Ok(())
	}
}

// endregion: --- Tests
