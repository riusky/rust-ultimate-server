use lib_config::app_config;
use std::sync::OnceLock;

pub fn web_config() -> &'static WebConfig {
	static INSTANCE: OnceLock<WebConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| WebConfig::load())
}

#[allow(non_snake_case)]
pub struct WebConfig {
	pub WEB_FOLDER: String,
	/// Enable Valkey caching for user permissions
	pub PERMISSION_CACHE_ENABLED: bool,
	/// Enable Valkey caching for cache-capable BMC models
	pub MODEL_CACHE_ENABLED: bool,
	/// Initial admin username used when bootstrapping development data
	pub INIT_ADMIN_USERNAME: String,
	/// Optional initial admin password; only applied if the user has no password
	pub INIT_ADMIN_PASSWORD: Option<String>,
}

impl WebConfig {
	fn load() -> WebConfig {
		let config = app_config();

		WebConfig {
			WEB_FOLDER: config.app.web_folder.clone(),
			PERMISSION_CACHE_ENABLED: config.cache.permission_enabled,
			MODEL_CACHE_ENABLED: config.cache.model_enabled,
			INIT_ADMIN_USERNAME: config.initial_admin.username.clone(),
			INIT_ADMIN_PASSWORD: config.initial_admin.password.clone(),
		}
	}
}
