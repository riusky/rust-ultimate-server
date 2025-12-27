//! Middleware to transform lib_rest_core::Error into lib_web::Error.
//!
//! This middleware should be applied at the REST router layer to convert
//! REST-specific errors into the unified lib_web::Error type, allowing
//! mw_res_map to handle all errors uniformly.

use crate::error::Error;
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use std::sync::Arc;
use tracing::warn;

/// Middleware that wraps lib_rest_core::Error into lib_web::Error::Rest.
///
/// This allows mw_res_map to only handle lib_web::Error, simplifying
/// the error handling flow and matching the RPC error handling pattern.
pub async fn mw_rest_error(req: Request<Body>, next: Next) -> Response {
	let mut res = next.run(req).await;

	// Check if there's a lib_rest_core::Error in extensions
	if let Some(rest_error) = res.extensions_mut().remove::<Arc<lib_rest_core::Error>>() {
		// Convert to lib_web::Error::Rest
		// Arc::try_unwrap should succeed since we just removed the only reference
		let web_error = match Arc::try_unwrap(rest_error) {
			Ok(err) => Error::Rest(err),
			Err(_) => {
				// This should not happen in normal flow
				warn!("Failed to unwrap Arc<lib_rest_core::Error>, multiple references exist");
				return res;
			}
		};

		// Insert the wrapped error
		res.extensions_mut().insert(Arc::new(web_error));
	}

	res
}
