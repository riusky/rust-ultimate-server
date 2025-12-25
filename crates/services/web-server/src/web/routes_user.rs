//! User management routes: register, delete account, change password, reset password

use axum::routing::post;
use axum::Router;
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_user;

/// Routes that don't require authentication
pub fn routes_public(mm: ModelManager) -> Router {
	Router::new()
		.route("/api/register", post(handlers_user::api_register_handler))
		.with_state(mm)
}

/// Routes that require authentication (need mw_ctx_require middleware)
pub fn routes_auth(mm: ModelManager) -> Router {
	Router::new()
		.route(
			"/api/user/delete-account",
			post(handlers_user::api_delete_account_handler),
		)
		.route(
			"/api/user/change-pwd",
			post(handlers_user::api_change_pwd_handler),
		)
		.route(
			"/api/user/reset-pwd",
			post(handlers_user::api_reset_pwd_handler), // Admin only
		)
		.with_state(mm)
}
