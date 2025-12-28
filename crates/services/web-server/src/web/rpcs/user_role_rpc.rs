//! User-Role RPC handlers for assigning roles to users

use crate::web::routes_rpc::PermissionCachePool;
use lib_core::generate_rpc_routes;
use lib_core::model::acs::{Role, RoleBmc, UserRoleBmc};
use lib_rpc_core::prelude::*;
use lib_web::utils::permission_cache::invalidate_user_permissions_cache;
use rpc_router::IntoParams;
use serde::Deserialize;

pub fn rpc_router_builder() -> RouterBuilder {
	generate_rpc_routes!(
		// User role management
		list_roles_for_user,
		set_roles_for_user,
	)
}

// region:    --- RPC Types

#[derive(Debug, Deserialize)]
pub struct ParamsListRolesForUser {
	pub user_id: i64,
}
impl IntoParams for ParamsListRolesForUser {}

#[derive(Debug, Deserialize)]
pub struct ParamsSetRolesForUser {
	pub user_id: i64,
	pub role_ids: Vec<i64>,
}
impl IntoParams for ParamsSetRolesForUser {}

// endregion: --- RPC Types

// region:    --- RPC Handlers

/// List all roles assigned to a user
/// Returns a list of Role entities
#[lib_macros::permission(
	key = "user_role:list_for_user",
	group = "User Role Management",
	display = "List User Roles",
	description = "Get all roles assigned to a specific user"
)]
pub async fn list_roles_for_user(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsListRolesForUser,
) -> Result<DataRpcResult<Vec<Role>>> {
	// Get role IDs for user
	let role_ids = UserRoleBmc::list_role_ids_for_user(&ctx, &mm, params.user_id).await?;

	// Get full role info for each role ID
	let mut roles = Vec::new();
	for role_id in role_ids {
		let role = RoleBmc::get(&ctx, &mm, role_id).await?;
		roles.push(role);
	}

	Ok(roles.into())
}

/// Set roles for a user (replaces all existing roles)
#[lib_macros::permission(
	key = "user_role:set_for_user",
	group = "User Role Management",
	display = "Set User Roles",
	description = "Set all roles for a user, replacing any existing role assignments"
)]
pub async fn set_roles_for_user(
	ctx: Ctx,
	mm: ModelManager,
	cache_pool: PermissionCachePool,
	params: ParamsSetRolesForUser,
) -> Result<DataRpcResult<()>> {
	UserRoleBmc::set_roles_for_user(&ctx, &mm, params.user_id, params.role_ids).await?;

	// Invalidate user permissions cache
	if let Some(pool) = &cache_pool.0 {
		let _ = invalidate_user_permissions_cache(pool, params.user_id).await;
	}

	Ok(().into())
}

// endregion: --- RPC Handlers
