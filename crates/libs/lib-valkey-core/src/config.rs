use lib_utils::envs::get_env;
use std::sync::OnceLock;

pub fn valkey_config() -> &'static ValkeyConfig {
	static INSTANCE: OnceLock<ValkeyConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		ValkeyConfig::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING VALKEY CONF - Cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct ValkeyConfig {
	pub VALKEY_URL: String,
	pub VALKEY_POOL_MAX_SIZE: u32,
	pub VALKEY_POOL_MIN_IDLE: u32,
}

impl ValkeyConfig {
	fn load_from_env() -> lib_utils::envs::Result<ValkeyConfig> {
		Ok(ValkeyConfig {
			VALKEY_URL: get_env("SERVICE_VALKEY_URL")
				.unwrap_or_else(|_| "redis://localhost:6379".to_string()),
			VALKEY_POOL_MAX_SIZE: get_env("SERVICE_VALKEY_POOL_MAX_SIZE")
				.unwrap_or_else(|_| "10".to_string())
				.parse()
				.unwrap_or(10),
			VALKEY_POOL_MIN_IDLE: get_env("SERVICE_VALKEY_POOL_MIN_IDLE")
				.unwrap_or_else(|_| "2".to_string())
				.parse()
				.unwrap_or(2),
		})
	}
}
