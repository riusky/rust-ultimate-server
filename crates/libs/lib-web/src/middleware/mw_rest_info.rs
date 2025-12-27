//! Middleware to inject ApiInfo for REST requests.
//!
//! This middleware analyzes REST API paths and injects appropriate ApiInfo
//! into the response extensions for unified logging and error handling.

use crate::handlers::api_handlers::ApiInfo;
use axum::body::Body;
use axum::http::{Method, Request, Uri};
use axum::middleware::Next;
use axum::response::Response;
use std::sync::Arc;
use tracing::debug;

/// Middleware that injects REST ApiInfo based on request path and method.
///
/// This should be applied to REST routes to enable unified logging and error handling.
///
/// Path pattern: `/api/rest/{resource}[/{id}][/{action}]`
///
/// Examples:
/// - `GET /api/rest/agents` -> resource: "agent", action: "list"
/// - `GET /api/rest/agents/123` -> resource: "agent", action: "get"
/// - `POST /api/rest/agents` -> resource: "agent", action: "create"
/// - `PUT /api/rest/agents/123` -> resource: "agent", action: "update"
/// - `DELETE /api/rest/agents/123` -> resource: "agent", action: "delete"
/// - `GET /api/rest/agents/paged` -> resource: "agent", action: "list_paged"
/// - `GET /api/rest/agents/123/export` -> resource: "agent", action: "export"
pub async fn mw_rest_info(req: Request<Body>, next: Next) -> Response {
	let method = req.method().clone();
	let uri = req.uri().clone();

	// Run the next handler
	let mut res = next.run(req).await;

	// Only inject if no ApiInfo is already set (don't override RPC info)
	if res.extensions().get::<Arc<ApiInfo>>().is_none() {
		if let Some(api_info) = parse_rest_api_info(&method, &uri) {
			debug!("{:<12} - mw_rest_info: {:?}", "MW", api_info);
			res.extensions_mut().insert(Arc::new(api_info));
		}
	}

	res
}

/// Parse REST API path to extract resource and action.
fn parse_rest_api_info(method: &Method, uri: &Uri) -> Option<ApiInfo> {
	let path = uri.path();

	// Check if this is a REST API path
	// Pattern: /api/rest/{resource}[/{id_or_action}][/{action}]
	if !path.starts_with("/api/rest/") {
		return None;
	}

	// Remove the /api/rest/ prefix
	let rest_path = &path["/api/rest/".len()..];

	// Split by '/'
	let parts: Vec<&str> = rest_path.split('/').filter(|s| !s.is_empty()).collect();

	if parts.is_empty() {
		return None;
	}

	// First part is the resource (pluralized usually, e.g., "agents")
	let resource_plural = parts[0];
	// Singularize: remove trailing 's' if present (simple heuristic)
	let resource = singularize(resource_plural);

	// Determine action based on method and path structure
	let action = match parts.len() {
		1 => {
			// /api/rest/{resource}
			match *method {
				Method::GET => "list",
				Method::POST => "create",
				_ => "unknown",
			}
		}
		2 => {
			// /api/rest/{resource}/{id_or_action}
			let second = parts[1];
			if second == "paged" {
				"list_paged"
			} else if is_numeric_or_uuid(second) {
				// It's an ID
				match *method {
					Method::GET => "get",
					Method::PUT | Method::PATCH => "update",
					Method::DELETE => "delete",
					_ => "unknown",
				}
			} else {
				// It's a custom action
				second
			}
		}
		_ => {
			// /api/rest/{resource}/{id}/{action} or more
			// The last non-ID part is likely the action
			if parts.len() >= 3 {
				let last = parts.last().unwrap();
				if !is_numeric_or_uuid(last) {
					*last
				} else {
					match *method {
						Method::GET => "get",
						Method::PUT | Method::PATCH => "update",
						Method::DELETE => "delete",
						Method::POST => "action",
						_ => "unknown",
					}
				}
			} else {
				"unknown"
			}
		}
	};

	Some(ApiInfo::rest(resource, action))
}

/// Simple singularization: removes trailing 's' if present.
/// This is a basic heuristic; for production, consider a proper inflector.
fn singularize(plural: &str) -> String {
	if plural.ends_with('s') && plural.len() > 1 {
		plural[..plural.len() - 1].to_string()
	} else {
		plural.to_string()
	}
}

/// Check if a string looks like a numeric ID or UUID.
fn is_numeric_or_uuid(s: &str) -> bool {
	// Check if it's all digits
	if s.chars().all(|c| c.is_ascii_digit()) {
		return true;
	}
	// Check if it looks like a UUID (contains dashes and hex chars)
	if s.len() == 36 && s.chars().filter(|&c| c == '-').count() == 4 {
		return s
			.chars()
			.all(|c| c.is_ascii_hexdigit() || c == '-');
	}
	false
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_singularize() {
		assert_eq!(singularize("agents"), "agent");
		assert_eq!(singularize("users"), "user");
		assert_eq!(singularize("convs"), "conv");
		assert_eq!(singularize("s"), "");
		assert_eq!(singularize("api"), "api");
	}

	#[test]
	fn test_is_numeric_or_uuid() {
		assert!(is_numeric_or_uuid("123"));
		assert!(is_numeric_or_uuid("550e8400-e29b-41d4-a716-446655440000"));
		assert!(!is_numeric_or_uuid("export"));
		assert!(!is_numeric_or_uuid("paged"));
	}

	#[test]
	fn test_parse_rest_api_info() {
		// List
		let uri: Uri = "/api/rest/agents".parse().unwrap();
		let info = parse_rest_api_info(&Method::GET, &uri).unwrap();
		assert!(matches!(info, ApiInfo::Rest { resource, action } if resource == "agent" && action == "list"));

		// Create
		let info = parse_rest_api_info(&Method::POST, &uri).unwrap();
		assert!(matches!(info, ApiInfo::Rest { resource, action } if resource == "agent" && action == "create"));

		// Get by ID
		let uri: Uri = "/api/rest/agents/123".parse().unwrap();
		let info = parse_rest_api_info(&Method::GET, &uri).unwrap();
		assert!(matches!(info, ApiInfo::Rest { resource, action } if resource == "agent" && action == "get"));

		// Update
		let info = parse_rest_api_info(&Method::PUT, &uri).unwrap();
		assert!(matches!(info, ApiInfo::Rest { resource, action } if resource == "agent" && action == "update"));

		// Delete
		let info = parse_rest_api_info(&Method::DELETE, &uri).unwrap();
		assert!(matches!(info, ApiInfo::Rest { resource, action } if resource == "agent" && action == "delete"));

		// Paged list
		let uri: Uri = "/api/rest/agents/paged".parse().unwrap();
		let info = parse_rest_api_info(&Method::GET, &uri).unwrap();
		assert!(matches!(info, ApiInfo::Rest { resource, action } if resource == "agent" && action == "list_paged"));

		// Custom action
		let uri: Uri = "/api/rest/agents/123/export".parse().unwrap();
		let info = parse_rest_api_info(&Method::GET, &uri).unwrap();
		assert!(matches!(info, ApiInfo::Rest { resource, action } if resource == "agent" && action == "export"));

		// Non-REST path
		let uri: Uri = "/api/rpc".parse().unwrap();
		assert!(parse_rest_api_info(&Method::POST, &uri).is_none());
	}
}
