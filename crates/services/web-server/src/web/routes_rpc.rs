use crate::web::rpcs::all_rpc_router_builder;
use axum::routing::post;
use axum::Router;
use lib_core::model::ModelManager;
use lib_valkey_core::ValkeyPool;
use lib_web::handlers::api_handlers;

/// Wrapper for ValkeyPool that implements rpc_router::FromResources
#[derive(Clone)]
pub struct PermissionCachePool(pub Option<ValkeyPool>);

impl rpc_router::FromResources for PermissionCachePool {}

///  Build the Axum router for '/api/rpc'
/// Note: This will build the `rpc-router::Router` that will be used by the
///       rpc_axum_handler
pub fn routes(mm: ModelManager, valkey_pool: Option<ValkeyPool>) -> Router {
	// Build the combined Rpc Router (from `rpc-router` crate)
	let rpc_router = all_rpc_router_builder()
		// Add the common resources for all rpc calls
		.append_resource(mm)
		.append_resource(PermissionCachePool(valkey_pool))
		.build();

	// Build the Axum Router for '/rpc'
	Router::new()
		.route("/rpc", post(api_handlers::rpc_axum_handler))
		.with_state(rpc_router)
}
