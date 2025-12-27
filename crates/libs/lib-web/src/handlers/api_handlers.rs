//! Unified API handlers and types for RPC and REST.
//!
//! This module provides:
//! - `ApiInfo` - Unified request info for both RPC and REST
//! - `rpc_axum_handler` - Axum handler for JSON-RPC requests

use crate::middleware::mw_auth::CtxW;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;
use rpc_router::resources_builder;
use serde_json::{json, Value};
use std::sync::Arc;

// region:    --- ApiInfo

/// Unified API request info for both RPC and REST
/// Injected into Axum Response extensions for logging and error serialization
#[derive(Debug, Clone)]
pub enum ApiInfo {
	/// JSON-RPC request info
	Rpc {
		id: Option<Value>,
		method: String,
	},
	/// REST API request info
	Rest {
		/// Resource name (e.g., "agent", "conv")
		resource: String,
		/// Action (e.g., "list", "get", "create", "update", "delete")
		action: String,
	},
}

impl ApiInfo {
	/// Create RPC info
	pub fn rpc(id: Option<Value>, method: impl Into<String>) -> Self {
		ApiInfo::Rpc {
			id,
			method: method.into(),
		}
	}

	/// Create REST info
	pub fn rest(resource: impl Into<String>, action: impl Into<String>) -> Self {
		ApiInfo::Rest {
			resource: resource.into(),
			action: action.into(),
		}
	}

	/// Get RPC id if this is an RPC request
	pub fn rpc_id(&self) -> Option<&Value> {
		match self {
			ApiInfo::Rpc { id, .. } => id.as_ref(),
			_ => None,
		}
	}

	/// Get method/action name for logging
	pub fn method_name(&self) -> String {
		match self {
			ApiInfo::Rpc { method, .. } => method.clone(),
			ApiInfo::Rest { resource, action } => format!("{}.{}", resource, action),
		}
	}

	/// Check if this is an RPC request
	pub fn is_rpc(&self) -> bool {
		matches!(self, ApiInfo::Rpc { .. })
	}

	/// Check if this is a REST request
	pub fn is_rest(&self) -> bool {
		matches!(self, ApiInfo::Rest { .. })
	}
}

// endregion: --- ApiInfo

/// JSON-RPC request handler for Axum.
///
/// This handler:
/// 1. Parses the JSON-RPC request
/// 2. Creates ApiInfo for logging
/// 3. Executes the RPC route with user context
/// 4. Returns JSON-RPC 2.0 formatted response
pub async fn rpc_axum_handler(
	State(rpc_router): State<rpc_router::Router>,
	ctx: CtxW,
	Json(rpc_req): Json<Value>,
) -> Response {
	let ctx = ctx.0;

	// -- Parse and RpcRequest validate the rpc_request
	let rpc_req = match rpc_router::RpcRequest::try_from(rpc_req) {
		Ok(rpc_req) => rpc_req,
		Err(rpc_req_error) => {
			let res = crate::Error::RpcRequestParsing(rpc_req_error).into_response();
			return res;
		}
	};

	// -- Create the API Info (RPC type)
	//    (will be set to the response.extensions)
	let api_info = ApiInfo::rpc(
		Some(rpc_req.id.to_value()),
		rpc_req.method.clone(),
	);

	// -- Add the request specific resources
	// Note: Since Ctx is per axum request, we construct additional RPC resources.
	//       These additional resources will be "overlayed" on top of the base router services,
	//       meaning they will take precedence over the base router ones, but won't replace them.
	let additional_resources = resources_builder![ctx].build();

	// -- Exec Rpc Route
	let rpc_call_result = rpc_router
		.call_with_resources(rpc_req, additional_resources)
		.await;

	// -- Build Json Rpc Success Response
	// Note: Error Json response will be generated in the mw_res_map as wil other error.
	let res = rpc_call_result.map(|rpc_call_response| {
		let body_response = json!({
			"jsonrpc": "2.0",
			"id": rpc_call_response.id,
			"result": rpc_call_response.value
		});
		Json(body_response)
	});

	// -- Create and Update Axum Response
	// Note: We store data in the Axum Response extensions so that
	//       we can unpack it in the `mw_res_map` for client-side rendering.
	//       This approach centralizes error handling for the client at the `mw_res_map` module
	let res: crate::error::Result<_> = res.map_err(crate::error::Error::from);
	let mut res = res.into_response();
	// Note: Here, add the ApiInfo into the Axum response to be used
	//       later in the `mw_res_map` for RequestLineLogging, and eventual JSON-RPC error serialization.
	res.extensions_mut().insert(Arc::new(api_info));

	res
}
