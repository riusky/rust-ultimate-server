use crate::error::{Error, Result};
use crate::handlers::api_handlers::ApiInfo;
use crate::log::{log_request, ClientErrorInfo};
use crate::middleware::mw_auth::CtxW;
use crate::middleware::mw_req_stamp::ReqStamp;

use axum::http::{Method, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::{json, to_value};
use std::sync::Arc;
use tracing::debug;
use uuid::Uuid;

pub async fn mw_reponse_map(
	ctx: Result<CtxW>, // Axum 0.8 does not seem to support Option anymore
	uri: Uri,
	req_method: Method,
	req_stamp: ReqStamp,
	res: Response,
) -> Response {
	let ctx = ctx.map(|ctx| ctx.0).ok();

	debug!("{:<12} - mw_reponse_map", "RES_MAPPER");
	let uuid = Uuid::new_v4();

	// -- Get API info (supports both RPC and REST)
	let api_info = res.extensions().get::<Arc<ApiInfo>>().map(Arc::as_ref);

	// -- Get error from lib-web::Error (REST errors are already wrapped by mw_rest_error)
	let (client_status_error, web_error) = get_error_info(&res);

	// -- If client error, build the new response based on API type
	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error = to_value(client_error).ok();
				let message = client_error.as_ref().and_then(|v| v.get("message"));
				let biz_code = client_error.as_ref().and_then(|v| v.get("biz_code"));
				let detail = client_error.as_ref().and_then(|v| v.get("detail"));

				// Build different error body based on API type
				let client_error_body = match api_info {
					// JSON-RPC format error response
					Some(ApiInfo::Rpc { id, .. }) => {
						json!({
							"jsonrpc": "2.0",
							"id": id,
							"error": {
								"code": status_code.as_u16(),
								"biz_code": biz_code,
								"message": message,
								"data": {
									"req_uuid": uuid.to_string(),
									"detail": detail
								},
							}
						})
					}
					// REST format error response
					Some(ApiInfo::Rest { resource, action }) => {
						json!({
							"success": false,
							"error": {
								"code": status_code.as_u16(),
								"biz_code": biz_code,
								"message": message,
								"detail": detail,
								"resource": resource,
								"action": action,
							},
							"meta": {
								"req_uuid": uuid.to_string(),
							}
						})
					}
					// Default/fallback format (generic REST-like)
					None => {
						json!({
							"success": false,
							"error": {
								"code": status_code.as_u16(),
								"biz_code": biz_code,
								"message": message,
								"detail": detail,
							},
							"meta": {
								"req_uuid": uuid.to_string(),
							}
						})
					}
				};

				debug!("CLIENT ERROR BODY:\n{client_error_body}");

				// Build the new response from the client_error_body
				(*status_code, Json(client_error_body)).into_response()
			});

	// -- Build and log the server log line.
	let client_error_info = client_status_error.map(|(_, ce)| ClientErrorInfo {
		error_type: ce.message.clone(),
		detail: ce.detail.clone(),
	});

	// TODO: Need to hander if log_request fail (but should not fail request)
	let _ = log_request(
		req_method,
		uri,
		req_stamp,
		api_info,
		ctx,
		web_error.as_ref().map(|e| e.as_ref()),
		client_error_info,
	)
	.await;

	debug!("\n");

	error_response.unwrap_or(res)
}

/// Wrapper for serializable client error
#[derive(Debug, Serialize)]
struct SerializableClientError {
	message: String,
	/// Business error code for frontend i18n lookup
	biz_code: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	detail: Option<serde_json::Value>,
}

/// Extract error information from response extensions.
/// Only handles lib_web::Error (REST errors are already wrapped by mw_rest_error).
fn get_error_info(
	res: &Response,
) -> (
	Option<(StatusCode, SerializableClientError)>,
	Option<Arc<Error>>,
) {
	if let Some(web_error) = res.extensions().get::<Arc<Error>>() {
		let (status, client_error) = web_error.client_status_and_error();
		let serializable = SerializableClientError {
			message: client_error.as_ref().to_string(),
			biz_code: client_error.biz_code().to_string(),
			detail: to_value(&client_error).ok().and_then(|v| v.get("detail").cloned()),
		};
		return (Some((status, serializable)), Some(Arc::clone(web_error)));
	}

	(None, None)
}
