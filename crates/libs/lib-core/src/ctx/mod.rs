// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

use crate::model::acs::UserPermissions;

// endregion: --- Modules

#[cfg_attr(feature = "with-rpc", derive(rpc_router::RpcResource))]
#[derive(Clone, Debug)]
pub struct Ctx {
	user_id: i64,

	/// Note: For the future ACS (Access Control System)
	conv_id: Option<i64>,

	/// User permissions (loaded by middleware)
	permissions: Option<UserPermissions>,
}

// Constructors.
impl Ctx {
	pub fn root_ctx() -> Self {
		Ctx {
			user_id: 0,
			conv_id: None,
			permissions: None,
		}
	}

	pub fn new(user_id: i64) -> Result<Self> {
		if user_id == 0 {
			Err(Error::CtxCannotNewRootCtx)
		} else {
			Ok(Self {
				user_id,
				conv_id: None,
				permissions: None,
			})
		}
	}

	/// Note: For the future ACS (Access Control System)
	pub fn add_conv_id(&self, conv_id: i64) -> Ctx {
		let mut ctx = self.clone();
		ctx.conv_id = Some(conv_id);
		ctx
	}

	/// Set user permissions (called by permission resolver middleware)
	pub fn with_permissions(&self, permissions: UserPermissions) -> Ctx {
		let mut ctx = self.clone();
		ctx.permissions = Some(permissions);
		ctx
	}
}

// Property Accessors.
impl Ctx {
	pub fn user_id(&self) -> i64 {
		self.user_id
	}

	//. /// Note: For the future ACS (Access Control System)
	pub fn conv_id(&self) -> Option<i64> {
		self.conv_id
	}

	/// Get user permissions
	pub fn permissions(&self) -> Option<&UserPermissions> {
		self.permissions.as_ref()
	}
}

// Permission checking methods.
impl Ctx {
	/// Check if user has a specific permission
	pub fn has_permission(&self, key: &str) -> bool {
		self.permissions
			.as_ref()
			.map(|p| p.has_permission(key))
			.unwrap_or(false)
	}

	/// Check if user has any of the specified permissions
	pub fn has_any_permission(&self, keys: &[&str]) -> bool {
		self.permissions
			.as_ref()
			.map(|p| p.has_any_permission(keys))
			.unwrap_or(false)
	}

	/// Check if user has all of the specified permissions
	pub fn has_all_permissions(&self, keys: &[&str]) -> bool {
		self.permissions
			.as_ref()
			.map(|p| p.has_all_permissions(keys))
			.unwrap_or(false)
	}

	/// Require a specific permission, return error if not granted
	pub fn require_permission(&self, key: &str) -> Result<()> {
		// Root user (user_id = 0) bypasses permission checks
		if self.user_id == 0 {
			return Ok(());
		}

		// Check if permissions are loaded
		let permissions = self
			.permissions
			.as_ref()
			.ok_or(Error::PermissionsNotLoaded)?;

		if permissions.has_permission(key) {
			Ok(())
		} else {
			Err(Error::PermissionDenied {
				user_id: self.user_id,
				permission: key.to_string(),
			})
		}
	}

	/// Require any of the specified permissions
	pub fn require_any_permission(&self, keys: &[&str]) -> Result<()> {
		// Root user bypasses permission checks
		if self.user_id == 0 {
			return Ok(());
		}

		let permissions = self
			.permissions
			.as_ref()
			.ok_or(Error::PermissionsNotLoaded)?;

		if permissions.has_any_permission(keys) {
			Ok(())
		} else {
			Err(Error::PermissionAnyDenied {
				user_id: self.user_id,
				permissions: keys.iter().map(|s| s.to_string()).collect(),
			})
		}
	}

	/// Require all of the specified permissions
	pub fn require_all_permissions(&self, keys: &[&str]) -> Result<()> {
		for key in keys {
			self.require_permission(key)?;
		}
		Ok(())
	}
}
