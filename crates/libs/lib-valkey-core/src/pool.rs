use crate::config::valkey_config;
use crate::Result;
use bb8_redis::RedisConnectionManager;
use tracing::info;

pub type ValkeyManager = RedisConnectionManager;
pub type ValkeyPool = bb8::Pool<ValkeyManager>;
pub type ValkeyConnection<'a> = bb8::PooledConnection<'a, ValkeyManager>;

/// Create a new Valkey connection pool with custom settings
pub async fn new_valkey_pool_with_config(
	url: &str,
	max_size: u32,
	min_idle: u32,
) -> Result<ValkeyPool> {
	info!("Connecting to Valkey at {}", url);

	let manager = RedisConnectionManager::new(url)?;

	let pool = bb8::Pool::builder()
		.max_size(max_size)
		.min_idle(Some(min_idle))
		.build(manager)
		.await?;

	// Test connection
	{
		let mut conn = pool.get().await?;
		redis::cmd("PING")
			.query_async::<String>(&mut *conn)
			.await?;
	}

	info!("Successfully connected to Valkey");

	Ok(pool)
}

/// Create a new Valkey connection pool using config from environment
pub async fn new_valkey_pool() -> Result<ValkeyPool> {
	let config = valkey_config();
	new_valkey_pool_with_config(
		&config.VALKEY_URL,
		config.VALKEY_POOL_MAX_SIZE,
		config.VALKEY_POOL_MIN_IDLE,
	)
	.await
}
