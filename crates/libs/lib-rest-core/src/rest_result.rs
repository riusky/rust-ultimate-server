//! The response module normalizes the REST API response format.
//!
//! All REST responses follow a unified format with `success: true` for successful operations:
//! - `RestResponse` - Standard response wrapper (200 OK)
//! - `RestCreated` - Response for create operations (201 Created)
//! - `RestDeleted` - Response for delete operations (200 OK with deleted entity)
//! - `RestPagedResponse` - Response for paginated list operations (200 OK with pagination info)
//!
//! Response format:
//! ```json
//! {
//!     "success": true,
//!     "data": { ... }
//! }
//! ```

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

/// Standard REST API response wrapper
/// Returns HTTP 200 OK with JSON body: { "success": true, "data": ... }
#[derive(Debug, Serialize)]
pub struct RestResponse<T>
where
	T: Serialize,
{
	/// Always true for successful responses
	pub success: bool,
	/// The response data
	pub data: T,
}

impl<T> From<T> for RestResponse<T>
where
	T: Serialize,
{
	fn from(val: T) -> Self {
		Self {
			success: true,
			data: val,
		}
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
/// Returns HTTP 201 Created with JSON body: { "success": true, "data": ... }
#[derive(Debug, Serialize)]
pub struct RestCreated<T>
where
	T: Serialize,
{
	/// Always true for successful responses
	pub success: bool,
	/// The created entity data
	pub data: T,
}

impl<T> From<T> for RestCreated<T>
where
	T: Serialize,
{
	fn from(val: T) -> Self {
		Self {
			success: true,
			data: val,
		}
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
/// Returns HTTP 200 OK with JSON body: { "success": true, "data": ... }
#[derive(Debug, Serialize)]
pub struct RestDeleted<T>
where
	T: Serialize,
{
	/// Always true for successful responses
	pub success: bool,
	/// The deleted entity data
	pub data: T,
}

impl<T> From<T> for RestDeleted<T>
where
	T: Serialize,
{
	fn from(val: T) -> Self {
		Self {
			success: true,
			data: val,
		}
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

/// Pagination metadata for paged responses
#[derive(Debug, Serialize)]
pub struct PageInfo {
	/// Total number of items matching the query
	pub total: i64,
	/// Number of items per page
	pub page_size: i64,
	/// Current page number (1-based)
	pub page_number: i64,
	/// Total number of pages
	pub total_pages: i64,
	/// Whether there are more pages after this one
	pub has_more: bool,
}

impl PageInfo {
	pub fn new(total: i64, page_size: i64, page_number: i64) -> Self {
		let total_pages = if page_size > 0 {
			(total + page_size - 1) / page_size
		} else {
			0
		};
		Self {
			total,
			page_size,
			page_number,
			total_pages,
			has_more: page_number < total_pages,
		}
	}
}

/// REST response for paginated list operations
/// Returns HTTP 200 OK with JSON body:
/// {
///   "success": true,
///   "data": [...],
///   "page_info": {
///     "total": 100,
///     "page_size": 10,
///     "page_number": 1,
///     "total_pages": 10,
///     "has_more": true
///   }
/// }
#[derive(Debug, Serialize)]
pub struct RestPagedResponse<T>
where
	T: Serialize,
{
	/// Always true for successful responses
	pub success: bool,
	/// The list of items for the current page
	pub data: Vec<T>,
	/// Pagination metadata
	pub page_info: PageInfo,
}

impl<T> RestPagedResponse<T>
where
	T: Serialize,
{
	pub fn new(data: Vec<T>, total: i64, page_size: i64, page_number: i64) -> Self {
		Self {
			success: true,
			data,
			page_info: PageInfo::new(total, page_size, page_number),
		}
	}
}

impl<T> IntoResponse for RestPagedResponse<T>
where
	T: Serialize,
{
	fn into_response(self) -> Response {
		(StatusCode::OK, Json(self)).into_response()
	}
}
