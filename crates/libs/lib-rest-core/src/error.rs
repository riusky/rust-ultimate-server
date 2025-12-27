//! This module encompasses errors for all `lib_rest` modules and rest handlers.
//! Variants from our application's library errors can be added as required by the handlers.

use derive_more::From;
use lib_macros::IntoResponseExt;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, Serialize, IntoResponseExt)]
pub enum Error {
	// -- App Libs
	#[from]
	Model(lib_core::model::Error),

	#[from]
	Ctx(lib_core::ctx::Error),

	// -- External Modules
	#[from]
	SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
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
