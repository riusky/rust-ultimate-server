use lib_config::app_config;
use lib_utils::b64::b64u_decode;
use std::sync::OnceLock;

pub fn auth_config() -> &'static AuthConfig {
	static INSTANCE: OnceLock<AuthConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		AuthConfig::load().unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
	})
}

#[allow(non_snake_case)]
pub struct AuthConfig {
	// -- Crypt
	pub PWD_KEY: Vec<u8>,

	pub TOKEN_KEY: Vec<u8>,
	pub TOKEN_DURATION_SEC: f64,
}

impl AuthConfig {
	fn load() -> Result<AuthConfig> {
		let config = app_config();
		let pwd_key = config.auth.require_pwd_key()?;
		let token_key = config.auth.require_token_key()?;

		Ok(AuthConfig {
			// -- Crypt
			PWD_KEY: b64u_decode(pwd_key).map_err(|_| Error::WrongFormat("auth.pwd_key"))?,

			TOKEN_KEY: b64u_decode(token_key).map_err(|_| Error::WrongFormat("auth.token_key"))?,
			TOKEN_DURATION_SEC: config.auth.token_duration_sec,
		})
	}
}

type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
enum Error {
	Config(lib_config::Error),
	WrongFormat(&'static str),
}

impl From<lib_config::Error> for Error {
	fn from(value: lib_config::Error) -> Self {
		Self::Config(value)
	}
}

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
		match self {
			Self::Config(err) => write!(fmt, "{err}"),
			Self::WrongFormat(name) => write!(fmt, "wrong format for {name}"),
		}
	}
}

impl std::error::Error for Error {}
