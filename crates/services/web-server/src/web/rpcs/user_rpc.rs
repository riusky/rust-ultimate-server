// region:    --- Imports

use lib_core::generate_rpc_routes;
use lib_core::model::user::{UserBmc, UserFilter, UserForCreate, UserWithInfo};
use lib_rpc_core::prelude::*;
use rpc_router::{IntoDefaultRpcParams, IntoParams};
use serde::{Deserialize, Serialize};

// endregion: --- Imports

// region:    --- RPC Router

pub fn rpc_router_builder() -> RouterBuilder {
	generate_rpc_routes!(list_users, create_user, reset_user_pwd,)
}

// endregion: --- RPC Router

// region:    --- RPC Types

/// Page info for RPC responses
#[derive(Debug, Serialize)]
pub struct PageInfo {
	pub total: i64,
	pub page_size: i64,
	pub page_number: i64,
	pub total_pages: i64,
	pub has_more: bool,
}

impl PageInfo {
	pub fn new(total: i64, page_size: i64, page_number: i64) -> Self {
		let total_pages = if page_size > 0 {
			(total + page_size - 1) / page_size
		} else {
			0
		};
		Self {
			total,
			page_size,
			page_number,
			total_pages,
			has_more: page_number < total_pages,
		}
	}
}

/// Paged result for list operations
#[derive(Debug, Serialize)]
pub struct PagedResult<T> {
	pub data: Vec<T>,
	pub page_info: PageInfo,
}

// endregion: --- RPC Types

// region:    --- RPC Params

#[derive(Debug, Deserialize, Default)]
pub struct ListUsersParams {
	pub filters: Option<Vec<UserFilter>>,
	pub page_size: Option<i64>,
	pub page_number: Option<i64>,
}
impl IntoDefaultRpcParams for ListUsersParams {}

#[derive(Debug, Deserialize)]
pub struct CreateUserParams {
	pub username: String,
	pub password: String,
}
impl IntoParams for CreateUserParams {}

#[derive(Debug, Deserialize)]
pub struct ResetUserPwdParams {
	pub user_id: i64,
	pub new_password: String,
}
impl IntoParams for ResetUserPwdParams {}

// endregion: --- RPC Params

// region:    --- Permissions

lib_core::register_crud_permissions!("user", "User", "User Management", "user account management");

// endregion: --- Permissions

// region:    --- RPC Handlers

/// List users with their user_info (LEFT JOIN)
#[lib_macros::permission(
	key = "user:list",
	group = "User Management",
	display = "List Users",
	description = "List all users with their profile information"
)]
pub async fn list_users(
	ctx: Ctx,
	mm: ModelManager,
	params: ListUsersParams,
) -> Result<DataRpcResult<PagedResult<UserWithInfo>>> {
	let page_size = params.page_size.unwrap_or(20).min(1000).max(1);
	let page_number = params.page_number.unwrap_or(1).max(1);
	let offset = (page_number - 1) * page_size;

	let list_options = Some(ListOptions {
		limit: Some(page_size),
		offset: Some(offset),
		order_bys: Some("id.desc".into()),
	});

	// Get total count
	let total = UserBmc::count(&ctx, &mm, params.filters.clone()).await?;

	// Get paged data
	let users = UserBmc::list_with_info(&ctx, &mm, params.filters, list_options).await?;

	let result = PagedResult {
		data: users,
		page_info: PageInfo::new(total, page_size, page_number),
	};

	Ok(result.into())
}

/// Create a new user
#[lib_macros::permission(
	key = "user:create",
	group = "User Management",
	display = "Create User",
	description = "Create a new user account"
)]
pub async fn create_user(
	ctx: Ctx,
	mm: ModelManager,
	params: CreateUserParams,
) -> Result<DataRpcResult<i64>> {
	let CreateUserParams { username, password } = params;

	let user_for_create = UserForCreate {
		username,
		pwd_clear: password,
	};

	let user_id = UserBmc::create(&ctx, &mm, user_for_create).await?;

	Ok(user_id.into())
}

/// Reset user password
#[lib_macros::permission(
	key = "user:update",
	group = "User Management",
	display = "Reset Password",
	description = "Reset a user's password"
)]
pub async fn reset_user_pwd(
	ctx: Ctx,
	mm: ModelManager,
	params: ResetUserPwdParams,
) -> Result<DataRpcResult<()>> {
	let ResetUserPwdParams {
		user_id,
		new_password,
	} = params;

	UserBmc::update_pwd(&ctx, &mm, user_id, &new_password).await?;

	Ok(().into())
}

// endregion: --- RPC Handlers
