use lib_config::app_config;
use std::sync::OnceLock;

pub fn core_config() -> &'static CoreConfig {
	static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		CoreConfig::load().unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
	})
}

#[allow(non_snake_case)]
pub struct CoreConfig {
	// -- Db
	pub DB_URL: String,

	// -- Web
	pub WEB_FOLDER: String,
}

impl CoreConfig {
	fn load() -> lib_config::Result<CoreConfig> {
		let config = app_config();

		Ok(CoreConfig {
			// -- Db
			DB_URL: config.db.require_url()?.to_string(),

			// -- Web
			WEB_FOLDER: config.app.web_folder.clone(),
		})
	}
}
