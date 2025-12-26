use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
	// -- Redis/Valkey errors
	#[from]
	Redis(redis::RedisError),

	// -- Pool errors
	PoolTimeout,
	PoolError(String),

	// -- Lock errors
	LockAcquireFailed,
	LockReleaseFailed,

	// -- Serialization errors
	#[from]
	SerdeJson(serde_json::Error),

	// -- Custom errors
	KeyNotFound {
		key: String,
	},
	InvalidValue {
		key: String,
		expected: &'static str,
	},
	Custom {
		message: String,
	},
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{self:?}")
	}
}

impl std::error::Error for Error {}

impl<E> From<bb8::RunError<E>> for Error
where
	E: std::fmt::Debug,
{
	fn from(err: bb8::RunError<E>) -> Self {
		match err {
			bb8::RunError::TimedOut => Error::PoolTimeout,
			bb8::RunError::User(e) => Error::PoolError(format!("{:?}", e)),
		}
	}
}
