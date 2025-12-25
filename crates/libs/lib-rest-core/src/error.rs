//! This module encompasses errors for all `lib_rest` modules and rest handlers.
//! Variants from our application's library errors can be added as required by the handlers.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, Serialize)]
pub enum Error {
	// -- App Libs
	#[from]
	Model(lib_core::model::Error),

	// -- External Modules
	#[from]
	SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		let status = match &self {
			Error::Model(model_err) => match model_err {
				lib_core::model::Error::EntityNotFound { .. } => StatusCode::NOT_FOUND,
				_ => StatusCode::INTERNAL_SERVER_ERROR,
			},
			Error::SerdeJson(_) => StatusCode::BAD_REQUEST,
		};

		let body = serde_json::json!({
			"error": {
				"message": format!("{self:?}"),
			}
		});

		(status, axum::Json(body)).into_response()
	}
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
