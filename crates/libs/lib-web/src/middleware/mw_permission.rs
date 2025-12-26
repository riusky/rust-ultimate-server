//! Permission resolver middleware
//!
//! This middleware runs after mw_ctx_resolver and loads user permissions
//! from the database (with optional Valkey caching).

use crate::middleware::mw_auth::{CtxExtError, CtxW};
use axum::body::Body;
use axum::extract::State;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lib_core::ctx::Ctx;
use lib_core::model::acs::{UserPermissions, UserRoleBmc};
use lib_core::model::ModelManager;
use tracing::debug;

/// Type alias for the context extraction result
type CtxExtResult = core::result::Result<CtxW, CtxExtError>;

/// Middleware to resolve user permissions and attach them to the context.
///
/// This middleware should be applied after `mw_ctx_resolver` in the middleware stack.
/// It loads the user's roles and permissions from the database and updates the Ctx.
///
/// # Flow
/// 1. Get CtxW from request extensions (set by mw_ctx_resolver)
/// 2. Load user permissions from database
/// 3. Update Ctx with permissions
/// 4. Store updated CtxW back to request extensions
pub async fn mw_permission_resolver(
	State(mm): State<ModelManager>,
	mut req: Request<Body>,
	next: Next,
) -> Response {
	debug!("{:<12} - mw_permission_resolver", "MIDDLEWARE");

	// Try to get the existing CtxW from extensions
	if let Some(ctx_result) = req.extensions().get::<CtxExtResult>().cloned() {
		if let Ok(ctx_w) = ctx_result {
			let ctx = &ctx_w.0;
			let user_id = ctx.user_id();

			// Only load permissions for non-root users
			if user_id != 0 {
				// Load permissions from database
				match load_user_permissions(&mm, user_id).await {
					Ok(permissions) => {
						// Update Ctx with permissions
						let ctx_with_perms = ctx.with_permissions(permissions);
						let new_ctx_w = CtxW(ctx_with_perms);

						// Update the request extensions with new CtxW
						req.extensions_mut().insert(Ok::<CtxW, CtxExtError>(new_ctx_w));
					}
					Err(e) => {
						debug!("{:<12} - mw_permission_resolver - Failed to load permissions: {:?}", "MIDDLEWARE", e);
						// Continue without permissions on error
					}
				}
			}
		}
	}
	next.run(req).await
}

/// Load user permissions from database
async fn load_user_permissions(
	mm: &ModelManager,
	user_id: i64,
) -> lib_core::model::Result<UserPermissions> {
	let ctx = Ctx::root_ctx();

	// Get user's roles
	let roles = UserRoleBmc::list_role_names_for_user(&ctx, mm, user_id).await?;

	// Get user's permission keys
	let permission_keys =
		UserRoleBmc::list_permission_keys_for_user(&ctx, mm, user_id).await?;

	Ok(UserPermissions::new(user_id, roles, permission_keys))
}

// region:    --- With Valkey Cache (Optional)

/// Valkey cache key for user permissions
pub fn user_permissions_cache_key(user_id: i64) -> String {
	format!("user:{}:permissions", user_id)
}

/// Cache TTL for user permissions (in seconds)
pub const PERMISSIONS_CACHE_TTL: u64 = 300; // 5 minutes

// TODO: Implement caching with Valkey when integrated
// pub async fn mw_permission_resolver_with_cache(
//     State(mm): State<ModelManager>,
//     State(valkey): State<ValkeyManager>,
//     mut req: Request<Body>,
//     next: Next,
// ) -> Response {
//     // 1. Try to get from Valkey cache
//     // 2. On cache miss, load from database
//     // 3. Store in cache with TTL
//     // 4. Update Ctx
// }

// endregion: --- With Valkey Cache (Optional)
