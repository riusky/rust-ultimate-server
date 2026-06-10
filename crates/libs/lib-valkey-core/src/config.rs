use lib_config::app_config;
use std::sync::OnceLock;

pub fn valkey_config() -> &'static ValkeyConfig {
	static INSTANCE: OnceLock<ValkeyConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| ValkeyConfig::load())
}

#[allow(non_snake_case)]
pub struct ValkeyConfig {
	pub VALKEY_URL: String,
	pub VALKEY_POOL_MAX_SIZE: u32,
	pub VALKEY_POOL_MIN_IDLE: u32,
}

impl ValkeyConfig {
	fn load() -> ValkeyConfig {
		let config = app_config();

		ValkeyConfig {
			VALKEY_URL: config.valkey.url.clone(),
			VALKEY_POOL_MAX_SIZE: config.valkey.pool_max_size,
			VALKEY_POOL_MIN_IDLE: config.valkey.pool_min_idle,
		}
	}
}
