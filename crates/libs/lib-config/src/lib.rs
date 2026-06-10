use config::{Config, File};
use serde::Deserialize;
use std::env;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::OnceLock;

pub fn app_config() -> &'static AppConfig {
	static INSTANCE: OnceLock<AppConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		AppConfig::load()
			.unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING APP CONFIG - Cause: {ex:?}"))
	})
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(default)]
pub struct AppConfig {
	pub app: AppSection,
	pub db: DbSection,
	pub auth: AuthSection,
	pub valkey: ValkeySection,
	pub cache: CacheSection,
	pub initial_admin: InitialAdminSection,
}

impl Default for AppConfig {
	fn default() -> Self {
		Self {
			app: AppSection::default(),
			db: DbSection::default(),
			auth: AuthSection::default(),
			valkey: ValkeySection::default(),
			cache: CacheSection::default(),
			initial_admin: InitialAdminSection::default(),
		}
	}
}

impl AppConfig {
	pub fn load() -> Result<Self> {
		let config_dir = env::var("SERVICE_CONFIG_DIR").unwrap_or_else(|_| "config".to_string());
		let env_name = env::var("SERVICE_ENV").unwrap_or_else(|_| "development".to_string());
		let config_file = env::var("SERVICE_CONFIG_FILE").ok();

		let mut config = Self::load_from_files(Path::new(&config_dir), &env_name, config_file)?;
		config.apply_env_overrides()?;
		Ok(config)
	}

	pub fn load_from_files(
		config_dir: &Path,
		env_name: &str,
		config_file: Option<String>,
	) -> Result<Self> {
		let mut builder = Config::builder()
			.add_source(File::from(config_dir.join("default.toml")).required(false))
			.add_source(File::from(config_dir.join(format!("{env_name}.toml"))).required(false));

		if let Some(config_file) = config_file {
			builder = builder.add_source(File::from(PathBuf::from(config_file)).required(false));
		}

		builder = builder.add_source(File::from(config_dir.join("local.toml")).required(false));

		builder
			.build()
			.map_err(Error::Config)?
			.try_deserialize()
			.map_err(Error::Config)
	}

	fn apply_env_overrides(&mut self) -> Result<()> {
		self.apply_env_overrides_from(|name| env::var(name).ok())
	}

	fn apply_env_overrides_from<F>(&mut self, get_env: F) -> Result<()>
	where
		F: Fn(&'static str) -> Option<String>,
	{
		if let Some(value) = get_env("SERVICE_WEB_FOLDER") {
			self.app.web_folder = value;
		}
		if let Some(value) = get_env("SERVICE_DB_URL") {
			self.db.url = Some(value);
		}
		if let Some(value) = get_env("SERVICE_PWD_KEY") {
			self.auth.pwd_key = Some(value);
		}
		if let Some(value) = get_env("SERVICE_TOKEN_KEY") {
			self.auth.token_key = Some(value);
		}
		if let Some(value) = get_env("SERVICE_TOKEN_DURATION_SEC") {
			self.auth.token_duration_sec = parse_env("SERVICE_TOKEN_DURATION_SEC", &value)?;
		}
		if let Some(value) = get_env("SERVICE_VALKEY_URL") {
			self.valkey.url = value;
		}
		if let Some(value) = get_env("SERVICE_VALKEY_POOL_MAX_SIZE") {
			self.valkey.pool_max_size = parse_env("SERVICE_VALKEY_POOL_MAX_SIZE", &value)?;
		}
		if let Some(value) = get_env("SERVICE_VALKEY_POOL_MIN_IDLE") {
			self.valkey.pool_min_idle = parse_env("SERVICE_VALKEY_POOL_MIN_IDLE", &value)?;
		}
		if let Some(value) = get_env("SERVICE_PERMISSION_CACHE_ENABLED") {
			self.cache.permission_enabled = parse_env("SERVICE_PERMISSION_CACHE_ENABLED", &value)?;
		}
		if let Some(value) = get_env("SERVICE_MODEL_CACHE_ENABLED") {
			self.cache.model_enabled = parse_env("SERVICE_MODEL_CACHE_ENABLED", &value)?;
		}
		if let Some(value) = get_env("INIT_ADMIN_USERNAME") {
			self.initial_admin.username = value;
		}
		if let Some(value) = get_env("INIT_ADMIN_PASSWORD") {
			self.initial_admin.password = Some(value);
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(default)]
pub struct AppSection {
	pub web_folder: String,
}

impl Default for AppSection {
	fn default() -> Self {
		Self {
			web_folder: "web-folder/".to_string(),
		}
	}
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct DbSection {
	pub url: Option<String>,
}

impl DbSection {
	pub fn require_url(&self) -> Result<&str> {
		self.url
			.as_deref()
			.ok_or(Error::MissingRequired("db.url or SERVICE_DB_URL"))
	}
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(default)]
pub struct AuthSection {
	pub pwd_key: Option<String>,
	pub token_key: Option<String>,
	pub token_duration_sec: f64,
}

impl Default for AuthSection {
	fn default() -> Self {
		Self {
			pwd_key: None,
			token_key: None,
			token_duration_sec: 1800.0,
		}
	}
}

impl AuthSection {
	pub fn require_pwd_key(&self) -> Result<&str> {
		self.pwd_key
			.as_deref()
			.ok_or(Error::MissingRequired("auth.pwd_key or SERVICE_PWD_KEY"))
	}

	pub fn require_token_key(&self) -> Result<&str> {
		self.token_key.as_deref().ok_or(Error::MissingRequired(
			"auth.token_key or SERVICE_TOKEN_KEY",
		))
	}
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(default)]
pub struct ValkeySection {
	pub url: String,
	pub pool_max_size: u32,
	pub pool_min_idle: u32,
}

impl Default for ValkeySection {
	fn default() -> Self {
		Self {
			url: "redis://localhost:6379".to_string(),
			pool_max_size: 10,
			pool_min_idle: 2,
		}
	}
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct CacheSection {
	pub permission_enabled: bool,
	pub model_enabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(default)]
pub struct InitialAdminSection {
	pub username: String,
	pub password: Option<String>,
}

impl Default for InitialAdminSection {
	fn default() -> Self {
		Self {
			username: "admin".to_string(),
			password: None,
		}
	}
}

fn parse_env<T>(name: &'static str, value: &str) -> Result<T>
where
	T: FromStr,
{
	value.parse().map_err(|_| Error::InvalidEnv {
		name,
		value: value.to_string(),
	})
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	Config(config::ConfigError),
	InvalidEnv { name: &'static str, value: String },
	MissingRequired(&'static str),
}

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default_config_has_safe_local_values() {
		let config = AppConfig::default();

		assert_eq!(config.app.web_folder, "web-folder/");
		assert_eq!(config.valkey.url, "redis://localhost:6379");
		assert!(!config.cache.permission_enabled);
		assert!(!config.cache.model_enabled);
		assert_eq!(config.initial_admin.username, "admin");
	}

	#[test]
	fn legacy_env_overrides_file_values() {
		let mut config = AppConfig {
			db: DbSection {
				url: Some("postgres://file".to_string()),
			},
			cache: CacheSection {
				permission_enabled: false,
				model_enabled: false,
			},
			..AppConfig::default()
		};

		config
			.apply_env_overrides_from(|name| match name {
				"SERVICE_DB_URL" => Some("postgres://env".to_string()),
				"SERVICE_PERMISSION_CACHE_ENABLED" => Some("true".to_string()),
				"SERVICE_MODEL_CACHE_ENABLED" => Some("true".to_string()),
				"INIT_ADMIN_PASSWORD" => Some("admin".to_string()),
				_ => None,
			})
			.unwrap();

		assert_eq!(config.db.url.as_deref(), Some("postgres://env"));
		assert!(config.cache.permission_enabled);
		assert!(config.cache.model_enabled);
		assert_eq!(config.initial_admin.password.as_deref(), Some("admin"));
	}

	#[test]
	fn invalid_env_values_fail_fast() {
		let mut config = AppConfig::default();

		let err = config
			.apply_env_overrides_from(|name| match name {
				"SERVICE_MODEL_CACHE_ENABLED" => Some("sometimes".to_string()),
				_ => None,
			})
			.unwrap_err();

		assert!(matches!(
			err,
			Error::InvalidEnv {
				name: "SERVICE_MODEL_CACHE_ENABLED",
				..
			}
		));
	}
}
