// region:    --- Modules

mod config;
mod error;
mod web;

pub use self::error::{Error, Result};
use config::web_config;

use lib_web::middleware::mw_auth::{mw_ctx_require, mw_ctx_resolver};
use lib_web::middleware::mw_permission::{
	mw_permission_resolver, mw_permission_resolver_with_cache,
};
use lib_web::middleware::mw_req_stamp::mw_req_stamp_resolver;
use lib_web::middleware::mw_res_map::mw_reponse_map;
use lib_web::middleware::mw_rest_info::mw_rest_info;
use lib_web::routes::routes_static;

use crate::web::routes_login;
use crate::web::routes_rest;
use crate::web::routes_user;

use axum::{middleware, Router};
use lib_core::ctx::Ctx;
use lib_core::model::acs::PermissionBmc;
use lib_core::model::user::{UserBmc, UserForLogin};
use lib_core::model::ModelManager;
use lib_valkey_core::{new_valkey_pool, ValkeyPool};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[cfg(debug_assertions)]
use lib_core::_dev_utils;

// endregion: --- Modules

/// Sets initial admin password from env when `INIT_ADMIN_PASSWORD` is set.
/// Idempotent: skips if the user already has a password (avoids overwriting).
async fn ensure_initial_admin_password(mm: &ModelManager) -> Result<()> {
	let pwd = match std::env::var("INIT_ADMIN_PASSWORD") {
		Ok(p) => p,
		Err(_) => return Ok(()),
	};
	let username = std::env::var("INIT_ADMIN_USERNAME").unwrap_or_else(|_| "admin".into());
	let ctx = Ctx::root_ctx();

	let user = match UserBmc::first_by_username::<UserForLogin>(&ctx, mm, &username).await {
		Ok(Some(u)) => u,
		Ok(None) => {
			tracing::warn!(
				"{:<12} - Initial admin user '{}' not found, skip",
				"INIT-ADMIN",
				username
			);
			return Ok(());
		}
		Err(e) => {
			tracing::error!("Failed to resolve initial admin user: {:?}", e);
			return Err(e.into());
		}
	};

	let already_has_pwd = user.pwd.as_ref().map_or(false, |s| !s.trim().is_empty());
	if already_has_pwd {
		info!(
			"{:<12} - Initial admin '{}' already has password, skip",
			"INIT-ADMIN", username
		);
		return Ok(());
	}

	UserBmc::update_pwd(&ctx, mm, user.id, &pwd)
		.await
		.inspect_err(|e| tracing::error!("Failed to set initial admin password: {:?}", e))?;
	info!(
		"{:<12} - Set initial admin password for '{}'",
		"INIT-ADMIN", username
	);
	Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		// .without_time() // For early local development.
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	// -- FOR DEV ONLY
	#[cfg(debug_assertions)]
	_dev_utils::init_dev().await;

	let config = web_config();
	let registered_model_caches = lib_core::model::cache::registered_model_caches();
	let model_cache_required = !registered_model_caches.is_empty();

	if model_cache_required {
		let resources = registered_model_caches
			.iter()
			.map(|cache| format!("{}:{}", cache.resource, cache.table))
			.collect::<Vec<_>>()
			.join(", ");
		info!("{:<12} - Model cache REQUIRED by: {}", "CONFIG", resources);
	} else {
		info!("{:<12} - Model cache DISABLED", "CONFIG");
	}

	// -- Initialize Valkey pool when any startup-declared cache requires it.
	let valkey_pool: Option<ValkeyPool> = if config.PERMISSION_CACHE_ENABLED || model_cache_required
	{
		new_valkey_pool()
			.await
			.inspect_err(|e| tracing::error!("Failed to create Valkey pool: {:?}", e))
			.map_err(|e| Error::ValkeyPool(e.to_string()))
			.map(Some)?
	} else {
		None
	};
	let permission_cache_pool = if config.PERMISSION_CACHE_ENABLED {
		info!("{:<12} - Permission cache ENABLED (Valkey)", "CONFIG");
		valkey_pool.clone()
	} else {
		info!("{:<12} - Permission cache DISABLED", "CONFIG");
		None
	};

	let mm = ModelManager::new_with_valkey_pool(valkey_pool.clone()).await?;

	ensure_initial_admin_password(&mm).await?;

	// -- Define Routes (this validates handler annotations via generate_rpc_routes!)
	// Note: generate_rpc_routes! in each RPC module validates handlers at runtime
	let routes_rpc =
		web::routes_rpc::routes(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));

	let routes_rest = routes_rest::routes(mm.clone())
		.layer(middleware::from_fn(mw_rest_info))
		.route_layer(middleware::from_fn(mw_ctx_require));

	// User management routes that require auth
	let routes_user_auth =
		routes_user::routes_auth(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));

	// -- Sync permissions from code to database
	let ctx = Ctx::root_ctx();
	PermissionBmc::sync_from_registry(&ctx, &mm)
		.await
		.inspect_err(|e| tracing::error!("Failed to sync permissions: {:?}", e))
		.map_err(|e| Error::PermissionSync(e.to_string()))?;

	// Note: Admin role bypasses all permission checks in Ctx::require_permission()
	// No need to assign individual permissions to admin role

	// -- Build routes with middleware
	// Middleware order (from outer to inner):
	// 1. mw_req_stamp_resolver - Add request timestamp
	// 2. CookieManager - Handle cookies
	// 3. mw_ctx_resolver - Resolve user context from token
	// 4. mw_permission_resolver - Load user permissions (with optional Valkey cache)
	// 5. mw_reponse_map - Map response
	let routes_all = Router::new()
		.merge(routes_login::routes(mm.clone()))
		.merge(routes_user::routes_public(mm.clone())) // register
		.merge(routes_user_auth) // delete-account, change-pwd
		.nest("/api", routes_rpc)
		.nest("/api", routes_rest)
		.layer(middleware::map_response(mw_reponse_map));

	// Add permission resolver middleware based on cache configuration
	let routes_all = if let Some(pool) = permission_cache_pool {
		routes_all.layer(middleware::from_fn_with_state(
			(mm.clone(), pool),
			mw_permission_resolver_with_cache,
		))
	} else {
		routes_all.layer(middleware::from_fn_with_state(
			mm.clone(),
			mw_permission_resolver,
		))
	};

	let routes_all = routes_all
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolver))
		.layer(CookieManagerLayer::new())
		.layer(middleware::from_fn(mw_req_stamp_resolver))
		.fallback_service(routes_static::serve_dir(&config.WEB_FOLDER));

	// region:    --- Start Server
	let listener = TcpListener::bind("0.0.0.0:8080")
		.await
		.map_err(|e| Error::Server(format!("bind: {e}")))?;
	info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
		.await
		.map_err(|e| Error::Server(format!("serve: {e}")))?;
	// endregion: --- Start Server

	Ok(())
}
