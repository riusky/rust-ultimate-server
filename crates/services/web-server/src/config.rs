use lib_utils::envs::get_env;
use std::sync::OnceLock;

pub fn web_config() -> &'static WebConfig {
	static INSTANCE: OnceLock<WebConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		WebConfig::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct WebConfig {
	pub WEB_FOLDER: String,
	/// Enable Valkey caching for user permissions
	pub PERMISSION_CACHE_ENABLED: bool,
}

impl WebConfig {
	fn load_from_env() -> lib_utils::envs::Result<WebConfig> {
		Ok(WebConfig {
			WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
			PERMISSION_CACHE_ENABLED: get_env("SERVICE_PERMISSION_CACHE_ENABLED")
				.unwrap_or_else(|_| "false".to_string())
				.parse()
				.unwrap_or(false),
		})
	}
}
