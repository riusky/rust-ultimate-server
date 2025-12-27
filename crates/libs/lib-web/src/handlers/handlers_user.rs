//! User management handlers: register, delete account, change password, reset password

use crate::error::{Error, Result};
use crate::middleware::mw_auth::CtxW;
use crate::utils::token;
use axum::extract::State;
use axum::Json;
use lib_auth::pwd::{self, ContentToHash};
use lib_core::ctx::Ctx;
use lib_core::model::user::{UserBmc, UserForCreate, UserForLogin};
use lib_core::model::ModelManager;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::debug;

// region:    --- Register

/// Register a new user (public - no auth required)
pub async fn api_register_handler(
	State(mm): State<ModelManager>,
	cookies: Cookies,
	Json(payload): Json<RegisterPayload>,
) -> Result<Json<Value>> {
	debug!("{:<12} - api_register_handler", "HANDLER");

	let RegisterPayload { username, pwd } = payload;

	// Validate input
	if username.trim().is_empty() {
		return Err(Error::RegisterFailUsernameEmpty);
	}
	if pwd.len() < 6 {
		return Err(Error::RegisterFailPwdTooShort);
	}

	let root_ctx = Ctx::root_ctx();

	// Create the user
	let user_id = UserBmc::create(
		&root_ctx,
		&mm,
		UserForCreate {
			username: username.clone(),
			pwd_clear: pwd,
		},
	)
	.await?;

	// Get user for token generation
	let user: UserForLogin = UserBmc::get(&root_ctx, &mm, user_id).await?;

	// Auto login after register
	token::set_token_cookie(&cookies, &user.username, user.token_salt)?;

	Ok(Json(json!({
		"result": {
			"success": true,
			"user_id": user_id
		}
	})))
}

#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
	username: String,
	pwd: String,
}

// endregion: --- Register

// region:    --- Delete Account

/// Delete user account (requires login, user can only delete own account)
pub async fn api_delete_account_handler(
	State(mm): State<ModelManager>,
	cookies: Cookies,
	CtxW(ctx): CtxW,
	Json(payload): Json<DeleteAccountPayload>,
) -> Result<Json<Value>> {
	debug!("{:<12} - api_delete_account_handler", "HANDLER");

	let user_id = ctx.user_id();

	// Verify password before deletion
	let user: UserForLogin = UserBmc::get(&ctx, &mm, user_id).await?;

	let Some(pwd) = user.pwd else {
		return Err(Error::UserHasNoPwd);
	};

	pwd::validate_pwd(
		ContentToHash {
			salt: user.pwd_salt,
			content: payload.pwd.clone(),
		},
		pwd,
	)
	.await
	.map_err(|_| Error::PwdNotMatching)?;

	// Delete the user
	UserBmc::delete(&ctx, &mm, user_id).await?;

	// Remove token cookie
	token::remove_token_cookie(&cookies)?;

	Ok(Json(json!({
		"result": {
			"success": true,
			"deleted_user_id": user_id
		}
	})))
}

#[derive(Debug, Deserialize)]
pub struct DeleteAccountPayload {
	pwd: String, // Require password confirmation
}

// endregion: --- Delete Account

// region:    --- Change Password

/// Change password (requires login, user can only change own password)
pub async fn api_change_pwd_handler(
	State(mm): State<ModelManager>,
	CtxW(ctx): CtxW,
	Json(payload): Json<ChangePwdPayload>,
) -> Result<Json<Value>> {
	debug!("{:<12} - api_change_pwd_handler", "HANDLER");

	let ChangePwdPayload { old_pwd, new_pwd } = payload;

	// Validate new password
	if new_pwd.len() < 6 {
		return Err(Error::RegisterFailPwdTooShort);
	}

	let user_id = ctx.user_id();

	// Get user and verify old password
	let user: UserForLogin = UserBmc::get(&ctx, &mm, user_id).await?;

	let Some(pwd) = user.pwd else {
		return Err(Error::UserHasNoPwd);
	};

	pwd::validate_pwd(
		ContentToHash {
			salt: user.pwd_salt,
			content: old_pwd,
		},
		pwd,
	)
	.await
	.map_err(|_| Error::PwdNotMatching)?;

	// Update to new password
	UserBmc::update_pwd(&ctx, &mm, user_id, &new_pwd).await?;

	Ok(Json(json!({
		"result": {
			"success": true
		}
	})))
}

#[derive(Debug, Deserialize)]
pub struct ChangePwdPayload {
	old_pwd: String,
	new_pwd: String,
}

// endregion: --- Change Password

// region:    --- Reset Password

/// Reset password (admin only - can reset any user's password without old password)
#[lib_macros::rest_permission(
	key = "user:reset-pwd",
	group = "User Management",
	display = "Reset User Password",
	desc = "Admin can reset any user's password"
)]
pub async fn api_reset_pwd_handler(
	State(mm): State<ModelManager>,
	ctx: CtxW,
	Json(payload): Json<ResetPwdPayload>,
) -> Result<Json<Value>> {
	debug!("{:<12} - api_reset_pwd_handler", "HANDLER");

	let ResetPwdPayload {
		target_user_id,
		new_pwd,
	} = payload;

	// Validate new password
	if new_pwd.len() < 6 {
		return Err(Error::RegisterFailPwdTooShort);
	}

	// Permission already checked by #[rest_permission] macro

	// Update password for target user
	UserBmc::update_pwd(&ctx.0, &mm, target_user_id, &new_pwd).await?;

	Ok(Json(json!({
		"result": {
			"success": true
		}
	})))
}

#[derive(Debug, Deserialize)]
pub struct ResetPwdPayload {
	target_user_id: i64,
	new_pwd: String,
}

// endregion: --- Reset Password
