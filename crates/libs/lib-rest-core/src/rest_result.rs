//! The response module normalizes the REST API response format.
//!
//! The primary types are:
//! - `RestResponse` - Standard response wrapper (200 OK)
//! - `RestCreated` - Response for create operations (201 Created)
//! - `RestDeleted` - Response for delete operations (200 OK with deleted entity)

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

/// Standard REST API response wrapper
/// Returns HTTP 200 OK with JSON body: { "data": ... }
#[derive(Debug, Serialize)]
pub struct RestResponse<T>
where
	T: Serialize,
{
	pub data: T,
}

impl<T> From<T> for RestResponse<T>
where
	T: Serialize,
{
	fn from(val: T) -> Self {
		Self { data: val }
	}
}

impl<T> IntoResponse for RestResponse<T>
where
	T: Serialize,
{
	fn into_response(self) -> Response {
		(StatusCode::OK, Json(self)).into_response()
	}
}

/// REST response for create operations
/// Returns HTTP 201 Created with JSON body: { "data": ... }
#[derive(Debug, Serialize)]
pub struct RestCreated<T>
where
	T: Serialize,
{
	pub data: T,
}

impl<T> From<T> for RestCreated<T>
where
	T: Serialize,
{
	fn from(val: T) -> Self {
		Self { data: val }
	}
}

impl<T> IntoResponse for RestCreated<T>
where
	T: Serialize,
{
	fn into_response(self) -> Response {
		(StatusCode::CREATED, Json(self)).into_response()
	}
}

/// REST response for delete operations
/// Returns HTTP 200 OK with JSON body containing the deleted entity: { "data": ... }
#[derive(Debug, Serialize)]
pub struct RestDeleted<T>
where
	T: Serialize,
{
	pub data: T,
}

impl<T> From<T> for RestDeleted<T>
where
	T: Serialize,
{
	fn from(val: T) -> Self {
		Self { data: val }
	}
}

impl<T> IntoResponse for RestDeleted<T>
where
	T: Serialize,
{
	fn into_response(self) -> Response {
		(StatusCode::OK, Json(self)).into_response()
	}
}
