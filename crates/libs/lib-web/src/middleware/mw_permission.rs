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
use lib_valkey_core::cache_keys::{ttl, CacheKey};
use lib_valkey_core::{commands, ValkeyPool};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Type alias for the context extraction result
type CtxExtResult = core::result::Result<CtxW, CtxExtError>;

// region:    --- Without Cache

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
						req.extensions_mut()
							.insert(Ok::<CtxW, CtxExtError>(new_ctx_w));
					}
					Err(e) => {
						debug!(
							"{:<12} - mw_permission_resolver - Failed to load permissions: {:?}",
							"MIDDLEWARE", e
						);
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
	let permission_keys = UserRoleBmc::list_permission_keys_for_user(&ctx, mm, user_id).await?;

	Ok(UserPermissions::new(user_id, roles, permission_keys))
}

// endregion: --- Without Cache

// region:    --- With Valkey Cache

/// Cached user permissions data structure for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedPermissions {
	user_id: i64,
	roles: Vec<String>,
	permission_keys: Vec<String>,
}

impl From<CachedPermissions> for UserPermissions {
	fn from(cached: CachedPermissions) -> Self {
		UserPermissions::new(cached.user_id, cached.roles, cached.permission_keys)
	}
}

impl From<&UserPermissions> for CachedPermissions {
	fn from(perms: &UserPermissions) -> Self {
		CachedPermissions {
			user_id: perms.user_id,
			roles: perms.roles.clone(),
			permission_keys: perms.permissions.iter().cloned().collect(),
		}
	}
}

/// Middleware to resolve user permissions with Valkey caching.
///
/// This is the cached version of `mw_permission_resolver`. It first tries to
/// load permissions from Valkey cache, and falls back to database on cache miss.
///
/// # Flow
/// 1. Get CtxW from request extensions (set by mw_ctx_resolver)
/// 2. Try to get permissions from Valkey cache
/// 3. On cache miss, load from database and store in cache
/// 4. Update Ctx with permissions
/// 5. Store updated CtxW back to request extensions
pub async fn mw_permission_resolver_with_cache(
	State((mm, valkey_pool)): State<(ModelManager, ValkeyPool)>,
	mut req: Request<Body>,
	next: Next,
) -> Response {
	debug!("{:<12} - mw_permission_resolver_with_cache", "MIDDLEWARE");

	// Try to get the existing CtxW from extensions
	if let Some(ctx_result) = req.extensions().get::<CtxExtResult>().cloned() {
		if let Ok(ctx_w) = ctx_result {
			let ctx = &ctx_w.0;
			let user_id = ctx.user_id();

			// Only load permissions for non-root users
			if user_id != 0 {
				// Try to load permissions (from cache or database)
				match load_user_permissions_cached(&mm, &valkey_pool, user_id).await {
					Ok(permissions) => {
						// Update Ctx with permissions
						let ctx_with_perms = ctx.with_permissions(permissions);
						let new_ctx_w = CtxW(ctx_with_perms);

						// Update the request extensions with new CtxW
						req.extensions_mut()
							.insert(Ok::<CtxW, CtxExtError>(new_ctx_w));
					}
					Err(e) => {
						debug!(
							"{:<12} - mw_permission_resolver_with_cache - Failed to load permissions: {:?}",
							"MIDDLEWARE", e
						);
						// Continue without permissions on error
					}
				}
			}
		}
	}
	next.run(req).await
}

/// Load user permissions with Valkey caching
async fn load_user_permissions_cached(
	mm: &ModelManager,
	valkey_pool: &ValkeyPool,
	user_id: i64,
) -> lib_core::model::Result<UserPermissions> {
	let cache_key = CacheKey::UserPermissions(user_id);

	// Try to get from cache first
	if let Ok(mut conn) = valkey_pool.get().await {
		if let Ok(Some(cached_json)) =
			commands::get::<_, _, String>(&mut *conn, cache_key.as_str()).await
		{
			if let Ok(cached) = serde_json::from_str::<CachedPermissions>(&cached_json) {
				debug!(
					"{:<12} - Cache HIT for user {} permissions",
					"MIDDLEWARE", user_id
				);
				return Ok(cached.into());
			}
		}
	}

	debug!(
		"{:<12} - Cache MISS for user {} permissions, loading from DB",
		"MIDDLEWARE", user_id
	);

	// Cache miss - load from database
	let permissions = load_user_permissions(mm, user_id).await?;

	// Store in cache
	if let Ok(mut conn) = valkey_pool.get().await {
		let cached = CachedPermissions::from(&permissions);
		if let Ok(json) = serde_json::to_string(&cached) {
			let _ = commands::set(
				&mut *conn,
				cache_key.as_str(),
				json,
				Some(ttl::USER_PERMISSIONS),
			)
			.await;
		}
	}

	Ok(permissions)
}

/// Invalidate user permissions cache
///
/// Call this when user's roles or permissions change
pub async fn invalidate_user_permissions_cache(
	valkey_pool: &ValkeyPool,
	user_id: i64,
) -> lib_valkey_core::Result<()> {
	let cache_key = CacheKey::UserPermissions(user_id);
	let mut conn = valkey_pool.get().await?;
	commands::del_one(&mut *conn, cache_key.as_str()).await?;
	debug!(
		"{:<12} - Invalidated permissions cache for user {}",
		"CACHE", user_id
	);
	Ok(())
}

// endregion: --- With Valkey Cache
