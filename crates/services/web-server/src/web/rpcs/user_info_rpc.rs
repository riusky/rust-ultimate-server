use lib_core::generate_rpc_routes;
use lib_core::model::user_info::{UserInfo, UserInfoBmc, UserInfoFilter};
use lib_rpc_core::prelude::*;
use rpc_router::IntoParams;
use serde::{Deserialize, Serialize};

pub fn rpc_router_builder() -> RouterBuilder {
	generate_rpc_routes!(
		// -- Query endpoints
		get_user_info,
		get_user_info_by_user_id,
		get_current_user_info,
		list_user_infos,
	)
}

// region:    --- RPC Types

#[derive(Debug, Deserialize)]
pub struct GetUserInfoParams {
	pub id: i64,
}
impl IntoParams for GetUserInfoParams {}

#[derive(Debug, Deserialize)]
pub struct GetUserInfoByUserIdParams {
	pub user_id: i64,
}
impl IntoParams for GetUserInfoByUserIdParams {}

#[derive(Debug, Deserialize)]
pub struct ListUserInfosParams {
	pub filters: Option<Vec<UserInfoFilter>>,
	pub list_options: Option<ListOptions>,
}
impl IntoParams for ListUserInfosParams {}

#[derive(Debug, Serialize)]
pub struct ListUserInfosResult {
	pub items: Vec<UserInfo>,
	pub total: i64,
}

// endregion: --- RPC Types

// region:    --- Permission Registration

lib_core::register_crud_permissions!(
	"user_info",
	"User Info",
	"User Management",
	"user profile information"
);

// endregion: --- Permission Registration

// region:    --- RPC Handlers

/// Get user info by ID
#[lib_macros::permission(
	key = "user_info:read",
	group = "User Management",
	display = "Get User Info",
	description = "View user profile information by ID"
)]
pub async fn get_user_info(
	ctx: Ctx,
	mm: ModelManager,
	params: GetUserInfoParams,
) -> Result<DataRpcResult<UserInfo>> {
	let user_info = UserInfoBmc::get(&ctx, &mm, params.id).await?;
	Ok(user_info.into())
}

/// Get user info by user_id (more commonly used)
#[lib_macros::permission(
	key = "user_info:read",
	group = "User Management",
	display = "Get User Info By User ID",
	description = "View user profile information by user ID"
)]
pub async fn get_user_info_by_user_id(
	ctx: Ctx,
	mm: ModelManager,
	params: GetUserInfoByUserIdParams,
) -> Result<DataRpcResult<UserInfo>> {
	let filter = UserInfoFilter {
		user_id: Some(params.user_id.into()),
		..Default::default()
	};

	let user_info = UserInfoBmc::first(&ctx, &mm, Some(vec![filter]), None)
		.await?
		.ok_or(lib_core::model::Error::EntityNotFound {
			entity: "user_info",
			id: params.user_id,
		})?;

	Ok(user_info.into())
}

/// Get current logged-in user's info
#[lib_macros::permission(
	key = "user_info:read",
	group = "User Management",
	display = "Get Current User Info",
	description = "View current logged-in user's profile information"
)]
pub async fn get_current_user_info(
	ctx: Ctx,
	mm: ModelManager,
) -> Result<DataRpcResult<UserInfo>> {
	let user_id = ctx.user_id();

	let filter = UserInfoFilter {
		user_id: Some(user_id.into()),
		..Default::default()
	};

	let user_info = UserInfoBmc::first(&ctx, &mm, Some(vec![filter]), None)
		.await?
		.ok_or(lib_core::model::Error::EntityNotFound {
			entity: "user_info",
			id: user_id,
		})?;

	Ok(user_info.into())
}

/// List user infos with optional filters and pagination
#[lib_macros::permission(
	key = "user_info:list",
	group = "User Management",
	display = "List User Infos",
	description = "View list of user profile information"
)]
pub async fn list_user_infos(
	ctx: Ctx,
	mm: ModelManager,
	params: ListUserInfosParams,
) -> Result<DataRpcResult<ListUserInfosResult>> {
	let items =
		UserInfoBmc::list(&ctx, &mm, params.filters.clone(), params.list_options).await?;
	let total = UserInfoBmc::count(&ctx, &mm, params.filters).await?;

	Ok(ListUserInfosResult { items, total }.into())
}

// endregion: --- RPC Handlers
