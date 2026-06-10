use crate::model::Result;
use lib_valkey_core::{cache_keys::CacheKey, commands, ValkeyPool};
use serde::Deserialize;
use serde::{de::DeserializeOwned, Serialize};
use std::future::Future;
use tracing::warn;

pub const MODEL_ENTITY_TTL_SECS: u64 = 300;
pub const MODEL_QUERY_TTL_SECS: u64 = 60;

#[derive(Debug, Clone, Copy, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CachePolicy {
	#[default]
	Use,
	Refresh,
	Bypass,
}

impl CachePolicy {
	pub fn read_enabled(self) -> bool {
		matches!(self, CachePolicy::Use)
	}

	pub fn write_enabled(self) -> bool {
		matches!(self, CachePolicy::Use | CachePolicy::Refresh)
	}
}

#[derive(Debug, Clone)]
pub struct RegisteredModelCache {
	pub resource: &'static str,
	pub table: &'static str,
	pub source: &'static str,
}

inventory::collect!(RegisteredModelCache);

pub fn registered_model_caches() -> Vec<&'static RegisteredModelCache> {
	inventory::iter::<RegisteredModelCache>
		.into_iter()
		.collect()
}

pub fn model_cache_required() -> bool {
	inventory::iter::<RegisteredModelCache>
		.into_iter()
		.next()
		.is_some()
}

pub fn model_entity_key(table: &str, id: i64) -> String {
	format!("model:{table}:id:{id}")
}

pub fn model_query_version_key(table: &str) -> String {
	format!("model:{table}:list:version")
}

pub fn model_query_key<T>(table: &str, operation: &str, version: i64, args: &T) -> Option<String>
where
	T: Serialize + ?Sized,
{
	let args_json = match serde_json::to_vec(args) {
		Ok(args_json) => args_json,
		Err(err) => {
			warn!(
				table,
				operation,
				error = ?err,
				"model cache query key serialization failed"
			);
			return None;
		}
	};

	let args_hash = blake3::hash(&args_json).to_hex().to_string();
	Some(format!("model:{table}:{operation}:v{version}:{args_hash}"))
}

pub async fn get_or_load_json<T, F, Fut>(
	cache_pool: Option<&ValkeyPool>,
	key: Option<&str>,
	ttl_secs: Option<u64>,
	policy: CachePolicy,
	loader: F,
) -> Result<T>
where
	T: Serialize + DeserializeOwned,
	F: FnOnce() -> Fut,
	Fut: Future<Output = Result<T>>,
{
	if policy.read_enabled() {
		if let (Some(cache_pool), Some(key)) = (cache_pool, key) {
			match cache_pool.get().await {
				Ok(mut conn) => match commands::get_json::<_, _, T>(&mut *conn, key).await {
					Ok(Some(value)) => return Ok(value),
					Ok(None) => {}
					Err(err) => {
						warn!(key, error = ?err, "model cache read failed");
						let _ = commands::del_one(&mut *conn, key).await;
					}
				},
				Err(err) => {
					warn!(key, error = ?err, "model cache connection failed");
				}
			}
		}
	}

	let value = loader().await?;

	if policy.write_enabled() {
		if let (Some(cache_pool), Some(key)) = (cache_pool, key) {
			write_json_best_effort(cache_pool, key, &value, ttl_secs).await;
		}
	}

	Ok(value)
}

pub async fn write_json_best_effort<T>(
	cache_pool: &ValkeyPool,
	key: &str,
	value: &T,
	ttl_secs: Option<u64>,
) where
	T: Serialize,
{
	match cache_pool.get().await {
		Ok(mut conn) => {
			if let Err(err) = commands::set_json(&mut *conn, key, value, ttl_secs).await {
				warn!(key, error = ?err, "model cache write failed");
			}
		}
		Err(err) => {
			warn!(key, error = ?err, "model cache connection failed");
		}
	}
}

pub async fn write_model_entity_best_effort<T>(
	cache_pool: Option<&ValkeyPool>,
	table: &str,
	id: i64,
	entity: &T,
) where
	T: Serialize,
{
	let Some(cache_pool) = cache_pool else {
		return;
	};

	let key = model_entity_key(table, id);
	write_json_best_effort(cache_pool, &key, entity, Some(MODEL_ENTITY_TTL_SECS)).await;
}

pub async fn delete_model_entity_best_effort(
	cache_pool: Option<&ValkeyPool>,
	table: &str,
	id: i64,
) {
	let Some(cache_pool) = cache_pool else {
		return;
	};

	let key = model_entity_key(table, id);
	delete_keys_best_effort(cache_pool, &[key]).await;
}

pub async fn delete_model_entities_best_effort(
	cache_pool: Option<&ValkeyPool>,
	table: &str,
	ids: &[i64],
) {
	let Some(cache_pool) = cache_pool else {
		return;
	};

	if ids.is_empty() {
		return;
	}

	let keys = ids
		.iter()
		.map(|id| model_entity_key(table, *id))
		.collect::<Vec<_>>();
	delete_keys_best_effort(cache_pool, &keys).await;
}

pub async fn model_query_version(cache_pool: Option<&ValkeyPool>, table: &str) -> i64 {
	let Some(cache_pool) = cache_pool else {
		return 0;
	};

	let key = model_query_version_key(table);
	match cache_pool.get().await {
		Ok(mut conn) => match commands::get::<_, _, i64>(&mut *conn, &key).await {
			Ok(Some(version)) => version,
			Ok(None) => 0,
			Err(err) => {
				warn!(key, error = ?err, "model cache version read failed");
				0
			}
		},
		Err(err) => {
			warn!(key, error = ?err, "model cache connection failed");
			0
		}
	}
}

pub async fn bump_model_query_version_best_effort(cache_pool: Option<&ValkeyPool>, table: &str) {
	let Some(cache_pool) = cache_pool else {
		return;
	};

	let key = model_query_version_key(table);
	match cache_pool.get().await {
		Ok(mut conn) => {
			if let Err(err) = commands::incr(&mut *conn, &key).await {
				warn!(key, error = ?err, "model cache version bump failed");
			}
		}
		Err(err) => {
			warn!(key, error = ?err, "model cache connection failed");
		}
	}
}

pub async fn invalidate_user_permissions_cache_best_effort(
	cache_pool: Option<&ValkeyPool>,
	user_id: i64,
) {
	let Some(cache_pool) = cache_pool else {
		return;
	};

	let key = CacheKey::UserPermissions(user_id).as_str();
	delete_keys_best_effort(cache_pool, &[key]).await;
}

pub async fn invalidate_users_permissions_cache_best_effort(
	cache_pool: Option<&ValkeyPool>,
	user_ids: &[i64],
) {
	let Some(cache_pool) = cache_pool else {
		return;
	};

	let keys = user_ids
		.iter()
		.map(|user_id| CacheKey::UserPermissions(*user_id).as_str())
		.collect::<Vec<_>>();
	delete_keys_best_effort(cache_pool, &keys).await;
}

async fn delete_keys_best_effort(cache_pool: &ValkeyPool, keys: &[String]) {
	if keys.is_empty() {
		return;
	}

	match cache_pool.get().await {
		Ok(mut conn) => {
			if let Err(err) = commands::del(&mut *conn, keys).await {
				warn!(keys = ?keys, error = ?err, "model cache delete failed");
			}
		}
		Err(err) => {
			warn!(keys = ?keys, error = ?err, "model cache connection failed");
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde::Serialize;

	#[derive(Serialize)]
	struct QueryArgs {
		name: &'static str,
		limit: i64,
	}

	#[test]
	fn test_model_entity_key_ok() {
		assert_eq!(model_entity_key("agent", 10), "model:agent:id:10");
	}

	#[test]
	fn test_model_query_version_key_ok() {
		assert_eq!(model_query_version_key("agent"), "model:agent:list:version");
	}

	#[test]
	fn test_model_query_key_is_stable() {
		let args = QueryArgs {
			name: "demo",
			limit: 10,
		};

		let key_a = model_query_key("agent", "list", 3, &args).unwrap();
		let key_b = model_query_key("agent", "list", 3, &args).unwrap();

		assert_eq!(key_a, key_b);
		assert!(key_a.starts_with("model:agent:list:v3:"));
	}

	#[test]
	fn test_cache_policy_deserialize_ok() {
		assert_eq!(
			serde_json::from_str::<CachePolicy>("\"use\"").unwrap(),
			CachePolicy::Use
		);
		assert_eq!(
			serde_json::from_str::<CachePolicy>("\"refresh\"").unwrap(),
			CachePolicy::Refresh
		);
		assert_eq!(
			serde_json::from_str::<CachePolicy>("\"bypass\"").unwrap(),
			CachePolicy::Bypass
		);
	}
}
