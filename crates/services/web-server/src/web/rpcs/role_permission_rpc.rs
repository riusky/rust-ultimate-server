//! Role-Permission RPC handlers for managing role permissions

use crate::web::routes_rpc::PermissionCachePool;
use lib_core::generate_rpc_routes;
use lib_core::model::acs::{Permission, PermissionBmc, RolePermissionBmc, UserRoleBmc};
use lib_rpc_core::prelude::*;
use lib_web::utils::permission_cache::invalidate_users_permissions_cache;
use rpc_router::IntoParams;
use serde::Deserialize;

pub fn rpc_router_builder() -> RouterBuilder {
	generate_rpc_routes!(
		// Role permission management
		list_permissions_for_role,
		set_permissions_for_role,
		list_all_permissions,
	)
}

// region:    --- RPC Types

#[derive(Debug, Deserialize)]
pub struct ParamsListPermissionsForRole {
	pub role_id: i64,
}
impl IntoParams for ParamsListPermissionsForRole {}

#[derive(Debug, Deserialize)]
pub struct ParamsSetPermissionsForRole {
	pub role_id: i64,
	pub permission_ids: Vec<i64>,
}
impl IntoParams for ParamsSetPermissionsForRole {}

// endregion: --- RPC Types

// region:    --- RPC Handlers

/// List all permissions assigned to a role
/// Returns a list of Permission entities
#[lib_macros::permission(
	key = "role_permission:list_for_role",
	group = "Role Permission Management",
	display = "List Role Permissions",
	description = "Get all permissions assigned to a specific role"
)]
pub async fn list_permissions_for_role(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsListPermissionsForRole,
) -> Result<DataRpcResult<Vec<Permission>>> {
	// Get permission IDs for role
	let permission_ids =
		RolePermissionBmc::list_permission_ids_for_role(&ctx, &mm, params.role_id).await?;

	// Get full permission info for each permission ID
	let mut permissions = Vec::new();
	for permission_id in permission_ids {
		let permission = PermissionBmc::get(&ctx, &mm, permission_id).await?;
		permissions.push(permission);
	}

	Ok(permissions.into())
}

/// Set permissions for a role (replaces all existing permissions)
#[lib_macros::permission(
	key = "role_permission:set_for_role",
	group = "Role Permission Management",
	display = "Set Role Permissions",
	description = "Set all permissions for a role, replacing any existing permission assignments"
)]
pub async fn set_permissions_for_role(
	ctx: Ctx,
	mm: ModelManager,
	cache_pool: PermissionCachePool,
	params: ParamsSetPermissionsForRole,
) -> Result<DataRpcResult<()>> {
	RolePermissionBmc::set_permissions_for_role(
		&ctx,
		&mm,
		params.role_id,
		params.permission_ids,
	)
	.await?;

	// Invalidate permissions cache for all users who have this role
	if let Some(pool) = &cache_pool.0 {
		let user_ids = UserRoleBmc::list_user_ids_for_role(&ctx, &mm, params.role_id).await?;
		let _ = invalidate_users_permissions_cache(pool, &user_ids).await;
	}

	Ok(().into())
}

/// List all available permissions
#[lib_macros::permission(
	key = "permission:list",
	group = "Permission Management",
	display = "List All Permissions",
	description = "Get all available permissions in the system"
)]
pub async fn list_all_permissions(
	ctx: Ctx,
	mm: ModelManager,
) -> Result<DataRpcResult<Vec<Permission>>> {
	let permissions = PermissionBmc::list(&ctx, &mm, None, None).await?;
	Ok(permissions.into())
}

// endregion: --- RPC Handlers
