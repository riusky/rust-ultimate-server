use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, Clone)]
pub enum Error {
	CtxCannotNewRootCtx,

	// -- Permission errors
	PermissionDenied {
		user_id: i64,
		permission: String,
	},
	PermissionAnyDenied {
		user_id: i64,
		permissions: Vec<String>,
	},
	PermissionsNotLoaded,
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
